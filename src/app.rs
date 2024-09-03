use crate::components::cfb::*;
use crate::components::mlb::*;
use crate::components::nhl::*;
use crate::components::soccer::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn app() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=  move || view! {
                    <main class="my-0 mx-auto max-w-3xl text-center">
                        <h2 class="p-6 text-4xl">"Welcome to the Sports Time Puller"</h2>
                        <p class="px-10 pb-10 text-center">"Click an option on the navigation bar above to get started."</p>
                    </main>
                }/>
                <Route path="cfb" view=  move || view! {
                    <main>
                        <Cfb />
                    </main>
                }/>
                <Route path="mlb" view=  move || view! {
                    <main>
                        <Mlb />
                    </main>
                }/>
                <Route path="nhl" view=  move || view! {
                    <main>
                        <Nhl />
                    </main>
                }/>
                <Route path="soccer" view=  move || view! {
                    <main>
                        <Soccer />
                    </main>
                }/>
            </Routes>
        </Router>
    }
}
