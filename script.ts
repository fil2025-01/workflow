const recordBtn = document.querySelector('#recordBtn') as HTMLButtonElement;
let mediaRecorder: MediaRecorder | null = null;
let audioChunks: Blob[] = [];

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
