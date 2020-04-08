
// All of the below is super-alpha so dont use just yet :)

use crate::seed_bind::UpdateElLocal;

use comp_state::*;
use seed::virtual_dom::attrs::Attrs;
use seed::{prelude::*, *};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;
use std::marker::PhantomData;
use validator::*;

#[derive(Clone)]
pub struct Form<T>
where
    T: Serialize + Clone + DeserializeOwned + 'static,
{
    pub data: T,
    pub json: serde_json::Value,
    pub field_strings: HashMap<String, String>,
    pub layout_style: LayoutStyle,
    pub validation_errors: Pos,
    pub label_pos: Pos,
    pub container_style: Style,
    pub errors: Result<(), ValidationErrors>,
    pub serde_errors: Result<(), SerdeErrors>,
    pub input_style: InputStyle,
}

#[derive(Clone)]
pub struct SerdeErrors(std::collections::HashMap<String, bool>);

#[derive(Clone)]
pub enum LayoutStyle {
    None,
    GridDirect,
    GridTailwind,
    VerticalDirect,
    VerticalTailwind,
    HorizontalDirect,
    HorizontalTailwind,
    CompactDirect,
    CompactTailwind,
    Custom(InputStyle)
}

impl<T> Form<T>
where
    T: Serialize + Clone + DeserializeOwned + 'static,
{
    fn new(data: T) -> Form<T> {
        let j = serde_json::to_value(&data).expect("Cannot serialize form struct");
        Form::<T> {
            data,
            json: j,
            field_strings: HashMap::new(),
            validation_errors: Pos::Below,
            label_pos: Pos::Unset,
            layout_style: LayoutStyle::GridDirect,
            errors: Ok(()),
            serde_errors: Ok(()),
            container_style: style! {},
            input_style: input_style_none(),
        }
    }
}

pub struct InputBuilder<T, F, Ms>
where
    F: FieldNameEnumTrait,
    T: Clone + 'static + Serialize + DeserializeOwned,
    Ms: 'static,
{
    error_response: std::collections::HashMap<&'static str, &'static str>,
    guidance: Option<String>,   

    form: StateAccess<Form<T>>,
    field_name: F,
    validation_errors_pos: Pos,
    label_pos: Pos,
    label_name: Option<String>,
    _phantom_type: PhantomData<Ms>,
}

#[derive(Clone, PartialEq)]
pub enum Pos {
    Right,
    Left,
    Above,
    Below,
    None,
    Unset,
}

pub trait StyleClassTupleUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms> StyleClassTupleUpdateEl<El<Ms>> for (Style, Attrs) {
    fn update_el(self, el: &mut El<Ms>) {
        self.0.update_el(el);
        self.1.update_el(el);
    }
}

#[derive(Clone)]
pub struct InputStyle {
    label : (Style, Attrs),
    guidance:  (Style, Attrs),
    label_and_guidance: (Style, Attrs),
    validation_position: (Style, Attrs),
    input_and_validation: (Style, Attrs),
    validation_errors: (Style, Attrs), 
    input:(Style, Attrs),
    invalid_input:(Style, Attrs),
    container: (Style, Attrs),
}


