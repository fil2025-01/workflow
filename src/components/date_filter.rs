use leptos::*;

#[component]
pub fn DateFilter(
    #[prop(into)] on_change: Callback<String>
) -> impl IntoView {
    let today = || {
        #[cfg(not(feature = "ssr"))]
        {
            js_sys::Date::new_0()
                .to_iso_string()
                .as_string()
                .unwrap_or_default()
                .split('T')
                .next()
                .unwrap_or_default()
                .to_string()
        }
        #[cfg(feature = "ssr")]
        {
            "".to_string()
        }
    };

    let (date, set_date) = create_signal(today());

    let handle_change = move |ev| {
        let val = event_target_value(&ev);
        set_date.set(val.clone());
        on_change.call(val);
    };

    view! {
        <div class="flex items-center text-sm gap-2">
            <label for="dateFilter">"Date:"</label>
            <input
                type="date"
                id="dateFilter"
                class="p-2 border rounded-md"
                prop:value=date
                on:change=handle_change
            />
        </div>
    }
}
