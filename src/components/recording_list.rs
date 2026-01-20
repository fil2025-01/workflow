use leptos::*;
use crate::models::dtos::{RecordingFile, TaskGroup};
use crate::components::task_group_selector::TaskGroupSelector;
use uuid::Uuid;

#[component]
pub fn RecordingList(
    recordings: MaybeSignal<Vec<RecordingFile>>,
    groups: MaybeSignal<Vec<TaskGroup>>,
    on_group_change: Callback<(Uuid, Option<Uuid>)>,
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
          each=move || recordings.get()
          key=|rec| rec.id
          children={
            let groups = groups.clone();
            move |rec| {
              let id = rec.id;
              let rec_path = rec.path.clone();
              let rec_name = rec.name.clone();
              let rec_status = rec.status.clone();
              let rec_group_id = rec.group_id;
              let groups = groups.clone();

              // Extract time from filename
              let time_str = {
                let match_res = rec_name.match_indices('_').collect::<Vec<_>>();
                if let Some(&(idx, _)) = match_res.last() {
                  if let Some(dot_idx) = rec_name.find('.') {
                    if idx < dot_idx {
                      let ts_str = &rec_name[idx+1..dot_idx];
                      if let Ok(ts) = ts_str.parse::<i64>() {
                        // This is crude, but works for placeholder until better formatting
                        format!("{}", ts)
                      } else {
                        "".to_string()
                      }
                    } else { "".to_string() }
                  } else { "".to_string() }
                } else { "".to_string() }
              };

              view! {
                <tr>
                  <td class="col-no">"?"</td> // Index needs careful handling in For
                  <td class="col-title">{rec_name}</td>
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
          }
        />
      </tbody>
    </table>
  }
}
