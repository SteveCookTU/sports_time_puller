use crate::time_zone::*;
use leptos::*;

#[component]
pub fn time_zone(cx: Scope, value: i8, set_time_zone: WriteSignal<i8>) -> impl IntoView {
    view! {
        cx,
        <select class="bg-transparent text-right border border-gray-600 rounded-md" on:change=move |ev| {
                    set_time_zone(event_target_value(&ev).parse::<i8>().unwrap_or_default());
                } value={value}>
                    <option value={TimeZone::Edt as i8}>{TimeZone::Edt.region()}</option>
                    <option value={TimeZone::Cdt as i8}>{TimeZone::Cdt.region()}</option>
                    <option value={TimeZone::Mdt as i8}>{TimeZone::Mdt.region()}</option>
                    <option value={TimeZone::Pdt as i8}>{TimeZone::Pdt.region()}</option>
        </select>
    }
}
