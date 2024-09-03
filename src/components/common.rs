use crate::app::*;
use leptos::*;
use leptos_meta::provide_meta_context;

#[component]
pub fn Common() -> impl IntoView {
    provide_meta_context();
    view! {
        <div>
            <div class="flex bg-gray-500 h-16 w-screen drop-shadow-xl justify-around items-center">
                <a class="nav-button" href="/cfb">"CFB"</a>
                <a class="nav-button" href="/mlb">"MLB"</a>
                <a class="nav-button" href="/nhl">"NHL"</a>
                <a class="nav-button" href="/soccer">"Soccer"</a>
            </div>
            <App />
        </div>
    }
}
