use comp_state::StateAccess;
use seed::{prelude::*, *};
pub trait UpdateElLocal<T> {
    fn update(self, el: &mut T);
}

impl<Ms> UpdateElLocal<El<Ms>> for (seed::Attrs, seed::EventHandler<Ms>) {
    fn update(self, el: &mut El<Ms>) {
        self.0.update(el);
        self.1.update(el);
    }
}

// pub trait UpdateElLocal<T> {
//     fn update_el(self, el: &mut T);
// }

// impl<Ms> UpdateElLocal<El<Ms>> for (seed::Attrs, seed::EventHandler<Ms>) {
//     fn update_el(self, el: &mut El<Ms>) {
//         self.0.update_el(el);
//         self.1.update_el(el);
//     }
// }

pub fn bind<Ms: Default, T: 'static + std::str::FromStr + std::fmt::Display>(
    attr: At,
    val: StateAccess<T>,
) -> (seed::virtual_dom::attrs::Attrs, seed::EventHandler<Ms>) {
    let val_disp = val.get_with(|v| format!("{}", v));

    (
        attrs!(attr => val_disp),
        input_ev(Ev::Input, move |ev| {
            if let Ok(parsed_type) = ev.parse::<T>() {
                val.set(parsed_type);
            }
            Ms::default()
        }),
    )
}
