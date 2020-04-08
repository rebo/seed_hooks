// all of this is super alpha and most likely broken so dont use this just yet


use comp_state::*;
use seed::prelude::*;

use seed::browser::fetch::FetchError;

use serde::de::DeserializeOwned;

pub type FetchResult<T> = std::result::Result<T, seed::browser::fetch::FetchError>;

#[topo::nested]
pub fn fetch_string_once<'a>(
    request: impl Into<Request<'a>> + 'static + Clone,
) -> StateAccess<Option<FetchResult<String>>> {
    let id = topo::Id::current();

    if !comp_state::state_exists_for_topo_id::<Option<FetchResult<String>>>(id) {
        let result = use_state_current(|| None);
        fetch_string(request.clone(), result);
    } else {
        use_state_current::<Option<FetchResult<String>>, _>(|| None);
    }
    use_state_current::<Option<FetchResult<String>>, _>(|| None)
}

pub struct FetchData<T, R>
where
    R: Into<Request<'static>> + 'static + Clone,
{
    pub result: Option<FetchResult<T>>,
    pub request: R,
}

#[topo::nested]
pub fn fetch_later<T, R>(request: R) -> StateAccess<FetchData<T, R>>
where
    T: DeserializeOwned + 'static,
    R: Into<Request<'static>> + 'static + Clone,
{
    // let request = request.into();
    // let request = request.into();
    use_state(|| FetchData::<T, R> {
        result: None,
        request,
    })

    // let id = topo::Id::current();
    // let result = use_state_current(|| FetchData {
    //     result: None,
    //     request: request,
    // });
}

#[topo::nested]
pub fn fetch_once<'a, T, Mdl, Ms, R>(request: R, msg: Ms) -> StateAccess<FetchData<T, R>>
where
    Ms: Default + 'static,
    Mdl: 'static,
    T: DeserializeOwned + 'static,
    R: Into<Request<'static>> + 'static + Clone,
{
    let id = topo::Id::current();

    if !comp_state::state_exists_for_topo_id::<FetchData<T, R>>(id) {
        let result = use_state_current(|| FetchData {
            result: None,
            request: request.clone(),
        });
        fetch_json::<T, Mdl, Ms, Node<Ms>, _>(request.clone(), result, msg);
    } else {
        use_state_current::<FetchData<T, R>, _>(|| FetchData {
            result: None,
            request: request.clone(),
        });
    }
    use_state_current::<FetchData<T, R>, _>(|| FetchData {
        result: None,
        request: request.clone(),
    })
}

#[topo::nested]
pub fn fetch_string<'a>(
    request: impl Into<Request<'a>> + 'static,
    state_access: StateAccess<Option<FetchResult<String>>>,
) {
    wasm_bindgen_futures::spawn_local({
        async move {
            let response = seed::browser::fetch::fetch(request).await;
            match response {
                Ok(ok_response) => {
                    if ok_response.status().is_ok() {
                        let string = ok_response.text().await.expect("String can't be created");
                        state_access.set(Some(Ok(string)));
                    } else {
                        state_access.set(Some(Err(FetchError::StatusError(ok_response.status()))))
                    }
                }
                Err(error_response) => state_access.set(Some(Err(error_response))),
            }
        }
    });
}

// #[topo::nested]
// pub fn fetch<'a>(request: impl Into<Request<'a>>) -> Result<Response> {
pub fn fetch_json<'a, T, Mdl, Ms, Vw: IntoNodes<Ms> + 'static, R>(
    request: R,
    state_access: StateAccess<FetchData<T, R>>,
    msg: Ms,
) where
    Ms: 'static,
    Mdl: 'static,
    R: Into<Request<'static>> + 'static + Clone,
    T: DeserializeOwned + 'static,
{
    // let result = use_state(|| None);
    let request = request.clone();
    wasm_bindgen_futures::spawn_local({
        async move {
            let response = seed::browser::fetch::fetch(request.clone()).await;
            match response {
                Ok(ok_response) => {
                    if ok_response.status().is_ok() {
                        match ok_response.json::<T>().await {
                            Ok(value) => state_access.set(FetchData {
                                result: Some(Ok(value)),
                                request: request.clone(),
                            }),
                            Err(serde_error) => state_access.set(FetchData {
                                result: Some(Err(serde_error)),
                                request: request.clone(),
                            }),
                        }
                    } else {
                        state_access.set(FetchData {
                            result: Some(Err(FetchError::StatusError(ok_response.status()))),
                            request: request.clone(),
                        })
                    }
                }
                Err(error_response) => state_access.set(FetchData {
                    result: Some(Err(error_response)),
                    request: request.clone(),
                }),
            }

            crate::schedule_update::schedule_update::<Ms, Mdl, Vw>(msg);
        }
    });
}

pub trait StateAccessFetchGetValue<T> {
    fn do_fetch<Mdl, Ms, Vw: IntoNodes<Ms> + 'static>(self, msg: Ms) -> Self
    where
        Ms: 'static,
        Mdl: 'static;
    fn fetched_value(self) -> Option<T>;
    fn is_ok(self) -> bool;
}
impl<T, R> StateAccessFetchGetValue<T> for StateAccess<FetchData<T, R>>
where
    T: Clone + DeserializeOwned + 'static,
    R: Into<Request<'static>> + 'static + Clone,
{
    fn do_fetch<Mdl, Ms, Vw: IntoNodes<Ms> + 'static>(self, msg: Ms) -> Self
    where
        Ms: 'static,
        Mdl: 'static,
    {
        let request = self.get_with(|fd| fd.request.clone());
        fetch_json::<T, Mdl, Ms, Node<Ms>, _>(request, self, msg);
        self
    }

    fn fetched_value(self) -> Option<T> {
        self.get_with(|fetch_response| {
            if let Some(maybe_t) = &fetch_response.result {
                if let Ok(is_a_t) = maybe_t {
                    Some(is_a_t.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
    fn is_ok(self) -> bool {
        self.fetched_value().is_some()
    }
}

pub trait StateAccessResultUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms, T, R> StateAccessResultUpdateEl<El<Ms>> for StateAccess<FetchData<T, R>>
where
    T: seed::virtual_dom::update_el::UpdateEl<Ms> + 'static + Clone + DeserializeOwned,
    R: Into<Request<'static>> + 'static + Clone,
{
    fn update_el(self, el: &mut El<Ms>) {
        if let Some(val) = self.fetched_value() {
            val.update_el(el)
        }
    }
}
