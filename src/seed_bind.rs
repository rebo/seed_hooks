use atomic_hooks::Observable;
use seed::{prelude::*, *};
use atomic_hooks::state_access::StateAccess;
use atomic_hooks::atom::Atom;

pub trait UpdateElLocal<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms> UpdateElLocal<El<Ms>> for (seed::Attrs, seed::EventHandler<Ms>) {
    fn update_el(self, el: &mut El<Ms>) {
        self.0.update_el(el);
        self.1.update_el(el);
    }
}


pub trait InputBind<Ms,T> where Ms: 'static, T: 'static+ std::str::FromStr + std::fmt::Display {
   fn bind( self, attr: At) -> (seed::virtual_dom::attrs::Attrs, seed::EventHandler<Ms>);
}


impl <Ms,T> InputBind<Ms,T>   for StateAccess<T>   where Ms: 'static, T: 'static+ std::str::FromStr + std::fmt::Display {
    fn bind( self, attr: At) -> (seed::virtual_dom::attrs::Attrs, seed::EventHandler<Ms>){
        let val_disp = self.observe_with(|v| format!("{}", v));
            (
            attrs!(attr => val_disp),
            input_ev(Ev::Input, move |ev| {
                if let Ok(parsed_type) = ev.parse::<T>() {
                    self.set(parsed_type);
                }
            }),
        )
    }
}

impl <Ms,T> InputBind<Ms,T>   for Atom<T>   where Ms: 'static, T: 'static+ std::str::FromStr + std::fmt::Display {
    fn bind( self, attr: At) -> (seed::virtual_dom::attrs::Attrs, seed::EventHandler<Ms>){
        let val_disp = self.observe_with(|v| format!("{}", v));
            (
            attrs!(attr => val_disp),
            input_ev(Ev::Input, move |ev| {
                if let Ok(parsed_type) = ev.parse::<T>() {
                    self.set(parsed_type);
                }
            }),
        )
    }
}

