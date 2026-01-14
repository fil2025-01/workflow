const recordBtn = document.querySelector('#recordBtn') as HTMLButtonElement;
const recordingsList = document.getElementById('recordingsList') as HTMLDivElement;
const dateFilter = document.getElementById('dateFilter') as HTMLInputElement;
const viewHistoryBtn = document.getElementById('viewHistoryBtn') as HTMLButtonElement;
const backBtn = document.getElementById('backBtn') as HTMLButtonElement;
const recordingSection = document.getElementById('recordingSection') as HTMLDivElement;
const historySection = document.getElementById('historySection') as HTMLDivElement;

let mediaRecorder: MediaRecorder | null = null;
let audioChunks: Blob[] = [];

const statsLabel = document.getElementById('statsLabel') as HTMLSpanElement;

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
        
        // Group files by base name (timestamp)
        const groups: {[key: string]: {audio?: string, transcript?: string}} = {};
        let count = 0;

        files.forEach(file => {
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

        const sortedKeys = Object.keys(groups).sort();
        count = sortedKeys.filter(k => groups[k].audio).length;
        statsLabel.textContent = `Total Recordings: ${count}`;

        if (count === 0) {
            recordingsList.innerHTML = '<p style="font-size: 13px; color: #666;">No recordings found.</p>';
            return;
        }

        const table = document.createElement('table');
        table.className = 'data-table';
        
        const thead = document.createElement('thead');
        thead.innerHTML = `
            <tr>
                <th>No.</th>
                <th>Name</th>
                <th>Audio</th>
                <th>Transcript</th>
                <th>Action</th>
            </tr>
        `;
        table.appendChild(thead);

        const tbody = document.createElement('tbody');
        let index = 1;

        for (const key of sortedKeys) {
            const group = groups[key];
            if (group.audio) {
                const tr = document.createElement('tr');
                
                // No.
                const tdNo = document.createElement('td');
                tdNo.textContent = index.toString();
                tr.appendChild(tdNo);

                // Name
                const tdName = document.createElement('td');
                tdName.textContent = key;
                tr.appendChild(tdName);

                // Audio
                const tdAudio = document.createElement('td');
                const audio = document.createElement('audio');
                audio.controls = true;
                audio.src = group.audio;
                audio.style.height = '30px'; // Compact player
                tdAudio.appendChild(audio);
                tr.appendChild(tdAudio);

                // Transcript (Preview)
                const tdTranscript = document.createElement('td');
                if (group.transcript) {
                    const preview = document.createElement('span');
                    preview.className = 'transcript-preview';
                    preview.textContent = 'Loading...';
                    tdTranscript.appendChild(preview);
                    
                    // Fetch preview content asynchronously
                    fetch(group.transcript).then(res => res.text()).then(text => {
                        preview.textContent = text.length > 50 ? text.substring(0, 50) + '...' : text;
                        preview.title = text; // Tooltip full text
                    }).catch(() => {
                        preview.textContent = 'Error';
                    });
                } else {
                    tdTranscript.textContent = '-';
                }
                tr.appendChild(tdTranscript);

                // Action
                const tdAction = document.createElement('td');
                const deleteBtn = document.createElement('button');
                deleteBtn.textContent = 'Delete';
                deleteBtn.className = 'delete-btn';
                
                deleteBtn.addEventListener('click', async () => {
                    if (confirm('Delete this recording?')) {
                        try {
                            const res = await fetch('/recordings', {
                                method: 'DELETE',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ path: group.audio || group.transcript })
                            });
                            if (res.ok) loadRecordings();
                        } catch (err) { console.error(err); }
                    }
                });
                tdAction.appendChild(deleteBtn);
                tr.appendChild(tdAction);

                tbody.appendChild(tr);
                index++;
            }
        }
        table.appendChild(tbody);
        recordingsList.appendChild(table);

    } catch (error) {
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
