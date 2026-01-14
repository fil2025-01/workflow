const recordBtn = document.querySelector('#recordBtn') as HTMLButtonElement;
const recordingsList = document.getElementById('recordingsList') as HTMLDivElement;
const dateFilter = document.getElementById('dateFilter') as HTMLInputElement;
const viewHistoryBtn = document.getElementById('viewHistoryBtn') as HTMLButtonElement;
const backBtn = document.getElementById('backBtn') as HTMLButtonElement;
const recordingSection = document.getElementById('recordingSection') as HTMLDivElement;
const historySection = document.getElementById('historySection') as HTMLDivElement;

let mediaRecorder: MediaRecorder | null = null;
let audioChunks: Blob[] = [];

async function loadRecordings() {
    try {
        const date = dateFilter.value;
        let url = '/recordings';
        if (date) {
            url += `?date=${date}`;
        }
        
        const response = await fetch(url);
        const files: {path: string, name: string, is_transcript: boolean}[] = await response.json();
        
        recordingsList.innerHTML = '';
        
        if (files.length === 0) {
            recordingsList.innerHTML = '<p>No recordings found for this date.</p>';
            return;
        }
        
        // Group files by base name (timestamp)
        const groups: {[key: string]: {audio?: string, transcript?: string}} = {};
        
        files.forEach(file => {
            // Remove extension to get base name (e.g., recording_123456789)
            const baseName = file.name.replace(/\.[^/.]+$/, "");
            if (!groups[baseName]) {
                groups[baseName] = {};
            }
            if (file.is_transcript) {
                groups[baseName].transcript = file.path;
            } else {
                groups[baseName].audio = file.path;
            }
        });

        // Sort by timestamp ascending (oldest to newest)
        const sortedKeys = Object.keys(groups).sort();

        for (const key of sortedKeys) {
            const group = groups[key];
            if (group.audio) {
                const div = document.createElement('div');
                div.className = 'recording-item';
                div.style.marginBottom = '20px';
                div.style.padding = '10px';
                div.style.border = '1px solid #ddd';
                div.style.borderRadius = '5px';

                const title = document.createElement('div');
                title.textContent = key;
                div.appendChild(title);

                const audio = document.createElement('audio');
                audio.controls = true;
                audio.src = group.audio;
                div.appendChild(audio);

                if (group.transcript) {
                    const transcriptDiv = document.createElement('div');
                    transcriptDiv.style.marginTop = '10px';
                    transcriptDiv.style.padding = '10px';
                    transcriptDiv.style.backgroundColor = '#f9f9f9';
                    
                    try {
                        const transcriptResponse = await fetch(group.transcript);
                        const text = await transcriptResponse.text();
                        transcriptDiv.textContent = text;
                    } catch (e) {
                        transcriptDiv.textContent = 'Error loading transcript.';
                    }
                    div.appendChild(transcriptDiv);
                }

                // Delete Button
                const deleteBtn = document.createElement('button');
                deleteBtn.textContent = 'Delete';
                deleteBtn.style.backgroundColor = '#ef4444'; // Red color
                deleteBtn.style.color = 'white';
                deleteBtn.style.border = 'none';
                deleteBtn.style.padding = '5px 10px';
                deleteBtn.style.borderRadius = '4px';
                deleteBtn.style.cursor = 'pointer';
                deleteBtn.style.marginTop = '10px';
                
                deleteBtn.addEventListener('click', async () => {
                    if (confirm('Are you sure you want to delete this recording?')) {
                        try {
                            const res = await fetch('/recordings', {
                                method: 'DELETE',
                                headers: {
                                    'Content-Type': 'application/json'
                                },
                                body: JSON.stringify({ path: group.audio || group.transcript })
                            });
                            if (res.ok) {
                                loadRecordings();
                            } else {
                                alert('Failed to delete recording.');
                            }
                        } catch (err) {
                            console.error('Error deleting:', err);
                            alert('Error deleting recording.');
                        }
                    }
                });
                div.appendChild(deleteBtn);

                recordingsList.appendChild(div);
            }
        }

    } catch (error) {
        console.error('Error loading recordings:', error);
        recordingsList.innerHTML = '<p>Error loading recordings.</p>';
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

recordBtn.addEventListener('click', async () => {
    if (!mediaRecorder || mediaRecorder.state === 'inactive') {
        try {
            console.log('mag record na');
            const stream = await navigator.mediaDevices.getUserMedia({audio: true});
            mediaRecorder = new MediaRecorder(stream);
            mediaRecorder.ondataavailable = (event: BlobEvent) => {
                audioChunks.push(event.data)
                console.log('data available', event.data)
            }
            mediaRecorder.onstop = async () => {
                const audioBlob = new Blob(audioChunks, { type: 'audio/webm'});
                audioChunks = [];

                const formData = new FormData();
                formData.append('file', audioBlob, 'recording.webm');

                try {
                    const response = await fetch('/upload', {
                        method: 'POST',
                        body: formData
                    })
                    if (response.ok) {
                        console.log('Audio uploaded successfully');
                        loadRecordings(); // Refresh list
                    } else {
                        console.error('Failed to upload audio')
                    }
                } catch (error) {
                    console.error('Error uploading audio:', error);
                }
            }
            mediaRecorder.start();
            recordBtn.textContent = 'Stop Recording';
            recordBtn.classList.add('recording');
        } catch (error) {
            console.error('Error accessing microphone:', error);
            return;
        }
    } else {
        console.log('wala pay na record');
        if (mediaRecorder) {
            mediaRecorder.stop();
        }
        recordBtn.textContent = 'Record Audio';
        recordBtn.classList.remove('recording');
        audioChunks = [];
    }
});
