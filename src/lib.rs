#![feature(track_caller)]

mod ev_handlers;

mod schedule_update;
mod seed_bind;
mod update_el;
mod utils;
pub use ev_handlers::StateAccessEventHandlers;
pub use schedule_update::{register_app, schedule_update};
pub use seed_bind::{UpdateElLocal, InputBind};
pub use update_el::{StateAccessUpdateEl, LocalUpdateEl2};
pub use utils::{
    after_render, after_render_once, get_html_element_by_id, //handle_unmount,
    request_animation_frame,
};


pub use atomic_hooks::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
