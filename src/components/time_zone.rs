use crate::time_zone::*;
use leptos::*;

#[component]
pub fn time_zone(cx: Scope, value: i8, set_time_zone: WriteSignal<i8>) -> impl IntoView {
    view! {
        cx,
        <select class="bg-transparent text-right" on:change=move |ev| {
                    set_time_zone(event_target_value(&ev).parse::<i8>().unwrap_or_default());
                } value={value}>
                    <option value={TimeZone::Est as i8}>{TimeZone::Est.region()}</option>
                    <option value={TimeZone::Cst as i8}>{TimeZone::Cst.region()}</option>
                    <option value={TimeZone::Mst as i8}>{TimeZone::Mst.region()}</option>
                    <option value={TimeZone::Pst as i8}>{TimeZone::Pst.region()}</option>
        </select>
    }
}
