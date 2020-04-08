use std::panic::Location;
use wasm_bindgen::JsCast;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use harsh::{Harsh,HarshBuilder};
use std::cell::RefCell;
use std::collections::HashSet;
use std::{collections::HashMap};
use seed::{*, prelude::*};

use tw_seed_hooks::{generate_tw_sizes, generate_tw_colors};

use lazy_static::*;

lazy_static! {
    pub static ref S: Style = Style {
        ..Style::default()
    };
}

thread_local! {
    pub static STYLES_USED : RefCell<HashSet<u64>> = RefCell::new(HashSet::<u64>::new());
    pub static HASH_IDS_GENERATOR: RefCell<Harsh> = RefCell::new(HarshBuilder::new().init().unwrap());
}

fn short_uniq_id(id:u64) -> String {
    HASH_IDS_GENERATOR.with(|h| h.borrow().encode(&[id]).unwrap())
}

pub fn use_theme<Ms, F>(theme: Theme, content: F) -> Node<Ms>
    where
        F: Fn()-> Node<Ms>
{
    illicit::child_env!( Theme => theme).enter(content)
}

#[derive(Clone)]
pub struct Rule { 
    theme: Theme,
    theme_key: ThemeKey,
    value: String,
    property: Property,
}
#[derive(Clone)]
pub enum ThemeKey {
    None,
    Scale(usize),
    BreakpointScales(Vec<usize>),
    Alias(String),
}


impl Rule {
    fn style_look_up(&self) -> &ThemeLookUp {
        use Property::*;
        match &self.property {
            Color|
            BackgroundColor|
            BorderColor|
            BorderTopColor|
            BorderRightColor|
            BorderBottomColor|
            BorderLeftColor|
            OutlineColor|
            Fill => &self.theme.colors,
            
            BorderWidth|
            BorderTopWidth|
            BorderRightWidth|
            BorderBottomWidth|
            BorderLeftWidth
            => &self.theme.border_widths,

            BorderStyle|
            BorderTopStyle|
            BorderRightStyle|
            BorderBottomStyle|
            BorderLeftStyle
            => &self.theme.border_styles,
            
            BorderRadius|
            BorderTopLeftRadius|
            BorderTopRightRadius|
            BorderBottomRightRadius|
            BorderBottomLeftRadius
            => &self.theme.radii,

            FontFamily => &self.theme.fonts,
            FontSize => &self.theme.font_sizes,
            FontWeight => &self.theme.font_weights,
            Transition => &self.theme.transitions,
            ZIndex => &self.theme.z_indices,
            LetterSpacing => &self.theme.letter_spacings,
            LineHeight=> &self.theme.line_heights,

            GridGap|
            GridColumnGap|
            GridRowGap|
            Padding|
            PaddingLeft|PaddingRight|PaddingTop|PaddingBottom|
            Margin|
            MarginLeft|MarginRight|MarginTop|MarginBottom
            => &self.theme.spaces,

            _ => &self.theme.colors,
        }
      
    }

    fn render(&self) -> String{


        let style_look_up = self.style_look_up();

        let value = match &self.theme_key {
            ThemeKey::Alias(alias) => {
                if let Some(val) = style_look_up.get_alias(alias){

                    val.clone()
                } else {self.value.clone()}
            }
            ThemeKey::Scale(idx) => {
                if let Some(val) = style_look_up.get_alias_from_scale(*idx){
                    val.clone()
                } else {self.value.clone()}
            }
            ThemeKey::BreakpointScales(_idxs) => {
                todo!()
            }
            ThemeKey::None => self.value.clone()
        };

        format!("    {}: {};\n", self.property.render(), value)
    }
}

#[derive(PartialEq, Clone)]
pub enum Property {
    Color,
    BackgroundColor,
    
    Padding,
    PaddingRight,
    PaddingLeft,
    PaddingTop,
    PaddingBottom,
    
    Margin,
    MarginRight,
    MarginLeft,
    MarginTop,
    MarginBottom,
           
    Border,
    BorderRight,
    BorderLeft,
    BorderTop,
    BorderBottom,
    
    BorderColor,
    BorderRightColor,
    BorderLeftColor,
    BorderTopColor,
    BorderBottomColor,
    OutlineColor,
    Fill,
  
