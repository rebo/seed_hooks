use comp_state::StateAccess;
use seed::prelude::*;

pub trait StateAccessEventHandlers<T>
where
    T: 'static,
{
    fn input_ev<F: FnOnce(&mut T, String) -> () + 'static + Clone, Ms: Default + 'static>(
        &self,
        event: Ev,
        func: F,
    ) -> seed::EventHandler<Ms>;

    fn mouse_ev<
        F: FnOnce(&mut T, web_sys::MouseEvent) -> () + 'static + Clone,
        Ms: Default + 'static,
    >(
        &self,
        event: Ev,
        func: F,
    ) -> seed::EventHandler<Ms>;
}

impl<T> StateAccessEventHandlers<T> for StateAccess<T>
where
    T: 'static,
{
    fn input_ev<F: FnOnce(&mut T, String) -> () + 'static + Clone, Ms: Default + 'static>(
        &self,
        event: Ev,
        func: F,
    ) -> seed::EventHandler<Ms> {
        let accessor = *self;
        input_ev(event, move |text| {
            accessor.update(|val| func(val, text));
            Ms::default()
        })
    }

    fn mouse_ev<
        F: FnOnce(&mut T, web_sys::MouseEvent) -> () + 'static + Clone,
        Ms: Default + 'static,
    >(
        &self,
        event: Ev,
        func: F,
    ) -> seed::EventHandler<Ms> {
        let accessor = *self;
        mouse_ev(event, move |m_ev| {
            accessor.update(|val| func(val, m_ev));
            Ms::default()
        })
    }
}
