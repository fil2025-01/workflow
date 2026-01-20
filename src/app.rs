use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::*;
use crate::models::dtos::{RecordingFile, TaskGroup};
use uuid::Uuid;

#[server(GetRecordings, "/api")]
pub async fn get_recordings(date: Option<String>) -> Result<Vec<RecordingFile>, ServerFnError> {
    use crate::api::handlers::list_recordings_inner;
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    list_recordings_inner(pool, date).await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetGroups, "/api")]
pub async fn get_groups() -> Result<Vec<TaskGroup>, ServerFnError> {
    use crate::api::handlers::get_groups_inner;
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    get_groups_inner(pool).await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(UpdateRecordingGroup, "/api")]
pub async fn update_recording_group(id: Uuid, group_id: Option<Uuid>) -> Result<(), ServerFnError> {
    use crate::api::handlers::update_recording_inner;
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    update_recording_inner(pool, id, group_id).await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(DeleteRecording, "/api")]
pub async fn delete_recording(id: Uuid) -> Result<(), ServerFnError> {
    use crate::api::handlers::delete_recording_by_id_inner;
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?;

    delete_recording_by_id_inner(pool, id).await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/workflow.css"/>
    <Title text="Workflow"/>

    <Router>
      <div class="app-layout h-screen">
        <Sidebar/>
        <main class="main-content">
          <Routes>
            <Route path="" view=HomePage/>
            <Route path="/*any" view=NotFound/>
          </Routes>
        </main>
      </div>
    </Router>
  }
}

#[component]
fn Sidebar() -> impl IntoView {
  view! {
    <div class="sidebar">
      <div class="sidebar-icon" title="Explorer">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"></path>
          <polyline points="13 2 13 9 20 9"></polyline>
        </svg>
      </div>
      <div class="sidebar-icon" title="Search">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
      </div>
    </div>
  }
}

#[component]
fn HomePage() -> impl IntoView {
  let (view_history, set_view_history) = create_signal(false);
  let (selected_date, set_selected_date) = create_signal(None::<String>);

  let recordings_resource = create_resource(
    move || selected_date.get(),
    |date| async move { get_recordings(date).await }
  );

  let groups_resource = create_resource(
    || (),
    |_| async move { get_groups().await }
  );

  let recordings = Signal::derive(move || {
    recordings_resource.get().and_then(|res| res.ok()).unwrap_or_default()
  });
  let groups = Signal::derive(move || {
    groups_resource.get().and_then(|res| res.ok()).unwrap_or_default()
  });

  let update_group_action = create_server_action::<UpdateRecordingGroup>();
  let delete_rec_action = create_server_action::<DeleteRecording>();

  // Refresh resources when actions complete
  create_effect(move |_| {
    if update_group_action.version().get() > 0 || delete_rec_action.version().get() > 0 {
      recordings_resource.refetch();
    }
  });

  // Polling for pending transcriptions
  create_effect(move |_| {
    let has_pending = recordings.get().iter().any(|r| r.status == "PENDING");
    if has_pending {
      let _ = set_timeout_with_handle(move || {
        recordings_resource.refetch();
      }, std::time::Duration::from_secs(3));
    }
  });

  view! {
    <Show
      when=move || view_history.get()
      fallback=move || view! {
        <div id="recordingSection" class="container">
          <div class="flex flex-col flex-wrap content-center justify-center">
            <h1>"Audio Workflow"</h1>
            <div class="mt-5 flex gap-2 w-full" style="max-width: 300px;">
              <RecordButton on_success=Callback::new(move |_| recordings_resource.refetch())/>
              <button
                id="viewHistoryBtn"
                class="btn btn-lg flex-1"
                on:click=move |_| set_view_history.set(true)>
                "Recordings"
              </button>
            </div>
          </div>
        </div>
      }>
      <div id="historySection">
        <h2 class="text-lg mb-2">"Recording History"</h2>
        <div class="flex items-center justify-between mb-2 pb-2 border-b bg-cadetblue p-2">
          <div class="flex items-center">
            <button
              id="backBtn"
              class="btn mr-2 rounded-md"
              on:click=move |_| set_view_history.set(false)>
              "Back"
            </button>
            <a href="/legacy" class="btn mr-2 rounded-md">"Legacy UI"</a>
            <span id="statsLabel" class="text-sm text-gray-600">
              "Total Recordings: " {move || recordings.get().len()}
            </span>
          </div>
          <div class="flex items-center">
            <RecordButton
              class="btn mr-2 rounded-md"
              label="Continue Recording"
              on_success=Callback::new(move |_| recordings_resource.refetch())
            />
          </div>
          <DateFilter on_change=move |date| {
            set_selected_date.set(Some(date));
          }/>
        </div>
        <div id="recordingsList">
          <Transition fallback=move || view! { <p>"Loading recordings..."</p> }>
            <RecordingList
              recordings=recordings.into()
              groups=groups.into()
              on_group_change=Callback::new(move |(rec_id, group_id)| {
                update_group_action.dispatch(UpdateRecordingGroup { id: rec_id, group_id });
              })
              on_delete=Callback::new(move |id| {
                #[cfg(not(feature = "ssr"))]
                {
                  if web_sys::window().unwrap().confirm_with_message("Delete this recording?").unwrap() {
                    delete_rec_action.dispatch(DeleteRecording { id });
                  }
                }
                #[cfg(feature = "ssr")]
                {
                  let _ = id;
                }
              })
            />
          </Transition>
        </div>
      </div>
    </Show>
  }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = use_context::<leptos_axum::ResponseOptions>();
        if let Some(resp) = resp {
            resp.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
      <h1>"Not Found"</h1>
    }
}