    BorderWidth,
    BorderRightWidth,
    BorderLeftWidth,
    BorderTopWidth,
    BorderBottomWidth,
    FontFamily,
    FontSize ,
    FontWeight,
    Transition,
    ZIndex ,
    LetterSpacing,
    LineHeight,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,

    GridGap,
    GridColumnGap,
    GridRowGap,

    BorderStyle,
    BorderRightStyle,
    BorderLeftStyle,
    BorderTopStyle,
    BorderBottomStyle,
   
    Width,
    Height,
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,
    BorderRadius,

    Display,
    

    Custom,

    AnimDuration,
    AnimDelay,
    AnimIterationCount,
    AnimDirection,
    AnimTimingFunction,
    AnimFillMode,
    AnimPlayState
    
}

impl Property {
    fn render(&self) -> String {
        use Property::*;
         match self {
            Color => "color",
            BackgroundColor => "background-color",

            Padding => "padding",
            PaddingRight => "padding-right",
            PaddingLeft => "padding-left",
            PaddingTop => "padding-top",
            PaddingBottom => "padding-bottom",


            Margin => "margin",
            MarginRight => "margin-right",
            MarginLeft => "margin-left",
            MarginTop => "margin-top",
            MarginBottom => "margin-bottom",


            Border => "border",
            BorderRight => "border-right",
            BorderLeft => "border-left",
            BorderTop => "border-top",
            BorderBottom => "border-bottom",

          
            BorderColor => "border-color",
            BorderRightColor => "border-right-color",
            BorderLeftColor =>"border-left-color",
            BorderTopColor => "border-top-color",
            BorderBottomColor => "border-bottom-color",

            OutlineColor => "outline-color",
            Fill=> "fill",

            BorderWidth => "border-width",
            BorderRightWidth => "border-right-width",
            BorderLeftWidth => "border-left-width",
            BorderTopWidth => "border-top-width",
            BorderBottomWidth => "border-bottom-width",

            BorderStyle => "border-style",
            BorderRightStyle => "border-right-style",
            BorderLeftStyle => "border-left-style",
            BorderTopStyle => "border-top-style",
            BorderBottomStyle => "border-bottom-style",

            Width => "width",
            Height => "height",
            MinWidth => "min-width",
            MaxWidth  => "max-width",
            MinHeight => "min-height",
            MaxHeight => "max-height",

            Display => "display",
            Transition => "transition",
            BorderRadius => "border-radius",

            BorderTopLeftRadius => "border-top-left-radius",
            BorderTopRightRadius => "border-top-right-radius",
            BorderBottomLeftRadius => "border-bottom-left-radius",
            BorderBottomRightRadius => "border-bottom-right-radius",

            AnimDuration => "animation-duration",
            AnimDelay => "animation-delay",
            AnimIterationCount => "animation-iteration-count",
            AnimDirection => "animation-direction",
            AnimTimingFunction => "animation-timing-function",
            AnimFillMode => "animation-fill-mode",
            AnimPlayState => "animation-play-state",

            FontFamily => "font-family",
            FontSize  => "font-size",
            FontWeight => "font-weight",
            ZIndex  => "z-index",
            
            LineHeight => "line-height",

            GridGap => "grid-gap",
            GridColumnGap => "grid-column-gap",
            GridRowGap => "grid-row-gap",
            LetterSpacing => "letter-spacing",
            Custom => "",
         }.to_string()
    }
}

#[derive(Clone)]
pub struct Style {
    theme : Theme,
    rules: Vec<Rule>,
    updated_at: Vec<String>,
    pseudo: Pseudo,
    media: Option<String>,
    name: String,
    keyframes: Keyframes,
}

fn get_theme() -> Theme{   
    if let  Some(theme) = illicit::Env::get::<Theme>(){
        theme.clone()
    } else {
        Theme::default()
    }
}

impl Default for Style {

#[track_caller]
 fn default() -> Self {
    Style{
        theme : get_theme(),
        pseudo: Pseudo::None,
        updated_at: vec![format!("{}", Location::caller())],
        rules: vec![],
        name: "".to_string(),
        media: None,
        keyframes: Keyframes::default(),
        }
    }
}

