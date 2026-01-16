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
async function loadRecordings() {
    try {
        const date = dateFilter.value;
        let url = '/recordings';
        if (date) {
            url += `?date=${date}`;
        }
        const response = await fetch(url);
        const files = await response.json();
        recordingsList.innerHTML = '';
        // Group files by base name (timestamp)
        const groups = {};
        let count = 0;
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
        count = sortedKeys.filter(k => groups[k].audio).length;
        statsLabel.textContent = `Total Recordings: ${count}`;
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
                            // Failed to parse directly, try cleaning markdown
                            const cleanText = textData.replace(/^```json\s*/, '').replace(/^```\s*/, '').replace(/\s*```$/, '');
                            try {
                                data = JSON.parse(cleanText);
                            }
                            catch (e2) {
                                // Treat as legacy raw text
                                data = textData;
                            }
                        }
                        let title = '';
                        let fullText = '';
                        if (typeof data === 'object' && data !== null && (data.title || data.transcript)) {
                            // Handle JSON format
                            title = data.title || key;
                            fullText = data.transcript || '';
                        }
                        else {
                            // Handle legacy text format
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
                    colTitle.textContent = key;
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
    window.scrollTo(0, 0);
});
// Reload on date change
dateFilter.addEventListener('change', loadRecordings);

// Helper function to handle recording logic
async function handleRecording(button, includeDate = false) {
    if (!mediaRecorder || mediaRecorder.state === 'inactive') {
        try {
            console.log('Starting recording...');
            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
            mediaRecorder = new MediaRecorder(stream);
            mediaRecorder.ondataavailable = (event) => {
                audioChunks.push(event.data);
                console.log('data available', event.data);
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
                        loadRecordings(); // Refresh list
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
            return;
        }
    }
    else {
        console.log('Stopping recording...');
        if (mediaRecorder) {
            mediaRecorder.stop();
        }
        // Reset both buttons just in case
        recordBtn.textContent = 'Record';
        recordBtn.classList.remove('recording');
        
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

