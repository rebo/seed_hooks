use seed::prelude::*;
use atomic_hooks::state_access::{StateAccess, CloneState};

pub trait StateAccessUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms, T> StateAccessUpdateEl<El<Ms>> for StateAccess<T>
where
    T: seed::virtual_dom::update_el::UpdateEl<Ms> + 'static + Clone,
{
    fn update_el(self, el: &mut El<Ms>) {
        self.get().update_el(el);
    }
}

pub trait LocalUpdateEl2<T> {
    fn update_el(self, el: &mut T);
}

// impl<Ms: 'static,T,U,A> LocalUpdateEl2<El<Ms>> for ReactiveStateAccess<T,U,A> where T: UpdateEl<Ms> + 'static + Clone{
//     fn update_el(self, el: &mut El<Ms>) {
//         self.observe().update_el(el);
//     }
// }
