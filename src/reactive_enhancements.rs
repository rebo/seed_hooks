use seed::{prelude::*,*};
use atomic_hooks::*;
use atomic_hooks::atom::Atom;
use atomic_hooks::state_access::{CloneState, StateAccess};
use atomic_hooks::reversible_atom::ReversibleAtom;

pub trait ReactiveEnhancements<T> {
    fn debounce_update<F:FnOnce(&mut T) -> () + 'static,Ms, Mdl, INodes>(self, timeout: f64, app: Option<App<Ms, Mdl, INodes>>, func: F) where
    INodes: IntoNodes<Ms> + 'static,;

    fn throttle_update<F:FnOnce(&mut T) -> () + 'static>(self, timeout: f64, func: F) ;
}

impl<T> ReactiveEnhancements<T> for Atom<T>
where
    T: 'static,
{
    fn debounce_update<F:FnOnce(&mut T) -> () + 'static,Ms, Mdl, INodes>(self, timeout: f64, app: Option<App<Ms, Mdl, INodes>>, func: F)where
    INodes: IntoNodes<Ms> + 'static,{

        let performance = window()
            .performance()
            .expect("performance should be available");


        let cached_time_accessor = use_state(||{ performance.now()});
        cached_time_accessor.set(performance.now());
       
       
        let timeout = gloo_timers::callback::Timeout::new(timeout as u32, move || {
            if performance.now() - cached_time_accessor.get() > timeout {
                
                self.update(func);
                if let Some(app) = app {app.update_with_option(None)};
            }
            
        });
        
        timeout.forget();
    }

    fn throttle_update<F:FnOnce(&mut T) -> () + 'static>(self, timeout: f64, func: F){

        let performance = window()
            .performance()
            .expect("performance should be available");


        let cached_time_accessor = use_state(||None);

        if let Some(cached_time) = cached_time_accessor.get(){
        if performance.now() - cached_time > timeout {
            cached_time_accessor.set(Some(performance.now()));
            self.update(func);
        }
        } else {
            self.update(func);
            cached_time_accessor.set(Some(performance.now()));
        }
        
        
        
    }
}


impl<T> ReactiveEnhancements<T> for ReversibleAtom<T>
where
    T: 'static + Clone,
{
    fn debounce_update<F:FnOnce(&mut T) -> () + 'static,Ms, Mdl, INodes>(self, timeout: f64, app: Option<App<Ms, Mdl, INodes>>, func: F)where
    INodes: IntoNodes<Ms> + 'static,{

        let performance = window()
            .performance()
            .expect("performance should be available");


        let cached_time_accessor = use_state(||{ performance.now()});
        cached_time_accessor.set(performance.now());
       
       
        let timeout = gloo_timers::callback::Timeout::new(timeout as u32, move || {
            if performance.now() - cached_time_accessor.get() > timeout {
                
                self.update(func);
                if let Some(app) = app {app.update_with_option(None)};
            }
            
        });
        
        timeout.forget();
    }

    fn throttle_update<F:FnOnce(&mut T) -> () + 'static>(self, timeout: f64, func: F){

        let performance = window()
            .performance()
            .expect("performance should be available");


        let cached_time_accessor = use_state(||None);

        if let Some(cached_time) = cached_time_accessor.get(){
        if performance.now() - cached_time > timeout {
            cached_time_accessor.set(Some(performance.now()));
            self.update(func);
        }
        } else {
            self.update(func);
            cached_time_accessor.set(Some(performance.now()));
        }
        
        
        
    }
}

impl<T> ReactiveEnhancements<T> for StateAccess<T>
where
    T: 'static,
{
    fn debounce_update<F:FnOnce(&mut T) -> () + 'static,Ms, Mdl, INodes>(self, timeout: f64, app: Option<App<Ms, Mdl, INodes>>, func: F)where
    INodes: IntoNodes<Ms> + 'static,{

        let performance = window()
            .performance()
            .expect("performance should be available");

        let cached_time_accessor = use_state(||{ performance.now()});
        cached_time_accessor.set(performance.now());
       
       
        let timeout = gloo_timers::callback::Timeout::new(timeout as u32, move || {
            if performance.now() - cached_time_accessor.get() > timeout {
                
                self.update(func);
                if let Some(app) = app {app.update_with_option(None)};
            }
            
        });
        
        timeout.forget();
    }

    fn throttle_update<F:FnOnce(&mut T) -> () + 'static>(self, timeout: f64, func: F) {

        let performance = window()
            .performance()
            .expect("performance should be available");


        let cached_time_accessor = use_state(||None);

        if let Some(cached_time) = cached_time_accessor.get(){
        if performance.now() - cached_time > timeout {
            cached_time_accessor.set(Some(performance.now()));
            self.update(func);
        }
        } else {
            self.update(func);
            cached_time_accessor.set(Some(performance.now()));
        }
        
        
        
    }
}