pub trait OverloadedProperty<T>
{
    fn overloaded_property(&self, property: Property, val: T) -> Style;
    
}


impl OverloadedProperty<&str> for Style {
    fn overloaded_property(&self,property: Property , val:&str ) -> Style {
        { let mut new_self = self.clone(); new_self.add_rule(property, val, ThemeKey::None); new_self }
    }
}

impl OverloadedProperty<(&str,&str)> for Style {
    fn overloaded_property(&self, property: Property, val: (&str,&str)) -> Style{ let mut new_self = self.clone(); new_self.add_rule(property, val.1, ThemeKey::Alias(val.0.to_string())); new_self }
}
impl OverloadedProperty<(usize,&str)> for Style {
    fn overloaded_property(&self, property: Property, val: (usize,&str)) -> Style{ let mut new_self = self.clone(); new_self.add_rule(property, val.1, ThemeKey::Scale(val.0)); new_self }
}

impl OverloadedProperty<(&[usize],&str)> for Style {
    fn overloaded_property(&self, property: Property, val: (&[usize],&str)) -> Style{ let mut new_self = self.clone(); new_self.add_rule(property, val.1, ThemeKey::BreakpointScales(val.0.to_vec())); new_self }
}




use Property::*;

impl Style {


    #[track_caller] pub fn css(&self, val:&str) -> Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller())); new_style.add_rule(Custom, val, ThemeKey::None);new_style }
    #[track_caller] pub fn margin<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Margin, val)}
    #[track_caller] pub fn margin_right<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginRight, val)}
    #[track_caller] pub fn margin_left<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginLeft, val)}
    #[track_caller] pub fn margin_top<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginTop, val)}
    #[track_caller] pub fn margin_bottom<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginBottom, val)}

    #[track_caller] pub fn m<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Margin, val)}
    #[track_caller] pub fn mr<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginRight, val)}
    #[track_caller] pub fn ml<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginLeft, val)}
    #[track_caller] pub fn mt<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginTop, val)}
    #[track_caller] pub fn mb<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(MarginBottom, val)}
    #[track_caller] pub fn mx<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone{  let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(MarginLeft, val.clone()); new_self.overloaded_property(MarginRight,val)}
    #[track_caller] pub fn my<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone {  let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(MarginTop, val.clone()); new_self.overloaded_property(MarginBottom,val)}
   
    #[track_caller] pub fn border_radius<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRadius, val)}
    #[track_caller] pub fn radius<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRadius, val)}
    #[track_caller] pub fn fill<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Fill, val)}
    #[track_caller] pub fn outline_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(OutlineColor, val)}
    #[track_caller] pub fn color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Color, val)}
    #[track_caller] pub fn background_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BackgroundColor, val)}
    #[track_caller] pub fn bg_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BackgroundColor, val)}

    #[track_caller] pub fn padding<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Padding, val)}
    #[track_caller] pub fn padding_right<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingRight, val)}
    #[track_caller] pub fn padding_left<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingLeft, val)}
    #[track_caller] pub fn padding_top<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingTop, val)}
    #[track_caller] pub fn padding_bottom<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingBottom, val)}
    #[track_caller] pub fn padding_x<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(PaddingLeft, val.clone()); new_self.overloaded_property(PaddingRight,val)}
    #[track_caller] pub fn padding_y<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(PaddingTop, val.clone()); new_self.overloaded_property(PaddingBottom,val)}

    #[track_caller] pub fn p<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Padding, val)}
    #[track_caller] pub fn pr<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingRight, val)}
    #[track_caller] pub fn pl<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingLeft, val)}
    #[track_caller] pub fn pt<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingTop, val)}
    #[track_caller] pub fn pb<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(PaddingBottom, val)}
    #[track_caller] pub fn px<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(PaddingLeft, val.clone()); new_self.overloaded_property(PaddingRight,val)}
    #[track_caller] pub fn py<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(PaddingTop, val.clone()); new_self.overloaded_property(PaddingBottom,val)}

    #[track_caller] pub fn border<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Border, val)}
    #[track_caller] pub fn border_right<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRight, val)}
    #[track_caller] pub fn border_left<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeft, val)}
    #[track_caller] pub fn border_top<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTop, val)}
    #[track_caller] pub fn border_bottom<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottom, val)}

