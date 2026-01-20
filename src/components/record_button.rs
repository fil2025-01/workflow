use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{MediaRecorder, MediaStream, MediaRecorderOptions, BlobEvent, Blob, FormData};
use std::rc::Rc;
use std::cell::RefCell;

#[component]
pub fn RecordButton(
    #[prop(optional)] on_success: Option<Callback<()>>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] label: Option<String>
) -> impl IntoView {
    let (is_recording, set_is_recording) = create_signal(false);

    // Store references to recorder and chunks in Rc<RefCell> to share across closures
    let media_recorder = Rc::new(RefCell::new(None::<MediaRecorder>));
    let audio_chunks = Rc::new(RefCell::new(Vec::<Blob>::new()));

    let on_click = {
        let media_recorder = media_recorder.clone();
        let audio_chunks = audio_chunks.clone();

        move |_| {
            if !is_recording.get() {
                // START RECORDING
                let media_recorder = media_recorder.clone();
                let audio_chunks = audio_chunks.clone();

                spawn_local(async move {
                    let window = web_sys::window().unwrap();
                    let navigator = window.navigator();
                    let media_devices = navigator.media_devices().unwrap();

                    let constraints = web_sys::MediaStreamConstraints::new();
                    constraints.set_audio(&JsValue::from_bool(true));

                    let stream_promise = media_devices.get_user_media_with_constraints(&constraints).unwrap();
                    let stream_js = wasm_bindgen_futures::JsFuture::from(stream_promise).await.unwrap();
                    let stream: MediaStream = stream_js.unchecked_into();

                    let options = MediaRecorderOptions::new();
                    options.set_mime_type("audio/webm");

                    let recorder = MediaRecorder::new_with_media_stream_and_media_recorder_options(&stream, &options).unwrap();

                    // ondataavailable
                    let audio_chunks_inner = audio_chunks.clone();
                    let on_data_callback = Closure::wrap(Box::new(move |ev: BlobEvent| {
                        audio_chunks_inner.borrow_mut().push(ev.data().unwrap());
                    }) as Box<dyn FnMut(BlobEvent)>);
                    recorder.set_ondataavailable(Some(on_data_callback.as_ref().unchecked_ref()));
                    on_data_callback.forget(); // Keep closure alive

                    // onstop
                    let audio_chunks_stop = audio_chunks.clone();
                    let on_stop_callback = Closure::wrap(Box::new(move |_| {
                        let chunks = audio_chunks_stop.borrow().clone();
                        let array = js_sys::Array::new();
                        for chunk in chunks {
                            array.push(&chunk);
                        }
                        let property_bag = web_sys::BlobPropertyBag::new();
                        property_bag.set_type("audio/webm");
                        let blob = Blob::new_with_blob_sequence_and_options(
                            &array,
                            &property_bag
                        ).unwrap();

                        // Upload
                        spawn_local(async move {
                            let form_data = FormData::new().unwrap();
                            form_data.append_with_blob_and_filename("file", &blob, "recording.webm").unwrap();

                            let window = web_sys::window().unwrap();
                            let init = web_sys::RequestInit::new();
                            init.set_method("POST");
                            init.set_body(&form_data);

                            let _ = wasm_bindgen_futures::JsFuture::from(
                                window.fetch_with_str_and_init("/upload", &init)
                            ).await;

                            if let Some(on_success) = on_success {
                                on_success.call(());
                            }
                        });
                    }) as Box<dyn FnMut(JsValue)>);
                    recorder.set_onstop(Some(on_stop_callback.as_ref().unchecked_ref()));
                    on_stop_callback.forget();

                    recorder.start().unwrap();
                    media_recorder.replace(Some(recorder));
                    set_is_recording.set(true);
                });
            } else {
                // STOP RECORDING
                if let Some(recorder) = media_recorder.borrow().as_ref() {
                    recorder.stop().unwrap();
                }
                audio_chunks.borrow_mut().clear();
                set_is_recording.set(false);
            }
        }
    };

    let btn_class = move || format!("btn {}", class.clone().unwrap_or_else(|| "btn-lg flex-1".to_string()));
    let btn_label = move || {
      if is_recording.get() {
        "Stop Recording".to_string()
      } else {
        label.clone().unwrap_or_else(|| "Record".to_string())
      }
    };

    view! {
      <button
        class=btn_class
        class:recording=is_recording
        on:click=on_click>
        {btn_label}
      </button>
    }
}