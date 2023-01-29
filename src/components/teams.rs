use leptos::web_sys::Event;
use leptos::*;
use num_traits::Num;
use std::future::Future;
use std::str::FromStr;

#[component]
pub fn teams<OutputT, T, F, Fu, GetTeams>(
    cx: Scope,
    value: T,
    on_change: F,
    get_teams: GetTeams,
) -> impl IntoView
where
    T: IntoAttribute + Num + Copy + FromStr + 'static,
    OutputT: AsRef<[(T, String)]> + Serializable + 'static,
    F: Fn(Event) + 'static,
    Fu: Future<Output = OutputT> + 'static,
    GetTeams: Fn() -> Fu + 'static,
{
    let teams = create_resource(cx, move || (), move |_| get_teams());

    view! {
        cx,
        <select class="bg-transparent text-right" on:change=on_change value={value}>
            {
                move || {
                    teams.with(|teams: &OutputT| {
                        teams.as_ref().iter().map(|(k, v)| {
                            view! {
                                cx,
                                <option value={*k}>{v}</option>
                            }
                        }).collect::<Vec<_>>().into_view(cx)
                    })
                }
            }
        </select>
    }
}