    #[track_caller] pub fn border_r<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRight, val)}
    #[track_caller] pub fn border_l<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeft, val)}
    #[track_caller] pub fn border_t<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTop, val)}
    #[track_caller] pub fn border_b<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottom, val)}
    #[track_caller] pub fn border_x<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(BorderLeft, val.clone()); new_self.overloaded_property(BorderRight,val)}
    #[track_caller] pub fn border_y<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>, T:Clone { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.overloaded_property(BorderTop, val.clone()); new_self.overloaded_property(BorderBottom,val)}

  
    #[track_caller] pub fn border_right_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRightColor, val)}
    #[track_caller] pub fn border_left_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeftColor, val)}
    #[track_caller] pub fn border_top_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTopColor, val)}
    #[track_caller] pub fn border_bottom_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottomColor, val)}

    #[track_caller] pub fn border_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderColor, val)}
    #[track_caller] pub fn border_r_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRightColor, val)}
    #[track_caller] pub fn border_l_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeftColor, val)}
    #[track_caller] pub fn border_t_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTopColor, val)}
    #[track_caller] pub fn border_b_color<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottomColor, val)}

    #[track_caller] pub fn inline_block<'a>(&self) -> Style where Self:OverloadedProperty<&'a str> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Display, "inline-block")}
    #[track_caller] pub fn display<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Display, val)}
   
    #[track_caller] pub fn transition<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Transition, val)}

    #[track_caller] pub fn border_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderWidth, val)}
    #[track_caller] pub fn border_right_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRightWidth, val)}
    #[track_caller] pub fn border_left_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeftWidth, val)}
    #[track_caller] pub fn border_top_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTopWidth, val)}
    #[track_caller] pub fn border_bottom_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottomWidth, val)}

    #[track_caller] pub fn border_r_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRightWidth, val)}
    #[track_caller] pub fn border_l_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeftWidth, val)}
    #[track_caller] pub fn border_t_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTopWidth, val)}
    #[track_caller] pub fn border_b_width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottomWidth, val)}

    #[track_caller] pub fn border_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderStyle, val)}
    #[track_caller] pub fn border_right_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRightStyle, val)}
    #[track_caller] pub fn border_left_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeftStyle, val)}
    #[track_caller] pub fn border_top_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTopStyle, val)}
    #[track_caller] pub fn border_bottom_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottomStyle, val)}

    #[track_caller] pub fn b_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderStyle, val)}
    #[track_caller] pub fn br_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderRightStyle, val)}
    #[track_caller] pub fn bl_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderLeftStyle, val)}
    #[track_caller] pub fn bt_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderTopStyle, val)}
    #[track_caller] pub fn bb_style<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(BorderBottomStyle, val)}

    #[track_caller] pub fn font_size<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(FontSize, val)}
    #[track_caller] pub fn font_family<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(FontFamily, val)}



    #[track_caller] pub fn width<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Width, val)}
    #[track_caller] pub fn w<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Width, val)}
    #[track_caller] pub fn height<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Height, val)}
    #[track_caller] pub fn h<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(Height, val)}
    #[track_caller] pub fn animation_duration<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(AnimDuration, val)}
    #[track_caller] pub fn duration<T>(&self, val:T) -> Style where Self:OverloadedProperty<T> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.overloaded_property(AnimDuration, val)}
    // tailwind methods
    
