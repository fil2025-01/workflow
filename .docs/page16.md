[Prev](./page15.md) | [Next](./page17.md)

# Plan: Grouping Recordings by Day Parts

We will introduce a grouping system to categorize recordings (tasks) into specific "Day Parts". This helps in organizing the daily workflow into distinct phases: Delegation, Implementation, and Recurring Tasks.

## 1. Database Schema Changes

### New Table: `task_groups`
We will create a lookup table for the groups to ensure consistency and extensibility.

*   **Table Name**: `task_groups`
*   **Columns**:
    *   `id` (UUID, PK)
    *   `name` (TEXT, NOT NULL) - e.g., "Day Part 1", "Day Part 2", "Day Part 3"
    *   `description` (TEXT) - e.g., "Delegation", "Implementation", "Recurring Tasks"
    *   `ordering` (INT) - To control display order (1, 2, 3)

### Modify Table: `recordings`
We will link recordings to these groups.

*   **New Column**: `group_id` (UUID, nullable, FK -> `task_groups.id`)
*   **Constraint**: Foreign key to `task_groups`.

### Migration Strategy
1.  Create `task_groups` table.
2.  Seed the 3 default groups:
    *   **Part 1**: Delegation ("Can I delegate this task?")
    *   **Part 2**: Implementation ("Doing/Implementing the task")
    *   **Part 3**: Recurring ("Recurring tasks, emails, etc.")
3.  Add `group_id` to `recordings`.

## 2. Backend Implementation (`src/api/`)

### New Endpoints
*   `GET /groups`: Returns the list of available task groups (ordered by `ordering`).

### Updated Endpoints
*   `PATCH /recordings/{id}`:
    *   **Purpose**: Update mutable fields of a recording.
    *   **Payload**: `{ "group_id": "..." }`
    *   **Logic**: Validate the `group_id` and update the record in the database.
*   `GET /recordings`:
    *   **Update**: Join with `task_groups` or simply fetch the `group_id` so the frontend knows the current assignment.

### Models (`src/models/`)
*   Create `TaskGroup` struct.
*   Update `RecordingFile` struct to include `group_id` (and potentially the group name if we do a join, but `group_id` is sufficient for a dropdown).

## 3. Frontend Implementation (`static/`)

### API Integration
*   Fetch available groups from `GET /groups` on page load.
*   Implement a function to call `PATCH /recordings/{id}` when a user changes the group.

### UI Changes (`index.html` & `src/script.ts`)
*   **Table Header**: Add a "Group" column.
*   **Table Row**:
    *   Insert a `<select>` dropdown in the "Group" column.
    *   Populate options from the fetched groups.
    *   Select the current `group_id` for that recording.
*   **Event Handling**: Listen for `change` events on the dropdown to trigger the API update.

## 4. Execution Steps

1.  **Migration**: Write and apply the SQL migration (`migrations/YYYYMMDDHHMMSS_add_task_groups.sql`).
2.  **Backend**: Implement `TaskGroup` model, `GET /groups` handler, and `PATCH /recordings/{id}` handler.
3.  **Frontend**: Update `script.ts` to fetch groups and render the dropdowns in the recording list.
4.  **Verify**: Test assigning groups to tasks and ensuring persistence on reload.

[Prev](./page15.md) | [Next](./page17.md)
