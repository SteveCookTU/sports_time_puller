use crate::components::mlb::*;
use crate::components::nhl::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn app(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path="" view= move |cx| view! {
                    cx,
                    <main class="my-0 mx-auto max-w-3xl text-center">
                        <h2 class="p-6 text-4xl">"Welcome to the Sports Time Puller"</h2>
                        <p class="px-10 pb-10 text-center">"Click an option on the navigation bar above to get started."</p>
                    </main>
                }/>
            </Routes>
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
                    <main>
                        <Nhl />
                    </main>
                }/>
            </Routes>
        </Router>
    }
}