    generate_tw_sizes!(
    [
        "p",
        "pl",
        "pr",
        "pt",
        "pb",
        "px",
        "py",
        "m",
        "ml",
        "mr",
        "mt",
        "mb",
        "mx",
        "my",
        "w",
        "h",
        "border",
        "border_l",
        "border_r",
        "border_t",
        "border_b",
        "border_x",
        "border_y"
    ],  [
       (0, "0"),
       (1, "0.25rem"),
       (2, "0.5rem"),
       (3, "0.75rem"),
       (4, "1rem"),
       (5, "1.25rem"),
       (6, "1.5rem"),
       (8, "2rem"),
       (10, "2.5rem"),
       (12, "3rem"),
       (16, "4rem"),
       (20, "5rem"),
       (24, "6rem"),
       (32, "8rem"),
       (40, "10rem"),
       (48, "12rem"),
       (56, "14rem"),
       (64, "16rem"),]
);


generate_tw_colors!( 
[
    ("text","color"),
    ("bg","bg_color"),
    ("border","border_color"),
    ("border_l","border_l_color"),
    ("border_r","border_r_color"),
    ("border_t","border_t_color"),
    ("border_b","border_b_color"),
    ("fill","fill"),
    ("outline","outline_color")
]
, [
("white", "#FFF"),
("black", "#000"),
("red", "#F00"),
("green", "#0F0"),
("blue", "#00F"),
("gray_100" ,"#F7FAFC"),
("gray_200" ,"#EDF2F7"),
("gray_300" ,"#E2E8F0"),
("gray_400" ,"#CBD5E0"),
("gray_500" ,"#A0AEC0"),
("gray_600" ,"#718096"),
("gray_700" ,"#4A5568"),
("gray_800" ,"#2D3748"),
("gray_900" ,"#1A202C"),
("red_100" ,"#FFF5F5"),
("red_200" ,"#FED7D7"),
("red_300" ,"#FEB2B2"),
("red_400" ,"#FC8181"),
("red_500" ,"#F56565"),
("red_600" ,"#E53E3E"),
("red_700" ,"#C53030"),
("red_800" ,"#9B2C2C"),
("red_900" ,"#742A2A"),
("orange_100" ,"#FFFAF0"),
("orange_200" ,"#FEEBC8"),
("orange_300" ,"#FBD38D"),
("orange_400" ,"#F6AD55"),
("orange_500" ,"#ED8936"),
("orange_600" ,"#DD6B20"),
("orange_700" ,"#C05621"),
("orange_800" ,"#9C4221"),
("orange_900" ,"#7B341E"),
("yellow_100" ,"#FFFFF0"),
("yellow_200" ,"#FEFCBF"),
("yellow_300" ,"#FAF089"),
("yellow_400" ,"#F6E05E"),
("yellow_500" ,"#ECC94B"),
("yellow_600" ,"#D69E2E"),
("yellow_700" ,"#B7791F"),
("yellow_800" ,"#975A16"),
("yellow_900" ,"#744210"),
("green_100" ,"#F0FFF4"),
("green_200" ,"#C6F6D5"),
("green_300" ,"#9AE6B4"),
("green_400" ,"#68D391"),
("green_500" ,"#48BB78"),
("green_600" ,"#38A169"),
("green_700" ,"#2F855A"),
("green_800" ,"#276749"),
("green_900" ,"#22543D"),
("teal_100" ,"#E6FFFA"),
("teal_200" ,"#B2F5EA"),
("teal_300" ,"#81E6D9"),
("teal_400" ,"#4FD1C5"),
("teal_500" ,"#38B2AC"),
("teal_600" ,"#319795"),
("teal_700" ,"#2C7A7B"),
("teal_800" ,"#285E61"),
("teal_900" ,"#234E52"),
("blue_100" ,"#EBF8FF"),
("blue_200" ,"#BEE3F8"),
("blue_300" ,"#90CDF4"),
("blue_400" ,"#63B3ED"),
("blue_500" ,"#4299E1"),
("blue_600" ,"#3182CE"),
("blue_700" ,"#2B6CB0"),
("blue_800" ,"#2C5282"),
("blue_900" ,"#2A4365"),
("indigo_100" ,"#EBF4FF"),
("indigo_200" ,"#C3DAFE"),
("indigo_300" ,"#A3BFFA"),
("indigo_400" ,"#7F9CF5"),
("indigo_500" ,"#667EEA"),
("indigo_600" ,"#5A67D8"),
("indigo_700" ,"#4C51BF"),
("indigo_800" ,"#434190"),
("indigo_900" ,"#3C366B"),
("purple_100" ,"#FAF5FF"),
("purple_200" ,"#E9D8FD"),
("purple_300" ,"#D6BCFA"),
("purple_400" ,"#B794F4"),
("purple_500" ,"#9F7AEA"),
("purple_600" ,"#805AD5"),
("purple_700" ,"#6B46C1"),
("purple_800" ,"#553C9A"),
("purple_900" ,"#44337A"),
("pink_100" ,"#FFF5F7"),
("pink_200" ,"#FED7E2"),
("pink_300" ,"#FBB6CE"),
("pink_400" ,"#F687B3"),
("pink_500" ,"#ED64A6"),
("pink_600" ,"#D53F8C"),
("pink_700" ,"#B83280"),
("pink_800" ,"#97266D"),
("pink_900", "#702459")]);
    fn add_rule(&mut self, property: Property, value: &str, theme_key: ThemeKey){
        self.rules.push(
            Rule {
                theme: get_theme(),
                theme_key,
                value: value.to_string(),
                property,
            }
        );
    }

