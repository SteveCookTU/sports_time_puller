use leptos::web_sys::Event;
use leptos::*;
use num_traits::Num;
use std::future::Future;
use std::str::FromStr;

#[component]
pub fn teams<OutputT, T, F, Fu, GetTeams>(
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
    let teams = create_resource(move || (), move |_| get_teams());

    view! {
        <select class="bg-transparent text-right border border-gray-600 rounded-md" on:change=on_change value={value}>
            {
                move || {
                    teams.with(|teams: &Option<OutputT>| {
                        if let Some(teams) = teams {
                            teams.as_ref().iter().map(|(k, v)| {
                                view! {
                                    <option value={*k}>{v}</option>
                                }
                            }).collect::<Vec<_>>().into_view()
                        } else {
                            view! {
                                <></>
                            }.into_view()
                        }
                    })
                }
            }
        </select>
    }
}
