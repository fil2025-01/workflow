use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::*;
use crate::models::dtos::{RecordingFile, TaskGroup};

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
    let (recordings, _set_recordings) = create_signal(Vec::<RecordingFile>::new());
    let (groups, _set_groups) = create_signal(Vec::<TaskGroup>::new());

    view! {
        <Show
            when=move || view_history.get()
            fallback=move || view! {
                <div id="recordingSection" class="container">
                    <div class="flex flex-col flex-wrap content-center justify-center">
                        <h1>"Audio Workflow"</h1>
                        <div class="mt-5 flex gap-2 w-full" style="max-width: 300px;">
                            <RecordButton/>
                            <button
                                id="viewHistoryBtn"
                                class="btn btn-lg flex-1"
                                on:click=move |_| set_view_history.set(true)
                            >
                                "Recordings"
                            </button>
                        </div>
                    </div>
                </div>
            }
        >
            <div id="historySection">
                <h2 class="text-lg mb-2">"Recording History"</h2>
                <div class="flex items-center justify-between mb-2 pb-2 border-b bg-cadetblue p-2">
                    <div class="flex items-center">
                        <button
                            id="backBtn"
                            class="btn mr-2 rounded-md"
                            on:click=move |_| set_view_history.set(false)
                        >
                            "Back"
                        </button>
                        <span id="statsLabel" class="text-sm text-gray-600">
                            "Total Recordings: " {move || recordings.get().len()}
                        </span>
                    </div>
                    <div class="flex items-center">
                        <button id="recordBtnContinuation" class="btn mr-2 rounded-md">"Continue Recording"</button>
                    </div>
                    <DateFilter on_change=move |date| {
                        logging::log!("Date changed to: {}", date);
                    }/>
                </div>
                <div id="recordingsList">
                    <RecordingList
                        recordings=recordings.into()
                        groups=groups.into()
                        on_group_change=Callback::new(move |(rec_id, group_id)| {
                            logging::log!("Rec {} group changed to {:?}", rec_id, group_id);
                        })
                        on_delete=Callback::new(move |id| {
                            logging::log!("Delete request for {}", id);
                        })
                    />
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