    #[track_caller]
    pub fn hover(&self) -> Style {
        let mut new_style = self.clone(); 
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.pseudo = Pseudo::Hover;
        new_style
    }

    #[track_caller]
    pub fn name(&self, name:&str) -> Style {
        let mut new_style = self.clone(); 
        new_style.updated_at.push(format!("{}", Location::caller()));

        new_style.name = name.to_string();

        new_style    
    }

    #[track_caller]
    pub fn keyframe(&self, key:i32 , to:Style) -> Style {
        let mut new_style = self.clone(); 
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.keyframes.frames.push((key, Box::new(to)));
        new_style    
    }
      


    #[track_caller]
    pub fn media(&self, val:&str) -> Style {
        let mut new_style = self.clone(); 
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.media = Some(val.to_string());
        new_style
    }


    fn render(&self) -> String {
        let mut style = "".to_string();

        style.push_str( &format!("    /* Defined at {} */\n", self.updated_at.last().unwrap()));

        for rule in &self.rules {
            match rule.property {
                Custom => style.push_str(&format!("{}\n",rule.value)),
                _ => style.push_str( &rule.render() ),
            }
            
        }

        style
    }


}

#[derive(Clone)]
pub enum Pseudo {
    None,
    Active,
    Checked,
    Disabled,
    Empty,
    Enabled,
    FirstChild,
    FirstOfType,
    Focus,
    Hover,
    InRange,
    Invalid,
    Lang(String),
    LastChild,
    LastOfType,
    Link,
    Not(String),
    NthChild(usize),
    NthLastChild(usize),
    NthLastOfType(usize),
    NthOfType(usize),
    OnlyOfType,
    OnlyChild,
    Optional,
    OutOfRange,
    ReadOnly,
    ReadWrite,
    Required,
    Root,
    Target,
    Valid,
    Visited,
}

impl Pseudo {
    fn render(&self)->String{
        use Pseudo::*;
        match self {
            None => "",
            Active=> ":active",
            Checked => ":checked",
            Disabled=> ":disabled",
            Empty=> ":empty",
            Enabled=> ":enabled",
            FirstChild=> ":first-child",
            FirstOfType=> ":first-of-type",
            Focus=> ":focus",
            Hover=> ":hover",
            InRange=> ":in-range",
            Invalid=> ":invalid",
            Lang(_xxx)=> ":langXXX",
            LastChild=> ":last-child",
            LastOfType=> ":last-of-type",
            Link=> ":link",
            Not(_xxx)=> ":notXXX",
            NthChild(_xxx)=> ":nth-childXXX",
            NthLastChild(_xxx)=> ":nth-last-childXXX",
            NthLastOfType(_xxx)=> ":nth-last-of-typeXXX",
            NthOfType(_xxx)=> ":nth-of-typeXXX",
            OnlyOfType=> ":only-of-type",
            OnlyChild=> ":only-child",
            Optional=> ":optional",
            OutOfRange=> ":out-of-range",
            ReadOnly=> ":read-only",
            ReadWrite=> ":read-write",
            Required=> ":required",
            Root=> ":root",
            Target=> ":target",
            Valid=> ":valid",
            Visited=> ":visited",
        }.to_string()
    }
}


#[derive(Default,Clone)]
pub  struct Keyframes {
    frames: Vec< (i32, Box<Style>)>,
}

impl Keyframes {

    fn render(&self) -> String {
        let mut rendered = "".to_string();

        for frame in &self.frames{
            rendered.push_str( &format!("{}% {{\n{}}}" , frame.0, frame.1.render()));
        }
        rendered
    }

}

