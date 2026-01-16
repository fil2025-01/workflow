"use strict";
const recordBtn = document.querySelector('#recordBtn');
const recordingsList = document.getElementById('recordingsList');
const dateFilter = document.getElementById('dateFilter');
const viewHistoryBtn = document.getElementById('viewHistoryBtn');
const backBtn = document.getElementById('backBtn');
const recordingSection = document.getElementById('recordingSection');
const historySection = document.getElementById('historySection');
let mediaRecorder = null;
let audioChunks = [];
const statsLabel = document.getElementById('statsLabel');
const tableTemplate = document.getElementById('table-template');
const rowTemplate = document.getElementById('row-template');
const emptyTemplate = document.getElementById('empty-template');
let pollingInterval = null;
function ensurePolling() {
    if (pollingInterval)
        return;
    console.log('Starting background polling for transcripts...');
    pollingInterval = setInterval(() => {
        // Only poll if we are actually looking at the history section
        if (historySection.style.display !== 'none') {
            loadRecordings();
        }
    }, 3000);
}
function stopPolling() {
    if (pollingInterval) {
        console.log('All transcriptions complete. Stopping poll.');
        clearInterval(pollingInterval);
        pollingInterval = null;
    }
}
async function loadRecordings() {
    try {
        const date = dateFilter.value;
        let url = `/recordings?_t=${new Date().getTime()}`;
        if (date) {
            url += `&date=${date}`;
        }
        const response = await fetch(url);
        const files = await response.json();
        recordingsList.innerHTML = '';
        // Group files by base name (timestamp)
        const groups = {};
        files.forEach(file => {
            const baseName = file.name.replace(/\.[^/.]+$/, "");
            if (!groups[baseName]) {
                groups[baseName] = {};
            }
            if (file.is_transcript) {
                groups[baseName].transcript = file.path;
            }
            else {
                groups[baseName].audio = file.path;
            }
        });
        const sortedKeys = Object.keys(groups).sort();
        const count = sortedKeys.filter(k => groups[k].audio).length;
        statsLabel.textContent = `Total Recordings: ${count}`;
        // Check if any audio is missing a transcript
        const hasPending = Object.values(groups).some(g => g.audio && !g.transcript);
        if (hasPending) {
            ensurePolling();
        }
        else {
            stopPolling();
        }
        if (count === 0) {
            const emptyNode = document.importNode(emptyTemplate.content, true);
            recordingsList.appendChild(emptyNode);
            return;
        }
        const tableNode = document.importNode(tableTemplate.content, true);
        const tbody = tableNode.querySelector('tbody');
        let index = 1;
        for (const key of sortedKeys) {
            const group = groups[key];
            if (group.audio) {
                const rowNode = document.importNode(rowTemplate.content, true);
                const tr = rowNode.querySelector('tr');
                // No.
                const colNo = tr.querySelector('.col-no');
                colNo.textContent = index.toString();
                // Title (Transcript or Name)
                const colTitle = tr.querySelector('.col-title');
                if (group.transcript) {
                    const preview = document.createElement('span');
                    preview.className = 'transcript-preview';
                    preview.textContent = 'Loading...';
                    colTitle.appendChild(preview);
                    fetch(group.transcript)
                        .then(res => res.text()) // Get text first
                        .then(textData => {
                        let data;
                        try {
                            data = JSON.parse(textData);
                        }
                        catch (e) {
                            const cleanText = textData.replace(/^```json\s*/, '').replace(/^```\s*/, '').replace(/\s*```$/, '');
                            try {
                                data = JSON.parse(cleanText);
                            }
                            catch (e2) {
                                data = textData;
                            }
                        }
                        let title = '';
                        let fullText = '';
                        if (typeof data === 'object' && data !== null && (data.title || data.transcript)) {
                            title = data.title || key;
                            fullText = data.transcript || '';
                        }
                        else {
                            const text = String(data);
                            const titleMatch = text.match(/^Title:\s*(.+)$/m);
                            const transcriptMatch = text.match(/^Transcript:\s*([\s\S]+)$/m);
                            if (titleMatch) {
                                title = titleMatch[1];
                                fullText = transcriptMatch ? transcriptMatch[1].trim() : text;
                            }
                            else {
                                title = text.length > 50 ? text.substring(0, 50) + '...' : text;
                                fullText = text;
                            }
                        }
                        preview.textContent = title;
                        preview.title = fullText;
                    })
                        .catch(() => {
                        preview.textContent = key;
                    });
                }
                else {
                    // If no transcript yet, show a processing state
                    const span = document.createElement('span');
                    span.className = 'text-gray-600 italic';
                    span.textContent = 'Transcribing...';
                    colTitle.appendChild(span);
                }
                // Audio
                const colAudio = tr.querySelector('.col-audio audio');
                colAudio.src = group.audio;
                // Action
                const deleteBtn = tr.querySelector('.delete-btn');
                deleteBtn.addEventListener('click', async () => {
                    if (confirm('Delete this recording?')) {
                        try {
                            const res = await fetch('/recordings', {
                                method: 'DELETE',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ path: group.audio || group.transcript })
                            });
                            if (res.ok)
                                loadRecordings();
                        }
                        catch (err) {
                            console.error(err);
                        }
                    }
                });
                tbody.appendChild(tr);
                index++;
            }
        }
        recordingsList.appendChild(tableNode);
    }
    catch (error) {
        console.error('Error loading recordings:', error);
        recordingsList.innerHTML = '<p>Error loading data.</p>';
    }
}
// Set default date to today
const today = new Date().toISOString().split('T')[0];
dateFilter.value = today;
// Navigation
viewHistoryBtn.addEventListener('click', () => {
    recordingSection.style.display = 'none';
    historySection.style.display = 'block';
    loadRecordings();
});
backBtn.addEventListener('click', () => {
    historySection.style.display = 'none';
    recordingSection.style.display = 'flex';
    stopPolling(); // Stop polling when leaving history view
    window.scrollTo(0, 0);
});
// Reload on date change
dateFilter.addEventListener('change', loadRecordings);
/**
 * Helper function to handle recording logic for both standard and continuation buttons.
 * @param button The button element that triggered the recording.
 * @param includeDate Whether to append the selected date filter to the upload request.
 */
async function handleRecording(button, includeDate = false) {
    if (!mediaRecorder || mediaRecorder.state === 'inactive') {
        try {
            console.log('Starting recording...');
            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
            mediaRecorder = new MediaRecorder(stream);
            mediaRecorder.ondataavailable = (event) => {
                audioChunks.push(event.data);
            };
            mediaRecorder.onstop = async () => {
                const audioBlob = new Blob(audioChunks, { type: 'audio/webm' });
                audioChunks = [];
                const formData = new FormData();
                formData.append('file', audioBlob, 'recording.webm');
                let uploadUrl = '/upload';
                if (includeDate && dateFilter.value) {
                    uploadUrl += `?date=${dateFilter.value}`;
                }
                try {
                    const response = await fetch(uploadUrl, {
                        method: 'POST',
                        body: formData
                    });
                    if (response.ok) {
                        console.log('Audio uploaded successfully');
                        loadRecordings();
                    }
                    else {
                        console.error('Failed to upload audio');
                    }
                }
                catch (error) {
                    console.error('Error uploading audio:', error);
                }
            };
            mediaRecorder.start();
            button.textContent = 'Stop Recording';
            button.classList.add('recording');
        }
        catch (error) {
            console.error('Error accessing microphone:', error);
        }
    }
    else {
        console.log('Stopping recording...');
        mediaRecorder.stop();
        // Reset standard button
        recordBtn.textContent = 'Record';
        recordBtn.classList.remove('recording');
        // Reset continuation button if it exists
        const contBtn = document.getElementById('recordBtnContinuation');
        if (contBtn) {
            contBtn.textContent = 'Continue Recording';
            contBtn.classList.remove('recording');
        }
        audioChunks = [];
    }
}
recordBtn.addEventListener('click', () => handleRecording(recordBtn, false));
const recordBtnContinuation = document.getElementById('recordBtnContinuation');
if (recordBtnContinuation) {
    recordBtnContinuation.addEventListener('click', () => handleRecording(recordBtnContinuation, true));
}
