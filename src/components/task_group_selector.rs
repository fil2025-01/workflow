use leptos::*;
use crate::models::dtos::TaskGroup;
use uuid::Uuid;

#[component]
pub fn TaskGroupSelector(
    groups: MaybeSignal<Vec<TaskGroup>>,
    selected_id: Option<Uuid>,
    on_change: Callback<Option<Uuid>>
) -> impl IntoView {
    let handle_change = move |ev| {
        let val = event_target_value(&ev);
        if val.is_empty() {
            on_change.call(None);
        } else if let Ok(id) = Uuid::parse_str(&val) {
            on_change.call(Some(id));
        }
    };

    view! {
        <select
            class="p-1 border rounded text-sm"
            on:change=handle_change
        >
            <option value="" selected=selected_id.is_none()>"Select Group"</option>
            <For
                each=move || groups.get()
                key=|group| group.id
                children=move |group| {
                    let group_id = group.id;
                    let is_selected = selected_id == Some(group_id);
                    view! {
                        <option
                            value=group_id.to_string()
                            selected=is_selected
                            title=group.description.clone().unwrap_or_default()
                        >
                            {group.name.clone()}
                        </option>
                    }
                }
            />
        </select>
    }
}
