use atomic_hooks::{clone_state_with_topo_id, set_state_with_topo_id,TopoKey};
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;


thread_local! {
    static ROOT_ID : atomic_hooks::TopoKey = TopoKey{
        ctx:None,
        id: atomic_hooks::topo::Id::current()
    };
}

pub fn register_app<Ms: 'static, Mdl, Vw: IntoNodes<Ms> + 'static>(
    app: seed::App<Ms, Mdl, Vw>,
) {
    let root_id = ROOT_ID.with(|id| id.clone());

    set_state_with_topo_id(app, root_id);
}

pub fn schedule_update<
    Ms: 'static,
    Mdl: 'static,
    Vw:IntoNodes<Ms> + 'static,
>(
    msg: Ms,
) {
    let root_id = ROOT_ID.with(|id| id.clone());

    if let Some(app) = clone_state_with_topo_id::<App<Ms, Mdl, Vw>>(root_id) {
        // crate::after_render(move |_| app.update(msg))

        let cb = Closure::once(Box::new(move || {
            app.update(msg);
            // web_sys::console::log_1(&"raf called".into());
        }) as Box<dyn FnOnce()>);

        log("scheduling");
        let window = web_sys::window().unwrap();
        let _res = window.request_animation_frame(
            // Note this method call, which uses `as_ref()` to get a `JsValue`
            // from our `Closure` which is then converted to a `&Function`
            // using the `JsCast::unchecked_ref` function.
            cb.as_ref().unchecked_ref(),
        );

        cb.forget();
    }
}
