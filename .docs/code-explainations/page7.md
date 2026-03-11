[Prev](./page6.md) | [Next](./page8.md)

# Explanation of `RecordingList` Component

## Location and Definition
The component is defined in `src/components/recording_list.rs`. It is a Leptos component that renders a data table of recordings.

## Where it is used
It is used in `src/app.rs` inside the `HomePage` component. Specifically, it appears within the `view_history` block (the "History View").

## How it is called
The component is invoked with reactive signals for data and callbacks for user actions.

```rust
<RecordingList
  recordings=recordings.into() // Signal<Vec<RecordingFile>>
  groups=groups.into()         // Signal<Vec<TaskGroup>>
  on_group_change=Callback::new(move |(rec_id, group_id)| {
    // Dispatches UpdateRecordingGroup server action
    update_group_action.dispatch(UpdateRecordingGroup { id: rec_id, group_id });
  })
  on_delete=Callback::new(move |id| {
    // Confirms and dispatches DeleteRecording server action
    // ...
  })
/>
```

## Internal Logic
Inside `RecordingList`, a `<For />` component iterates over the `recordings` signal. For each recording, it renders a table row (`<tr>`) displaying:
- Index
- Title (parsed from JSON transcription if available)
- Status
- Group Selector (using `TaskGroupSelector`)
- Audio Player
- Timestamp
- Delete Button

[Prev](./page6.md) | [Next](./page8.md)