impl<T, F, Ms> InputBuilder<T, F, Ms>
where
    F: FieldNameEnumTrait,
    T: Clone + 'static + Serialize + DeserializeOwned + Validate,
    Ms: 'static,
{
    fn new(form: StateAccess<Form<T>>, field_name: F) -> InputBuilder<T, F, Ms> {

    
        InputBuilder {
            form,
            guidance: None,
            field_name,
            label_pos: Pos::Unset,
            label_name: None,
        

            error_response: std::collections::HashMap::new(),

            validation_errors_pos: Pos::Unset,
            _phantom_type: PhantomData::<Ms>,
        }
    }


    pub fn show_errors_on(mut self, pos: Pos) -> InputBuilder<T, F, Ms> {
        self.validation_errors_pos = pos;
        self
    }

    pub fn show_label(mut self, pos: Pos) -> InputBuilder<T, F, Ms> {
        self.label_pos = pos;
        self
    }

    pub fn label<S: Into<String>>(mut self, name: S) -> InputBuilder<T, F, Ms> {
        self.label_name = Some(name.into());
        self
    }

    pub fn guidance<S: Into<String>>(mut self, name: S) -> InputBuilder<T, F, Ms> {
        self.guidance = Some(name.into());
        self
    }

    pub fn respond_to_error(
        mut self,
        code: &'static str,
        response: &'static str,
    ) -> InputBuilder<T, F, Ms> {
        self.error_response.insert(code, response);
        self
    }

    // View Methods

    pub fn view(self) -> Vec<Node<Ms>> {
        nodes![
            self.label_and_guidance_elem(),
            self.input_and_validation_elem(),
        ]
    }

    pub fn styling(&self) -> InputStyle {
        self.form.get().input_style.clone()
    }

    // Lebel and Guidance Element
    pub fn label_and_guidance_elem(&self) -> Node<Ms> {
        div![
            self.styling().label_and_guidance,
            self.label_elem(),
            div![self.styling().guidance, self.guidance.clone()],
        ]
    }

    /// Label Element

    fn label_elem(&self) -> Node<Ms> {
        label![
            self.styling().label,
            attrs! {At::For=> self.unique_id() },
            self.label_name(),
        ]
    }
    //Input and validation element

    pub fn input_and_validation_elem(&self) -> Node<Ms> {
        div![
            self.styling().input_and_validation,
            self.styling().validation_position,
            self.input_elem(),
            self.validation_errors_elem(),
        ]
    }

    // Validation Errors element
    pub fn validation_errors_elem(&self) -> Node<Ms> {
        div![
            self.styling().validation_errors,
            if !self.is_valid() {
                let validation_error_strings = if !self.validation_error_check() {
                    let errors = self.form.get().errors.unwrap_err();
                    let errors = errors.field_errors();
                    let field_name_string = self.field_name_string();
                    let errors = errors.get(&field_name_string.as_ref()).unwrap();

                    let error_responses = self.error_response.clone();
                    errors
                        .iter()
                        .map(|e| {
                            if let Some(resp) = &error_responses.get(&e.code.as_ref()) {
                                format!("{}", resp.to_string())
                            } else {
                                format!("Invalid {}", self.field_name_string())
                            }
                        })
                        .collect::<Vec<String>>()
                } else {
                    vec![]
                };

                let serde_error = if !self.serde_error_check() {
                    vec![format!("Invalid {}", self.field_name_string())]
                } else {
                    vec![]
                };

                if !serde_error.is_empty() {
                    serde_error
                } else {
                    validation_error_strings
                }
            } else {
                nodes![]
            }
        ]
    }

    pub fn validation_position(&self) -> Pos {
        if self.validation_errors_pos == Pos::Unset {
            self.form.get().validation_errors
        } else {
            self.validation_errors_pos.clone()
        }
    }

 

    // Input Element
    pub fn input_elem(&self) -> Node<Ms> {
        input![
            id!(self.unique_id()),
            self.styling().input,
            if !self.is_valid() {self.styling().invalid_input} else {(style![], class![])},
            self.form.bind(self.field_name.clone())
        ]
    }

    // Utility Methods

    pub fn validation_error_check(&self) -> bool {
        let field_name = self.field_name.name();

        if let Err(errors) = self.form.get().errors {
            errors.field_errors().get(&field_name.as_ref()).is_none()
        } else {
            true
        }
    }

    pub fn serde_error_check(&self) -> bool {
        let field_name = self.field_name.name();
        if let Err(errors) = self.form.get().serde_errors {
            errors.0.get(&field_name).is_none()
        } else {
            true
        }
    }

    pub fn is_valid(&self) -> bool {
        self.validation_error_check() && self.serde_error_check()
    }

    pub fn field_name_string(&self) -> String {
        self.field_name.name()
    }

    pub fn unique_id(&self) -> String {
        format!("{:#?}{}", self.form.id, self.field_name_string())
    }

    pub fn label_name(&self) -> String {
        if let Some(name) = &self.label_name {
            name.clone()
        } else {
            self.field_name_string().clone()
        }
    }
}

pub trait FieldNameEnumTrait: Clone {
    fn name(&self) -> String;
}

pub trait ValidationFormMethods<T>
where
    T: Clone + 'static + Serialize + DeserializeOwned + Validate,
{
    fn validate(&self);
}