pub trait LocalUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms> LocalUpdateEl<El<Ms>> for Style
{
    fn update_el(self, el: &mut El<Ms>) {

        let rendered_css = self.render();
       
       let variant_hash = hash_64(&rendered_css, &self.updated_at);
              
       let class_name = format!("sc-{}",add_css_to_head(&rendered_css, variant_hash, &self));
       
        C![class_name].update_el(el);
    }
}

impl<Ms> LocalUpdateEl<El<Ms>> for &[Style]
{
    fn update_el(self, el: &mut El<Ms>) {

        let vec_of_rendered_css = self.iter().map(|s| s.render()).collect::<Vec<String>>();

        let mut s = DefaultHasher::new();
        vec_of_rendered_css.hash(&mut s);
        let variant_hash = s.finish();
        
        let css_aleady_created = STYLES_USED.with(|css_set_ref| css_set_ref.borrow().contains(&variant_hash));
       
        
        let mut name = "".to_string();
        for (_, style) in vec_of_rendered_css.iter().zip(self.iter()) {
            name.push_str(&style.name);
        }
        
        if !css_aleady_created {
        for (rendered_css, style) in vec_of_rendered_css.iter().zip(self.iter()) {


                add_css_to_head_unchecked(&rendered_css, variant_hash, style,&name );
            }
    
        }
        let short_hash = format!("{}{}", name, short_uniq_id(variant_hash));
        let class_name = format!("sc-{}",short_hash);
       
        C![class_name].update_el(el);
    }
}

fn hash_64<T: AsRef<str> + Hash>(css : &str, locations: &[T]) -> u64 {
    let mut s = DefaultHasher::new();
    (css,locations).hash(&mut s);
    s.finish()
}




fn add_css_to_head(css: &str, variant_hash: u64, style: &Style ) -> String{

    let css_aleady_created = STYLES_USED.with(|css_set_ref| css_set_ref.borrow().contains(&variant_hash));
    let short_hash = format!("{}{}", style.name, short_uniq_id(variant_hash));

    if !css_aleady_created {
        add_css_to_head_unchecked(css,variant_hash, style, &style.name);
    }
    short_hash
}



fn add_css_to_head_unchecked(css: &str, variant_hash: u64 , style: &Style, name: &str) -> String{


    let short_hash = format!("{}{}", name, short_uniq_id(variant_hash));

    let head_elem = document().get_elements_by_tag_name("head").item(0).unwrap();

    let css = if !style.keyframes.frames.is_empty() {
        format!("{}    animation-name: anim-{};\n",css,short_hash)
    } else {
        css.to_string()
    };

    let mut full_css = if let Some(media) = &style.media {
        format!("{}{{\n.sc-{}{}{{\n{}}}}}\n",media, short_hash, style.pseudo.render() , css)
    }  else {
        format!("\n.sc-{}{}{{\n{}}}\n", short_hash, style.pseudo.render() , css)
    };

    if !style.keyframes.frames.is_empty() {
        full_css.push_str(&format!("\n\n@keyframes anim-{}{{ \n  {}  \n}}\n", short_hash, style.keyframes.render() ));
    }
    
    if let Some( style_elem)  = head_elem.get_elements_by_tag_name("style").item(0){
    let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
    let existing_style_content = style_elem.inner_html();

    let new_style_content = format!("{}\n{}", existing_style_content, full_css);
    style_elem.set_inner_html(&new_style_content);

    } else {
    let style_elem  = document().create_element("style").unwrap().dyn_into::<web_sys::HtmlStyleElement>().unwrap();
    style_elem.set_inner_html(&full_css);
    let _ = head_elem.append_child( &style_elem );
    }
    STYLES_USED.with(|css_set_ref| css_set_ref.borrow_mut().insert(variant_hash));

    short_hash
}

#[derive(Clone, Debug, Default)]
pub struct ThemeLookUp {
    scale: Vec<String>,
    aliases: HashMap<String, String>
}

impl ThemeLookUp {
    fn alias<S:Into<String>>(&mut self, key : S, value: S){
        let key = key.into();
        let value = value.into();
        self.aliases.insert(key, value);
    }

    fn get_alias<S: Into<String>>(&self, key : S) -> Option<String> {
        self.aliases.get(&key.into()).cloned()
    }


