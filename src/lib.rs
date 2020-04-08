#![feature(track_caller)]

mod ev_handlers;
pub mod fetch;
pub mod form;
mod schedule_update;
mod seed_bind;
mod update_el;
mod utils;

pub use ev_handlers::StateAccessEventHandlers;
pub use schedule_update::{register_app, schedule_update};
pub use seed_bind::{bind, UpdateElLocal};
pub use update_el::StateAccessUpdateEl;
pub use utils::{
    after_render, after_render_once, get_html_element_by_id, handle_unmount,
    request_animation_frame,
};

pub use comp_state::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