impl<T> ValidationFormMethods<T> for StateAccess<Form<T>>
where
    T: Clone + 'static + Serialize + DeserializeOwned + Validate,
{
    fn validate(&self) {
        let form = self.get();

        self.update(|f| f.errors = form.data.validate());
    }
}

pub trait FormNameMethods<T, F>
where
    F: FieldNameEnumTrait,
    T: Clone + 'static + Serialize + DeserializeOwned,
{
    fn input<Ms>(&self, key: F) -> InputBuilder<T, F, Ms>
    where
        Ms: 'static;

    fn bind<Ms:'static>(self, key: F) -> (Attrs, seed::EventHandler<Ms>);
    
}

pub trait StateAccessFormUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms, T> StateAccessFormUpdateEl<El<Ms>> for StateAccess<Form<T>>
where
    T: Clone + 'static + Serialize + DeserializeOwned + Validate,
{
    fn update_el(self, el: &mut El<Ms>) {
        // self.get().update_el(el);
        self.view().update_el(el);
    }
}

pub trait InputBuilderUpdateElTrait<T> {
    fn update_el(self, el: &mut T);
}

impl<T, F, Ms> InputBuilderUpdateElTrait<El<Ms>> for InputBuilder<T, F, Ms>
where
    T: Clone + 'static + Serialize + DeserializeOwned + Validate,
    F: FieldNameEnumTrait,
    Ms: Default,
{
    fn update_el(self, el: &mut El<Ms>) {
        // self.get().update_el(el);
        self.view().update_el(el);
    }
}

pub trait FormGeneralMethods<T>
where
    T: Clone + 'static + Serialize + DeserializeOwned,
{
    fn show_errors(self, pos: Pos) -> Self;

    fn layout(self, layoutstyle: LayoutStyle) -> (Style, Attrs);

    fn view<Ms>(&self) -> Node<Ms>;
}

impl<T> FormGeneralMethods<T> for StateAccess<Form<T>>
where
    T: Serialize + Clone + DeserializeOwned + 'static + Validate,
{
    fn layout(self, layout_style: LayoutStyle) -> (Style, Attrs) {
        self.update( |f| {
            f.layout_style = layout_style.clone();
            f.input_style = match layout_style {
                LayoutStyle::None => input_style_none(),
                LayoutStyle::GridDirect => input_style_tailwind_grid(),
                LayoutStyle::GridTailwind => input_style_tailwind_grid(),
                LayoutStyle::VerticalDirect => input_style_tailwind_grid(),
                LayoutStyle::VerticalTailwind => input_style_tailwind_grid(),
                LayoutStyle::HorizontalDirect => input_style_tailwind_grid(),
                LayoutStyle::HorizontalTailwind => input_style_tailwind_grid(),
                LayoutStyle::CompactDirect => input_style_tailwind_grid(),
                LayoutStyle::CompactTailwind => input_style_tailwind_grid(),
                LayoutStyle::Custom(_input_style) => input_style_tailwind_grid(),
            };
        });
        self.get().input_style.container
    }

    fn show_errors(self, pos: Pos) -> Self {
        self.update(|f| f.validation_errors = pos);
        self
    }

    fn view<Ms>(&self) -> Node<Ms> {
        div![]
    }
}

