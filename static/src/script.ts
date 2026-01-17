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
const tableTemplate = document.getElementById('table-template') as HTMLTemplateElement;
const rowTemplate = document.getElementById('row-template') as HTMLTemplateElement;
const emptyTemplate = document.getElementById('empty-template') as HTMLTemplateElement;

interface Recording {
  path: string;
  name: string;
  status: string;
  transcription: any;
}

async function loadRecordings() {
  try {
    const date = dateFilter.value;
    let url = `/recordings?_t=${new Date().getTime()}`;
    if (date) {
      url += `&date=${date}`;
    }

    const response = await fetch(url);
    const recordings: Recording[] = await response.json();

    recordingsList.innerHTML = '';
    statsLabel.textContent = `Total Recordings: ${recordings.length}`;

    // Check if any recording is still transcribing
    const hasPending = recordings.some(r => r.status === 'PENDING');
    if (hasPending) {
      ensurePolling();
    } else {
      stopPolling();
    }

    if (recordings.length === 0) {
      const emptyNode = document.importNode(emptyTemplate.content, true);
      recordingsList.appendChild(emptyNode);
      return;
    }

    const tableNode = document.importNode(tableTemplate.content, true);
    const tbody = tableNode.querySelector('tbody') as HTMLTableSectionElement;

    recordings.forEach((rec, index) => {
      const rowNode = document.importNode(rowTemplate.content, true);
      const tr = rowNode.querySelector('tr') as HTMLTableRowElement;

      // No.
      (tr.querySelector('.col-no') as HTMLTableCellElement).textContent = (index + 1).toString();

      // Title/Transcription
      const colTitle = tr.querySelector('.col-title') as HTMLTableCellElement;
      if (rec.transcription) {
        const preview = document.createElement('span');
        preview.className = 'transcript-preview cursor-pointer';
        
        let title = '';
        let fullText = '';

        if (typeof rec.transcription === 'object' && rec.transcription !== null) {
          title = rec.transcription.title || rec.name;
          fullText = rec.transcription.transcript || '';
        } else {
          title = rec.name;
        }

        preview.textContent = title;
        preview.title = fullText;
        colTitle.appendChild(preview);
      } else {
        colTitle.textContent = rec.name;
      }

      // Status
      const colStatus = tr.querySelector('.col-status') as HTMLTableCellElement;
      colStatus.textContent = rec.status;
      if (rec.status === 'PENDING') {
        colStatus.classList.add('text-gray-600', 'italic');
        colStatus.textContent = 'Transcribing...';
      } else if (rec.status === 'FAILED') {
        colStatus.classList.add('text-red-600');
      }

      // Audio
      const colAudio = tr.querySelector('.col-audio audio') as HTMLAudioElement;
      colAudio.src = rec.path;

      // Action
      const deleteBtn = tr.querySelector('.delete-btn') as HTMLButtonElement;
      deleteBtn.addEventListener('click', async () => {
        if (confirm('Delete this recording?')) {
          try {
            const res = await fetch('/recordings', {
              method: 'DELETE',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ path: rec.path })
            });
            if (res.ok) loadRecordings();
          } catch (err) { console.error(err); }
        }
      });

      tbody.appendChild(tr);
    });

    recordingsList.appendChild(tableNode);

  } catch (error) {
    console.error('Error loading recordings:', error);
    recordingsList.innerHTML = '<p>Error loading data.</p>';
  }
}

let pollingInterval: any = null;

function ensurePolling() {
  if (pollingInterval) return;
  console.log('Starting background polling for transcripts...');
  pollingInterval = setInterval(() => {
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
  stopPolling();
  window.scrollTo(0, 0);
});

// Reload on date change
dateFilter.addEventListener('change', loadRecordings);

async function handleRecording(button: HTMLButtonElement, includeDate: boolean = false) {
  if (!mediaRecorder || mediaRecorder.state === 'inactive') {
    try {
      console.log('Starting recording...');
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorder = new MediaRecorder(stream);
      mediaRecorder.ondataavailable = (event: BlobEvent) => {
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
          } else {
            console.error('Failed to upload audio');
          }
        } catch (error) {
          console.error('Error uploading audio:', error);
        }
      };
      mediaRecorder.start();
      button.textContent = 'Stop Recording';
      button.classList.add('recording');
    } catch (error) {
      console.error('Error accessing microphone:', error);
    }
  } else {
    console.log('Stopping recording...');
    mediaRecorder.stop();
    
    recordBtn.textContent = 'Record';
    recordBtn.classList.remove('recording');

    const contBtn = document.getElementById('recordBtnContinuation') as HTMLButtonElement | null;
    if (contBtn) {
      contBtn.textContent = 'Continue Recording';
      contBtn.classList.remove('recording');
    }
    
    audioChunks = [];
  }
}

recordBtn.addEventListener('click', () => handleRecording(recordBtn, false));

const recordBtnContinuation = document.getElementById('recordBtnContinuation') as HTMLButtonElement | null;
if (recordBtnContinuation) {
  recordBtnContinuation.addEventListener('click', () => handleRecording(recordBtnContinuation, true));
}