    fn get_alias_from_scale(&self, idx: usize) -> Option<String> {
        self.scale.get(idx).map(|v| { v.clone()})
    }

}

#[derive(Debug, Clone, Default)]
pub struct Theme {
   pub colors: ThemeLookUp,
   pub spaces: ThemeLookUp,
   pub font_sizes: ThemeLookUp,
   pub fonts: ThemeLookUp,
   pub font_weights: ThemeLookUp,
   pub line_heights: ThemeLookUp,
   pub letter_spacings: ThemeLookUp,
   pub sizes: ThemeLookUp,
   pub borders: ThemeLookUp,
   pub border_widths: ThemeLookUp,
   pub border_styles: ThemeLookUp,
   pub radii: ThemeLookUp,
   pub shadows: ThemeLookUp,
   pub z_indices: ThemeLookUp,
   pub transitions: ThemeLookUp,
}

impl Theme {
   pub fn new() -> Theme {
        Theme::default()
    }

   pub fn color(&self, alias:&str, value:&str) -> Theme {
        let  mut new_self = self.clone();
        new_self.colors.alias(alias,value);
        new_self
    }

   pub fn color_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.colors.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }


   pub fn space(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.spaces.alias(alias,value);
        new_self
    }

   pub fn space_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let  mut new_self = self.clone();
        new_self.spaces.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

   pub fn font_size(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.font_sizes.alias(alias,value);
        new_self
    }

   pub fn font_size_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.font_sizes.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }
    
    pub fn font(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.fonts.alias(alias,value);
        new_self
    }

    pub fn font_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.fonts.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }
    
    pub fn font_weight(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.font_weights.alias(alias,value);
        new_self
    }

    pub fn font_weight_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.font_weights.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

    pub fn size(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.sizes.alias(alias,value);
        new_self
    }

    pub fn size_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.sizes.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }


    pub fn line_height(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.line_heights.alias(alias,value);
        new_self
    }

    pub fn line_heights_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.line_heights.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }


    pub fn letter_spacing(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.letter_spacings.alias(alias,value);
        new_self
    }

    pub fn letter_spacing_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.letter_spacings.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }


    pub fn border(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.borders.alias(alias,value);
        new_self
    }

    pub fn border_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.borders.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }


    pub fn border_width(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.border_widths.alias(alias,value);
        new_self
    }

    pub fn border_width_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.border_widths.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

    pub fn border_style(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.border_styles.alias(alias,value);
        new_self
    }

    pub fn border_style_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.border_styles.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

    pub fn shadow(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.shadows.alias(alias,value);
        new_self
    }

    pub fn shadow_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.shadows.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

    pub fn z_index(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.z_indices.alias(alias,value);
        new_self
    }

    pub fn z_index_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.z_indices.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

    pub fn transition(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.transitions.alias(alias,value);
        new_self
    }

    pub fn transition_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.transitions.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }

    pub fn radius(&self, alias:&str, value:&str) -> Theme {
        let mut new_self = self.clone();
        new_self.radii.alias(alias,value);
        new_self
    }

    pub fn radius_scale<'a, S: Into<Vec<&'a str>>>(&self, scale: S) -> Theme {
        let mut new_self = self.clone();
        new_self.radii.scale = scale.into().iter().map(|s| s.to_string()).collect();
        new_self
    }


}







// enum AnimPlayState {
//     Running,
//     Paused
// }

// impl Default for AnimPlayState {
//     fn default() -> Self {
//         AnimPlayState::Running
//     }
// }

// enum AnimFillMode {
//     None,
//     Forwards,

//     Backwards,
//     Both,
// }


// impl Default for AnimFillMode {
//     fn default() -> Self {
//         AnimFillMode::None
//     }
// }


// enum AnimTimingFunction {
//     Ease,
//     Linear,
//     EaseIn,
//     EaseOut,
//     EaseInOut,
//     CubicBezier(f32,f32,f32,f32)
// }

// impl Default for AnimTimingFunction {
//     fn default() -> Self {
//     AnimTimingFunction::Ease
//     }
// }


// enum AnimDirection {
//     Normal,
//     Reverse,
//     Alternate,
//     AlternateReverse,
// }


// impl Default for AnimDirection {
//     fn default() -> Self {
//         AnimDirection::Normal
//     }
// }


