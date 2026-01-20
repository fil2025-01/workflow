use leptos::*;

#[component]
pub fn RecordButton() -> impl IntoView {
    let (is_recording, set_is_recording) = create_signal(false);

    let on_click = move |_| {
        set_is_recording.update(|recording| *recording = !*recording);
        // TODO: Implement actual recording logic with web-sys
    };

    view! {
        <button
            id="recordBtn"
            class="btn btn-lg flex-1"
            class:recording=is_recording
            on:click=on_click
        >
            {move || if is_recording.get() { "Stop Recording" } else { "Record" }}
        </button>
    }
}
