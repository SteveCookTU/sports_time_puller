use sports_time_puller::components::common::*;

fn main() {
    use leptos::*;
    let _console_init = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx, <Common /> }
    });
}
