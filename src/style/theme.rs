
use crate::style::css_values::*;
use std::{collections::HashMap};
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;

// pub trait ColorTheme: Eq +  Hash + Clone{
//     type T;
// }
pub trait BorderTheme: Eq + Hash + Clone{}
pub trait BorderWidthTheme: Eq  + Hash + Clone{}
pub trait BorderStyleTheme: Eq +  Hash + Clone{}
pub trait SpaceTheme: Eq +  Hash + Clone{}
pub trait LineHeightTheme: Eq +  Hash + Clone{}
pub trait LetterSpacingTheme: Eq +  Hash + Clone{}
pub trait BorderRadiusTheme: Eq +  Hash + Clone{}
pub trait FontTheme: Eq +  Hash + Clone{}
pub trait FontSizeTheme: Eq +  Hash + Clone{}
pub trait SizeTheme: Eq +  Hash + Clone{}
pub trait TransitionTheme: Eq +  Hash + Clone{}
pub trait ZIndexTheme: Eq +  Hash + Clone{}
pub trait DisplayTheme: Eq +  Hash + Clone{}
pub trait ColorTheme: Eq +  Hash + Clone{}

trait ATheme{
    type StyleList;
}



impl <T>From<(T,CssSize)> for CssSize where T: SizeTheme + 'static {
    fn from(v:(T,CssSize)) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        if let Some(theme_value) = borrowed_theme.get::<T,CssSize>(v.0){
            theme_value.clone()
        } else {
            v.1
        }
    }
}

impl <T> From<T> for CssSize where T:SizeTheme + 'static{
    fn from(v: T) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        borrowed_theme.get::<T,CssSize>(v).unwrap()
    }
}


impl <T>From<(T,CssSpace)> for CssSpace where T: SpaceTheme + 'static {
    fn from(v:(T,CssSpace)) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        if let Some(theme_value) = borrowed_theme.get::<T,CssSpace>(v.0){
            theme_value.clone()
        } else {
            v.1
        }
    }
}

impl <T> From<T> for CssSpace where T:SpaceTheme + 'static{
    fn from(v: T) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        borrowed_theme.get::<T,CssSpace>(v).unwrap()
    }
}



trait ThemeLookUpTrait{
    type R: 'static + Clone ;

    fn get_value(self) -> Option<Self::R>where Self:'static+ Clone + Hash + Eq {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();

        borrowed_theme.general_get::<Self,Self::R>(self)
    }
}



pub trait OverloadedGeneralStyleLookUp<T,R> {
    fn overloaded_general_lookup(&self, alias: T) -> Option<R>;
}


impl <Q,R> OverloadedGeneralStyleLookUp<Q,R> for AnyTheme where Q:'static+  Clone + Hash + Eq , R:'static+ Clone {
    fn overloaded_general_lookup(&self, alias: Q) -> Option<R> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,R>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}

use anymap::any::{Any};
  


pub fn get_theme() -> Rc<RefCell<AnyTheme>>{   
    if let  Some(theme) = illicit::Env::get::<Rc<RefCell<AnyTheme>>>(){
        theme.clone()
    } else {
        Rc::new(RefCell::new(AnyTheme::default()))
    }
}

#[derive(Debug)]
pub struct AnyTheme{
   
   pub anymap: anymap::Map<dyn Any >,
   pub spaces_scale: Vec<CssSpace>,
   pub font_sizes_scale: Vec<CssFontSize>,
   pub fonts_scale: Vec<CssFont>,
   pub font_weights_scale: Vec<CssFontWeight>,
   pub line_heights_scale: Vec<CssLineHeight>,
   pub letter_spacings_scale: Vec<CssLetterSpacing>,
   pub sizes_scale: Vec<CssSize>,
   pub borders_scale: Vec<CssBorder>,
   pub border_widths_scale: Vec<CssBorderWidth>,
}                



