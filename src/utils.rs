use comp_state::{do_once, topo, StateAccess};
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[topo::nested]
pub fn after_render_once<F: Fn(f64) -> () + 'static + Clone>(func: F) -> StateAccess<bool> {
    do_once(move || after_render(func.clone()))
}

pub fn after_render<F: Fn(f64) -> () + 'static>(func: F) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |delta| {
        func(delta);
        f.borrow_mut().take();
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

pub fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn get_html_element_by_id(id: &str) -> Option<web_sys::HtmlElement> {
    let maybe_elem = document()
        .get_element_by_id(id)
        .map(wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>);

    if let Some(Ok(elem)) = maybe_elem {
        Some(elem)
    } else {
        None
    }
}

pub fn handle_drop_types<Ms>() -> Node<Ms> {
    comp_state::execute_and_remove_drop_types();
    comp_state::reset_unseen_id_list();
    empty![]
}
