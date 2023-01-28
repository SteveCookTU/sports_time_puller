use crate::app::*;
use leptos::*;
use leptos_meta::provide_meta_context;

#[component]
pub fn Common(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <div>
            <div class="flex bg-gray-500 h-16 w-screen drop-shadow-xl justify-around items-center">
                <a class="nav-button" href="/mlb">"MLB"</a>
                <a class="nav-button" href="/nba">"NBA"</a>
                <a class="nav-button" href="/nhl">"NHL"</a>
            </div>
            <App />
        </div>
    }
}