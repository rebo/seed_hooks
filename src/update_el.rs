use comp_state::{CloneState, StateAccess};
use seed::prelude::*;

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