impl<T, F> FormNameMethods<T, F> for StateAccess<Form<T>>
where
    T: Serialize + Clone + DeserializeOwned + 'static + Validate,
    F: FieldNameEnumTrait,
{
    fn input<Ms>(&self, key: F) -> InputBuilder<T, F, Ms>
    where
        Ms:  'static,
    {
        InputBuilder::new(*self, key)
    }

    fn bind<Ms: 'static>(
        self,
        // val: StateAccess<serde_json::Value>,
        key: F,
    ) -> (Attrs, seed::EventHandler<Ms>)
    {
        let form_data = self.get().clone();

        let key_value = form_data
            .json
            .get(key.name())
            .expect("key to be present")
            .clone();

        let json_val_disp = match key_value.clone() {
            Value::Null => "".to_string(),
            Value::Bool(truth) => format!("{}", truth),
            Value::Number(num) => format!("{}", num),
            Value::String(a_string) => a_string.clone(),
            _ => "".to_string(),
        };
        let key_name = key.name();

        let val_disp = if let Some(field_string) = form_data.field_strings.get(&key_name.clone()) {
            field_string.clone()
        } else {
            json_val_disp
        };

        (
            attrs!(At::Value => val_disp),
            input_ev(Ev::Input, move |ev| {
                let cloned_fd = self.get().clone();
                let mut fd = self.get().clone();
                fd.field_strings.insert(key_name.clone(), ev.clone());

                if let Some(value_for_key) = fd.json.get_mut(key_name.clone()) {
                    let result = match key_value {
                        serde_json::value::Value::Null => Ok(Value::Null),

                        serde_json::value::Value::Bool(_previous) => {
                            if let Ok(parsed_bool) = ev.parse::<bool>() {
                                Ok(Value::Bool(parsed_bool))
                            } else {
                                // set errors here...
                                Err(())
                            }
                        }

                        serde_json::value::Value::Number(_previous) => {
                            if let Ok(parsed_number) = ev.parse::<serde_json::Number>() {
                                if let Err(serde_errors) = &cloned_fd.serde_errors {
                                    let mut serde_errors = serde_errors.clone();
                                    serde_errors.0.remove(&key_name.clone());

                                    if serde_errors.0.len() == 0 {
                                        fd.serde_errors = Ok(());
                                    } else {
                                        fd.serde_errors = Err(serde_errors);
                                    }
                                    // remove errror
                                }

                                Ok(Value::Number(parsed_number))
                            } else {
                                // set errors here
                                match &cloned_fd.serde_errors {
                                    Ok(_) => {
                                        let mut new_errors = std::collections::HashMap::new();
                                        new_errors.insert(key_name.clone(), true);
                                        fd.serde_errors = Err(SerdeErrors(new_errors))
                                    }

                                    Err(serde_errors) => {
                                        let mut serde_errors = serde_errors.clone();
                                        serde_errors.0.insert(key_name.clone(), true);
                                        fd.serde_errors = Err(serde_errors);
                                        // save error
                                    }
                                }

                                Err(())
                            }
                        }

                        serde_json::value::Value::String(_) => Ok(Value::String(ev)),

                        Value::Object(object) => Ok(Value::Object(object.clone())),

                        Value::Array(arr) => Ok(Value::Array(arr.clone())),
                    };

                    if let Ok(value) = result {
                        *value_for_key = value;
                        let cloned_fd = fd.clone();
                        match serde_json::from_value(cloned_fd.json) {
                            Ok(serde_parsed_form_data) => {
                                fd.data = serde_parsed_form_data;

                                if let Err(serde_errors) = &cloned_fd.serde_errors {
                                    let mut serde_errors = serde_errors.clone();
                                    serde_errors.0.remove(&key_name.clone());

                                    if serde_errors.0.len() == 0 {
                                        fd.serde_errors = Ok(());
                                    } else {
                                        fd.serde_errors = Err(serde_errors);
                                    }
                                    // remove errror
                                }
                            }
                            Err(error) => {
                                // set errors
                                log!(error);

                                match &cloned_fd.serde_errors {
                                    Ok(_) => {
                                        let mut new_errors = std::collections::HashMap::new();
                                        new_errors.insert(key_name.clone(), true);
                                        fd.serde_errors = Err(SerdeErrors(new_errors))
                                    }
                                    Err(serde_errors) => {
                                        let mut serde_errors = serde_errors.clone();
                                        serde_errors.0.insert(key_name.clone(), true);
                                        fd.serde_errors = Err(serde_errors);
                                        // save error
                                    }
                                }

                                // log!("there is an error");
                            }
                        }
                    }
                }

                self.set(fd.clone());
                log!("set it!");
            }),
        )
    }
}

#[topo::nested]
pub fn use_form<T, F: Fn() -> T>(form_func: F) -> StateAccess<Form<T>>
where
    T: Serialize + Clone + DeserializeOwned + 'static,
{
    let form = use_state(|| Form::new(form_func()));
    form
}

// fn input_style_direct_grid() -> InputStyle{
//     let shared_vertical_padding = "0.25rem";
//     let shared_horizontal_padding = "0.25rem";

