# Handling the Continuation Recording Button

The "Continue Recording" button (ID: `recordBtnContinuation`) in `static/index.html` was implemented to allow users to add new recordings to a specific date in the past, rather than always defaulting to the current date.

## HTML Structure
In `static/index.html` (line 48), the button is defined within the History Section:
```html
<button id="recordBtnContinuation" class="btn mr-2 rounded-md">Continue Recording</button>
```

## TypeScript Implementation
In `static/script.ts`, this button is handled using a unified `handleRecording` helper function.

### 1. Unified Recording Helper
The `handleRecording` function abstracts the complexity of `MediaRecorder` and handles both the main record button and the continuation button:

```typescript
async function handleRecording(button: HTMLButtonElement, includeDate: boolean = false) {
  // ... media recorder setup ...
  
  mediaRecorder.onstop = async () => {
    // ... blob creation ...
    
    let uploadUrl = '/upload';
    // If includeDate is true, append the currently selected date filter to the query string
    if (includeDate && dateFilter.value) {
      uploadUrl += `?date=${dateFilter.value}`;
    }

    const response = await fetch(uploadUrl, {
      method: 'POST',
      body: formData
    });
    // ...
  };
}
```

### 2. Event Listeners
Two separate listeners are set up for the different recording entry points:

- **Standard Recording**: Always uploads without a date parameter (defaults to today on the server).
  ```typescript
  recordBtn.addEventListener('click', () => handleRecording(recordBtn, false));
  ```

- **Continuation Recording**: Checks the `#recordBtnContinuation` element and, when clicked, tells the helper to include the date filter in the request.
  ```typescript
  const recordBtnContinuation = document.getElementById('recordBtnContinuation') as HTMLButtonElement | null;
  if (recordBtnContinuation) {
    recordBtnContinuation.addEventListener('click', () => handleRecording(recordBtnContinuation, true));
  }
  ```

### 3. Server-Side Integration
The `uploadUrl` constructed in the TypeScript file (e.g., `/upload?date=2026-01-12`) is parsed by the Rust backend in `src/api/handlers.rs`, which ensures the file is saved in the correct year/month/day directory structure under `recordings/`.
