use leptos::*;
use crate::models::dtos::{RecordingFile, TaskGroup};
use crate::components::task_group_selector::TaskGroupSelector;
use uuid::Uuid;

#[component]
pub fn RecordingList(
  recordings: MaybeSignal<Vec<RecordingFile>>,
  groups: MaybeSignal<Vec<TaskGroup>>,
  on_group_change: Callback<(Uuid, Option<Uuid>)>,
  on_title_change: Callback<(Uuid, String)>,
  on_delete: Callback<Uuid>
) -> impl IntoView {
  view! {
    <table class="data-table">
      <thead>
        <tr>
          <th>"No."</th>
          <th>"Title"</th>
          <th>"Status"</th>
          <th>"Group"</th>
          <th>"Audio"</th>
          <th>"Time"</th>
          <th>"Action"</th>
        </tr>
      </thead>
      <tbody>
        <For
          each=move || recordings.get().into_iter().enumerate()
          key=|(idx, rec)| (rec.id, rec.status.clone(), *idx)
          children={
            let groups = groups.clone();
            let on_group_change = on_group_change.clone();
            let on_title_change = on_title_change.clone();
            let on_delete = on_delete.clone();
            move |(index, rec)| {
              view! {
                <RecordingRow
                  index=index
                  rec=rec
                  groups=groups.clone()
                  on_group_change=on_group_change
                  on_title_change=on_title_change
                  on_delete=on_delete
                />
              }
            }
          }
        />
      </tbody>
    </table>
  }
}

#[component]
fn RecordingRow(
  index: usize,
  rec: RecordingFile,
  groups: MaybeSignal<Vec<TaskGroup>>,
  on_group_change: Callback<(Uuid, Option<Uuid>)>,
  on_title_change: Callback<(Uuid, String)>,
  on_delete: Callback<Uuid>
) -> impl IntoView {
  let id = rec.id;
  let rec_path = rec.path.clone();
  let rec_name = rec.name.clone();
  let rec_status = rec.status.clone();
  let rec_group_id = rec.group_id;

  // Extract title and transcript from JSON
  let (title, full_text) = match &rec.transcription {
    Some(json) => {
      let title = json.get("title")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| rec.name.clone());
      let transcript = json.get("transcript")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
      (title, transcript)
    },
    None => (rec.name.clone(), "".to_string())
  };

  // State for editing title
  let (is_editing, set_is_editing) = create_signal(false);
  let (edit_title, set_edit_title) = create_signal(title.clone());

  // Extract time from filename
  let time_str = {
    let match_res = rec_name.match_indices('_').collect::<Vec<_>>();
    if let Some(&(idx, _)) = match_res.last() {
      if let Some(dot_idx) = rec_name.find('.') {
        if idx < dot_idx {
          let ts_str = &rec_name[idx+1..dot_idx];
          if let Ok(ts) = ts_str.parse::<i64>() {
            use chrono::{TimeZone, Local};
            let datetime = Local.timestamp_opt(ts, 0).unwrap();
            datetime.format("%I:%M %p").to_string()
          } else {
            "".to_string()
          }
        } else { "".to_string() }
      } else { "".to_string() }
    } else { "".to_string() }
  };

  view! {
    <tr>
      <td class="col-no">{index + 1}</td>
      <td class="col-title" title=full_text>
        <Show
          when=move || is_editing.get()
          fallback={
            let title = title.clone();
            move || {
              let title_for_click = title.clone();
              view! {
                <div class="flex items-center justify-between group">
                  <span>{title.clone()}</span>
                  <button
                    class="btn-icon opacity-0 group-hover:opacity-100 transition-opacity ml-2"
                    on:click=move |_| {
                      set_edit_title.set(title_for_click.clone());
                      set_is_editing.set(true);
                    }
                    title="Edit Title"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                    </svg>
                  </button>
                </div>
              }
            }
          }
        >
          <div class="flex items-center gap-1">
            <input
              type="text"
              class="border rounded px-1 py-0.5 text-sm w-full"
              prop:value=move || edit_title.get()
              on:input=move |ev| set_edit_title.set(event_target_value(&ev))
              on:keydown=move |ev| {
                if ev.key() == "Enter" {
                  on_title_change.call((id, edit_title.get()));
                  set_is_editing.set(false);
                } else if ev.key() == "Escape" {
                  set_is_editing.set(false);
                }
              }
            />
            <button
              class="btn-icon text-green-600"
              on:click=move |_| {
                on_title_change.call((id, edit_title.get()));
                set_is_editing.set(false);
              }
              title="Save"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
            </button>
            <button
              class="btn-icon text-red-600"
              on:click=move |_| set_is_editing.set(false)
              title="Cancel"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
          </div>
        </Show>
      </td>
      <td class="col-status">{rec_status}</td>
      <td class="col-group">
        <TaskGroupSelector
          groups=groups
          selected_id=rec_group_id
          on_change=Callback::new(move |new_group| on_group_change.call((id, new_group)))
        />
      </td>
      <td class="col-audio">
        <audio controls style="height: 30px;" src=rec_path></audio>
      </td>
      <td class="col-time">{time_str}</td>
      <td class="col-action">
        <button class="btn-icon delete-btn" on:click=move |_| on_delete.call(id)>"Delete"</button>
      </td>
    </tr>
  }
}