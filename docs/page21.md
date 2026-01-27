[Prev](./page20.md)

# Plan: Detailed Task Extraction vs. Summarization

**Goal:** Transform a single "brain dump" recording into multiple, granular task entries in the database. Instead of a high-level summary that loses detail, the system will explode one audio file into a list of specific, actionable items.

## The Problem
Currently, if you record: *"I fixed the auth bug, then I called John about the API, and finally I updated the documentation."*
*   **Current Behavior:** Creates 1 entry. Title: "Fixed Auth Bug & Other Tasks". Transcript: "Summary of actions..."
*   **Issue:** The details ("Called John", "Updated Docs") are buried in the transcript text. They are not individual rows in the table, meaning they cannot be assigned to different "Day Parts" or checked off individually.

## The Solution: "One-to-Many" Explosion
We will modify the processing pipeline so that one upload triggers the creation of *multiple* database rows.

### 1. Updated Prompt Strategy (`src/service/prompt.md`)
We need to instruct Gemini to return a **list of tasks** rather than a single summary.

**New Prompt Structure:**
> "Analyze the following audio transcript. It may contain a stream-of-consciousness list of actions I performed or need to perform.
> Break this down into distinct, atomic tasks.
> For each distinct task you identify, provide:
> 1. A clear, actionable Title.
> 2. The specific details/context mentioned for that task.
>
> Return the result as a JSON Array of objects: `[{ "title": "...", "details": "..." }, ...]`"

### 2. Backend Logic Update (`src/service/transcription.rs` & `api/recordings.rs`)
The current flow is: `Upload -> Insert 1 DB Row -> Transcribe -> Update that 1 DB Row`.

**New Flow:**
1.  **Upload:** Save audio file to disk. Insert **Parent** Recording row (Status: PROCESSING).
2.  **Transcribe:** Send audio to Gemini.
3.  **Parse:** Receive the JSON Array of tasks from Gemini.
4.  **Explode:**
    *   **If 1 Task found:** Update the Parent row as usual.
    *   **If > 1 Task found:**
        *   Keep the Parent row as the "Source Artifact" (maybe mark as ARCHIVED or PARENT).
        *   Insert **new rows** into the `recordings` table for each extra task.
        *   *Challenge:* These new rows won't have unique audio files.
        *   *Solution:* They can share the same `file_path` (pointing to the source audio) OR we can treat them as "Text-Only" entries derived from the audio.
        *   *Better Solution:* The `recordings` table should probably be renamed to `tasks` or `items`, where `audio_path` is optional. For now, we can just duplicate the file path or leave it null for the child tasks and link them via a `parent_id`.

### 3. Database Schema Migration
We need a way to link these exploded items back to the original audio file.
*   **Add Column:** `parent_id` (UUID, nullable) to `recordings` table.
*   **Logic:**
    *   The original upload is the "Parent".
    *   The extracted tasks are "Children" linked to that Parent.

### 4. UI Adjustments (`src/components/recording_list.rs`)
*   **Visual Indentation:** If an item has a `parent_id`, maybe indent it or group it visually under the original recording?
*   **Or Flat List:** Just treat them as normal items. The user cares about the *Tasks*, not the file structure. A flat list is probably better for the "Day Part" workflow.

## Implementation Roadmap

### Step 1: Prompt Engineering
*   Modify `src/service/prompt.md` to request a JSON Array.
*   Test with sample "multi-task" audio to ensure Gemini follows instructions.

### Step 2: Schema Change
*   `cargo sqlx migrate add add_parent_id_to_recordings`
*   Apply migration.

### Step 3: Refactor Transcription Service
*   Update `transcribe_and_update` to handle a `Vec<Task>` response.
*   Iterate through the array:
    *   Task 1 updates the original row.
    *   Task 2..N create new rows, copying the `file_path` but setting `parent_id` to the original row's ID.

### Step 4: Frontend Handling
*   Ensure the `RecordingList` correctly displays these new items (it should happen automatically if they are just rows in the DB).

[Prev](./page20.md)
