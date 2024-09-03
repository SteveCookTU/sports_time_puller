use crate::app::*;
use leptos::*;
use leptos_meta::provide_meta_context;

#[component]
pub fn Common() -> impl IntoView {
    provide_meta_context();
    view! {
        <div>
            <div class="flex bg-gray-500 h-16 w-screen drop-shadow-xl justify-around items-center">
                <a class="nav-button" href="/sports_time_puller/cfb">"CFB"</a>
                <a class="nav-button" href="/sports_time_puller/mlb">"MLB"</a>
                <a class="nav-button" href="/sports_time_puller/nhl">"NHL"</a>
                <a class="nav-button" href="/sports_time_puller/soccer">"Soccer"</a>
            </div>
            <App />
        </div>
    }
}