//     // let minor_height = "h-6";
//     // let major_height = "h-10";

//     InputStyle {
//         label: (
//             style! {
//                 St::Width=>"auto",
//                 St::TextAlign=> "right",
//                 // St::ContentModel => "box-border",
//                 St::PaddingTop => "0.25em",
//                 St::PaddingLeft => "0.25em",
//                 St::PaddingBottom => "0.25em",
//                 St::Height => "4em",
//             },
//             class![],
//         ),
//         input:  (
//             style! {
//                 St::Border => "solid 0.5em #333",
//                 St::Width => "100%",
//                 St::Outline=> "none",
//                 St::Padding => "0.5em",
//             },
//             class!["border-solid border-2 border-gray-400 focus:border-yellow-500 w-full outline-none", "p-2", major_height],
//         ),
//         guidance:   (
//                 style! {},
//                 class!["w-auto text-sm text-right text-gray-600", minor_height],
//             ),
//         label_and_guidance:(
//             style! {},
//             class!["flex flex-col col-start-1 col-end-1"],
//         ),
//         validation_position:(
//             style! {},
//             class!["flex flex-col"],
//         ),
//         input_and_validation:  (
//             style! {},
//             class![
//                    "col-start-3",
//                     "col-end-4",
//                     // tailwind.config.js
//                     //
//                     // module.exports = {
//                     //     theme: {
//                     //       extend: {
//                     //         gridTemplateColumns: {
//                     //           // Simple 16 column grid
//                     //   +       'form': ' 1fr 1em 2fr',
//                     //         }
//                     //       }
//                     //     }
//                     //   }
//                     ],
//         ),
//         validation_errors:   (
//             style! {
//             },
//             class!["w-auto text-sm text-right text-red-600", minor_height],
//         ),
 
//         invalid_input: ( style!{},class![
//             "border-solid border-2 border-red-400"
//         ]),

//         container   :(
//             style! {St::GridTemplateColumns => "1fr 0.5em 2fr"},
//             class!["grid grid-flow-row-dense row-gap-1 col-gap-1 p-4"],
//         ),
//     }
// }



fn input_style_tailwind_grid() -> InputStyle{
    let minor_height = "h-6";
    let major_height = "h-10";
    InputStyle {
        label: (
            style! {},
            class![
                "w-auto",
                "text-right",
                "box-border"
                , "pt-2 pb-2 pl-2",
                major_height
            ],
        ),
        input:  (
            style! {},
            class!["border-solid border-2 border-gray-400 focus:border-yellow-500 w-full outline-none", "p-2", major_height],
        ),
        guidance:   (
                style! {},
                class!["w-auto text-sm text-right text-gray-600", minor_height],
            ),
        label_and_guidance:(
            style! {},
            class!["flex flex-col col-start-1 col-end-1"],
        ),
        validation_position:(
            style! {},
            class!["flex flex-col"],
        ),
        input_and_validation:  (
            style! {},
            class![
                   "col-start-3",
                    "col-end-4",
                    // tailwind.config.js
                    //
                    // module.exports = {
                    //     theme: {
                    //       extend: {
                    //         gridTemplateColumns: {
                    //           // Simple 16 column grid
                    //   +       'form': ' 1fr 1em 2fr',
                    //         }
                    //       }
                    //     }
                    //   }
                    ],
        ),
        validation_errors:   (
            style! {
            },
            class!["w-auto text-sm text-right text-red-600", minor_height],
        ),
 
        invalid_input: ( style!{},class![
            "border-solid border-2 border-red-400"
        ]),

        container   :(
            style! {St::GridTemplateColumns => "1fr 0.5em 2fr"},
            class!["grid grid-flow-row-dense row-gap-1 col-gap-1 p-4"],
        ),
    }
}


fn input_style_none() -> InputStyle {
    InputStyle {
        label : (style!{}, class![]),
        guidance:   (style!{}, class![]),
        label_and_guidance: (style!{}, class![]),
        invalid_input: (style!{}, class![]),
        validation_position: (style!{}, class![]),
        input_and_validation: (style!{}, class![]),
        validation_errors:  (style!{}, class![]),
        input: (style!{}, class![]),
        container:  (style!{}, class![]),
    }
}