impl Default for AnyTheme {   
    fn default() -> Self {
        AnyTheme {
            anymap: anymap::Map::<dyn Any>::new(),
            spaces_scale: vec![],
            font_sizes_scale: vec![],
            fonts_scale: vec![],
            font_weights_scale: vec![],
            line_heights_scale: vec![],
            letter_spacings_scale: vec![],
            sizes_scale: vec![],
            borders_scale: vec![],
            border_widths_scale: vec![],
        }
    }

}


pub trait OverloadedStyleLookUp<T,R> {
    fn overloaded_lookup(&self, alias: T) -> Option<R>;
}

impl <Q:'static + SizeTheme> OverloadedStyleLookUp<Q,CssSize> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssSize> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssSize>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}

impl <T>From<(T,CssColor)> for CssColor where T: ColorTheme + 'static {
    fn from(v:(T,CssColor)) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        if let Some(theme_value) = borrowed_theme.get::<T,CssColor>(v.0){
            theme_value.clone()
        } else {
            v.1
        }
    }
}

impl <T> From<T> for CssColor where T:ColorTheme + 'static{
    fn from(v: T) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        borrowed_theme.get::<T,CssColor>(v).unwrap()
    }
}


impl <Q:'static + SpaceTheme> OverloadedStyleLookUp<Q,CssSpace> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssSpace> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssSpace>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}



impl <Q:'static + Eq + Hash+ ColorTheme > OverloadedStyleLookUp<Q,CssColor> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssColor> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssColor>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}


impl <Q:'static + BorderTheme > OverloadedStyleLookUp<Q,CssBorder> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorder> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssBorder>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}



impl <Q:'static + BorderWidthTheme > OverloadedStyleLookUp<Q,CssBorderWidth> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorderWidth> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssBorderWidth>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}


impl <Q:'static + BorderStyleTheme > OverloadedStyleLookUp<Q,CssBorderStyle> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorderStyle> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssBorderStyle>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}



impl <Q:'static + BorderRadiusTheme > OverloadedStyleLookUp<Q,CssBorderRadius> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorderRadius> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssBorderRadius>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}


impl <Q:'static + TransitionTheme > OverloadedStyleLookUp<Q,CssTransition> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssTransition> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssTransition>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}


impl <Q:'static + ZIndexTheme > OverloadedStyleLookUp<Q,CssZIndex> for AnyTheme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssZIndex> {
        if let Some(hm) = self.anymap.get::< HashMap::<Q,CssZIndex>  >(){
            hm.get(&alias).cloned()
        } else {            
            None
        }
    }
}


impl AnyTheme {
   pub fn new() -> AnyTheme {
        AnyTheme::default()
    }

    pub fn space_scale<S: Into<Vec<CssSpace>>>( &mut self, scale: S) -> &mut AnyTheme {
        self.spaces_scale = scale.into();
        self
   }

   pub fn font_size_scale<S: Into<Vec<CssFontSize>>>( &mut self, scale: S) -> &mut AnyTheme {
        self.font_sizes_scale = scale.into();
        self
    }
    

    pub fn font_weight_scale<S: Into<Vec<CssFontWeight>>>( &mut self, scale: S) -> &mut AnyTheme {
        self.font_weights_scale = scale.into();
        self 
    }


    pub fn size_scale<S: Into<Vec<CssSize>>>(&mut self, scale: S) -> &mut AnyTheme { 
        self.sizes_scale = scale.into();
        self
    }

    pub fn line_heights_scale<S: Into<Vec<CssLineHeight>>>(&mut self, scale: S) -> &mut AnyTheme {
        self.line_heights_scale = scale.into();
        self
    }

    pub fn letter_spacing_scale<S: Into<Vec<CssLetterSpacing>>>(&mut self, scale: S) -> &mut AnyTheme {
        self.letter_spacings_scale =scale.into();
        self
    }

    pub fn border_scale<S: Into<Vec<CssBorder>>>(&mut self, scale: S) -> &mut AnyTheme {      
        self.borders_scale = scale.into();
        self
    }

