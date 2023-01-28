use crate::components::mlb::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes base={"sports_time_puller".to_string()}>
                <Route path="" view=  move |cx| view! {
                    cx,
                    <main class="my-0 mx-auto max-w-3xl text-center">
                        <h2 class="p-6 text-4xl">"Welcome to the Sports Time Puller"</h2>
                        <p class="px-10 pb-10 text-center">"Click an option on the navigation bar above to get started."</p>
                    </main>
                }/>
                <Route path="mlb" view=  move |cx| view! {
                    cx,
                    <main>
                        <Mlb />
                    </main>
                }/>
                <Route path="nba" view=  move |cx| view! {
                    cx,
                    <main class="my-0 mx-auto max-w-3xl text-center">
                        <h2 class="p-6 text-4xl">"NBA"</h2>
                    </main>
                }/>
                <Route path="nhl" view=  move |cx| view! {
                    cx,
                    <main class="my-0 mx-auto max-w-3xl text-center">
                        <h2 class="p-6 text-4xl">"NHL"</h2>
                    </main>
                }/>
            </Routes>
        </Router>
    }
}