    pub fn border_width_scale<S: Into<Vec<CssBorderWidth>>>(&mut self, scale: S) -> &mut AnyTheme {
        self.border_widths_scale = scale.into();
        self
    }

    pub fn general_get<T,R>(&self, alias: T) -> Option<R> 
    where Self:OverloadedGeneralStyleLookUp<T,R>
     ,T:Clone {
        self.overloaded_general_lookup(alias)
    }

    pub fn get<T,R>(&self, alias: T) -> Option<R> 
    where Self:OverloadedStyleLookUp<T,R>
     ,T:Clone {
        self.overloaded_lookup(alias)
    }

    pub fn set<T, V>(self, alias: T, value: V) -> Self
        where Self:OverloadedStyleSet<T, V>
    {
        let mut myself = self;
        myself.overloaded_set(alias, value);
        myself
    }

    // fn get_size<T: 'static + SizeTheme>(&mut self, alias:T, value: CssSize) -> Option<CssSize>{
    //     self.anymap.get::< HashMap::<T,CssSize>  >().map(|hm| hm.get(&alias)).flatten().cloned()
    // }
}


pub trait OverloadedStyleSet<T, V> {
    fn overloaded_set(&mut self, alias: T, value: V);
}



impl <Q:'static + SizeTheme> OverloadedStyleSet<Q, CssSize> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssSize) {  
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssSize>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssSize>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}

impl <Q:'static + ColorTheme> OverloadedStyleSet<Q, CssColor> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssColor) {  
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssColor>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssColor>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}


impl <Q:'static + SpaceTheme> OverloadedStyleSet<Q, CssSpace> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssSpace) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssSpace>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssSpace>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}


impl <Q:'static + FontSizeTheme> OverloadedStyleSet<Q, CssFontSize> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssFontSize) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssFontSize>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssFontSize>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}


impl <Q:'static + BorderTheme> OverloadedStyleSet<Q, CssBorder> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssBorder) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssBorder>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssBorder>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}


impl <Q:'static + BorderWidthTheme> OverloadedStyleSet<Q, CssBorderWidth> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssBorderWidth) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssBorderWidth>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssBorderWidth>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}


impl <Q:'static + BorderStyleTheme> OverloadedStyleSet<Q, CssBorderStyle> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssBorderStyle) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssBorderStyle>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssBorderStyle>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}

impl <Q:'static + BorderRadiusTheme> OverloadedStyleSet<Q, CssBorderRadius> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssBorderRadius) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssBorderRadius>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssBorderRadius>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}



impl <Q:'static + TransitionTheme> OverloadedStyleSet<Q, CssTransition> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssTransition) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssTransition>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssTransition>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}


impl <Q:'static + LineHeightTheme> OverloadedStyleSet<Q, CssLineHeight> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssLineHeight) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssLineHeight>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssLineHeight>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}



impl <Q:'static + LetterSpacingTheme> OverloadedStyleSet<Q, CssLetterSpacing> for AnyTheme {
    fn overloaded_set(&mut self, alias: Q, value: CssLetterSpacing) {
        if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssLetterSpacing>  >(){
            hm.insert(alias,value);
        } else {            
            let mut hm = HashMap::<Q,CssLetterSpacing>::new();
                    hm.insert(alias,value);
                    self.anymap.insert(hm);
        }
    }
}



// impl <Q:'static + LetterSpacingTheme> OverloadedStyleSet<Q, CssLetterSpacing> for AnyTheme {
//     fn overloaded_set(&mut self, alias: Q, value: CssLetterSpacing) {
//         if let Some(hm) = self.anymap.get_mut::< HashMap::<Q,CssLetterSpacing>  >(){
//             hm.insert(alias,value);
//         } else {            
//             let mut hm = HashMap::<Q,CssLetterSpacing>::new();
//                     hm.insert(alias,value);
//                     self.anymap.insert(hm);
//         }
//     }
// }



