use std::panic::Location;
use wasm_bindgen::JsCast;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use harsh::{Harsh,HarshBuilder};
use std::cell::RefCell;
use std::collections::HashSet;

use seed::{*, prelude::*};

use tw_seed_hooks::{generate_tw_sizes, generate_tw_colors, generate_style_accessors};
use lazy_static::*;
mod css_values;
pub use css_values::*;
use std::rc::Rc;
pub mod measures;
pub use measures::*;
mod theme;
pub use theme::*;

lazy_static! {
    pub static ref S: Style = Style {
        ..Style::default()
    };
}

thread_local! {
    pub static STYLES_IN_ELEM: RefCell<Vec<Style>> = RefCell::new(vec![]);
    pub static STYLES_USED : RefCell<HashSet<u64>> = RefCell::new(HashSet::<u64>::new());
    pub static HASH_IDS_GENERATOR: RefCell<Harsh> = RefCell::new(HarshBuilder::new().init().unwrap());
}

fn short_uniq_id(id:u64) -> String {
    HASH_IDS_GENERATOR.with(|h| h.borrow().encode(&[id]).unwrap())
}


pub fn use_theme<Ms, F>(theme: AnyTheme, content: F) -> Node<Ms>
    where
        F: Fn()-> Node<Ms>
{
    let boxed_any_theme = Rc::new(RefCell::new(theme));
    illicit::child_env!( Rc<RefCell<AnyTheme>> => boxed_any_theme).enter(content)
}



#[derive(Clone, Debug)]
pub struct Rule {

    value: CssValue,
    property: Property,
}

impl Rule {
    fn render(&self) -> String{
        format!("    {}: {};\n", self.property.render(), self.value)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Property {

    // Color Type Properties
    Color,
    BackgroundColor,
    BorderColor,
    BorderRightColor,
    BorderLeftColor,
    BorderTopColor,
    BorderBottomColor,
    OutlineColor,
    Fill,

    // Space type properties
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

    GridGap,
    GridColumnGap,
    GridRowGap,

    // Border Type Properties
    Border,
    BorderRight,
    BorderLeft,
    BorderTop,
    BorderBottom,

    // Border Witdth Type properties
    BorderWidth,
    BorderRightWidth,
    BorderLeftWidth,
    BorderTopWidth,
    BorderBottomWidth,

    // Border Style Type Properties
    BorderStyle,
    BorderRightStyle,
    BorderLeftStyle,
    BorderTopStyle,
    BorderBottomStyle,

    // Font Family Type Properties
    FontFamily,

    // Font Size Type Properties
    FontSize ,

    // Font Weight Type Properoties
    FontWeight,

    // Transition Type Properties
    Transition,

    // ZIndex Type Properties
    ZIndex ,


    // Letter Spacing Type Properties
    LetterSpacing,

    // LineSpacing Type Properties
    LineHeight,

    // Radius Type Properties
    BorderRadius,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,

    // Sizing Type Properties
    Width,
    Height,
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,

    Display,

    // Non Tytped Properties
    AnimDuration,
    AnimDelay,
    AnimIterationCount,
    AnimDirection,
    AnimTimingFunction,
    AnimFillMode,
    AnimPlayState,

    Custom,

AlignContent,
AnimationDelay,
AnimationDirection,
AnimationDuration,
AnimationFillMode,
AnimationIterationCount,
AnimationName,
AnimationPlayState,
AnimationTimingFunction,
AlignItems,
AlignSelf,
Animation,
BackfaceVisibility,
Background,
BackgroundAttachment,
BackgroundBlendMode,
BackgroundClip,
BackgroundImage,
BackgroundOrigin,
BackgroundPosition,
BackgroundRepeat,
BackgroundSize,
BorderCollapse,
BorderImage,
BorderImageOutset,
BorderImageRepeat,
BorderImageSlice,
BorderImageSource,
BorderImageWidth,
Bottom,
BoxDecorationBreak,
BoxShadow,
BoxSizing,
BreakAfter,
BreakBefore,
BreakInside,
CaptionSide,
CaretColor,
Clear,
Clip,
ClipPath,
ColumnCount,
ColumnFill,
ColumnGap,
ColumnRule,
ColumnRuleColor,
ColumnRuleStyle,
ColumnRuleWidth,
ColumnSpan,
ColumnWidth,
Columns,
Content,
CounterIncrement,
CounterReset,
Direction,
EmptyCells,
Filter,
Flex,
FlexBasis,
FlexDirection,
FlexFlow,
FlexGrow,
FlexShrink,
FlexWrap,
Float,
FontFeatureSettings,
FontKerning,
FontLanguageOverride,
FontSizeAdjust,
FontStretch,
FontSynthesis,
FontVariant,
FontVariantAlternates,
FontVariantCaps,
FontVariantEastAsian,
FontVariantLigatures,
FontVariantNumeric,
FontVariantPosition,
Grid,
GridArea,
GridAutoColumns,
GridAutoFlow,
GridAutoRows,
GridColumn,
GridColumnEnd,
GridColumnStart,
GridRow,
GridRowEnd,
GridRowStart,
GridTemplate,
GridTemplateAreas,
GridTemplateColumns,
GridTemplateRows,
Hyphens,
ImageRendering,
Isolation,
JustifyContent,
Left,
LineBreak,
Mask,
MaskType,
MixBlendMode,
ObjectFit,
ObjectPosition,
Opacity,
Order,
Orphans,
Overflow,
OverflowWrap,
OverflowX,
OverflowY,
PageBreakAfter,
PageBreakBefore,
PageBreakInside,
Perspective,
PerspectiveOrigin,
PlaceContent,
PointerEvents,
Position,
Quotes,
Resize,
Right,
ScrollBehavior,
ShapeImageThreshold,
ShapeMargin,
TabSize,
TableLayout,
TextAlign,
TextAlignLast,
TextCombineUpright,
TextDecoration,
TextDecorationColor,
TextDecorationLine,
TextDecorationStyle,
TextEmphasis,
TextEmphasisColor,
TextEmphasisPosition,
TextEmphasisStyle,
TextIndent,
TextJustify,
TextOrientation,
TextOverflow,
TextShadow,
TextTransform,
TextUnderlinePosition,
Top,
TouchAction,
Transform,
TransformOrigin,
TransformStyle,
TransitionDelay,
TransitionDuration,
TransitionProperty,
TransitionTimingFunction,
UnicodeBidi,
UserSelect,
VerticalAlign,
Visibility,
WhiteSpace,
Widows,
WillChange,
WordBreak,
WordSpacing,
WordWrap,
WritingMode,

}

impl Property {
    fn render(&self) -> String {
  
         match self {
            Property::Color => "color",
            Property::BackgroundColor => "background-color",

            Property::Padding => "padding",
            Property::PaddingRight => "padding-right",
            Property::PaddingLeft => "padding-left",
            Property::PaddingTop => "padding-top",
            Property::PaddingBottom => "padding-bottom",

            Property::Margin => "margin",
            Property::MarginRight => "margin-right",
            Property::MarginLeft => "margin-left",
            Property::MarginTop => "margin-top",
            Property::MarginBottom => "margin-bottom",

            Property::Border => "border",
            Property::BorderRight => "border-right",
            Property::BorderLeft => "border-left",
            Property::BorderTop => "border-top",
            Property::BorderBottom => "border-bottom",

            Property::BorderColor => "border-color",
            Property::BorderRightColor => "border-right-color",
            Property::BorderLeftColor =>"border-left-color",
            Property::BorderTopColor => "border-top-color",
            Property::BorderBottomColor => "border-bottom-color",

            Property::OutlineColor => "outline-color",
            Property::Fill=> "fill",

            Property::BorderWidth => "border-width",
            Property::BorderRightWidth => "border-right-width",
            Property::BorderLeftWidth => "border-left-width",
            Property::BorderTopWidth => "border-top-width",
            Property::BorderBottomWidth => "border-bottom-width",

            Property::BorderStyle => "border-style",
            Property::BorderRightStyle => "border-right-style",
            Property::BorderLeftStyle => "border-left-style",
            Property::BorderTopStyle => "border-top-style",
            Property::BorderBottomStyle => "border-bottom-style",

            Property::Width => "width",
            Property::Height => "height",
            Property::MinWidth => "min-width",
            Property::MaxWidth  => "max-width",
            Property::MinHeight => "min-height",
            Property::MaxHeight => "max-height",

            Property::Display => "display",
            Property::Transition => "transition",
            Property::BorderRadius => "border-radius",

            Property::BorderTopLeftRadius => "border-top-left-radius",
            Property::BorderTopRightRadius => "border-top-right-radius",
            Property::BorderBottomLeftRadius => "border-bottom-left-radius",
            Property::BorderBottomRightRadius => "border-bottom-right-radius",

            Property::AnimDuration => "animation-duration",
            Property::AnimDelay => "animation-delay",
            Property::AnimIterationCount => "animation-iteration-count",
            Property::AnimDirection => "animation-direction",
            Property::AnimTimingFunction => "animation-timing-function",
            Property::AnimFillMode => "animation-fill-mode",
            Property::AnimPlayState => "animation-play-state",

            Property::FontFamily => "font-family",
            Property::FontSize  => "font-size",
            Property::FontWeight => "font-weight",
            Property::ZIndex  => "z-index",

            Property::LineHeight => "line-height",

            Property::GridGap => "grid-gap",
        
            Property::GridRowGap => "grid-row-gap",
            Property::LetterSpacing => "letter-spacing",
            Property::Custom => "",

            Property::AlignContent => "align-content",
            Property::AlignItems => "align-items",
            Property::AlignSelf => "align-self",
            Property::Animation => "animation",
            Property::AnimationDelay=> "animation_delay",
            Property::AnimationDirection=> "animation_direction",
            Property::AnimationDuration=> "animation_duration",
            Property::AnimationFillMode=> "animation_fill_mode",
            Property::AnimationIterationCount=> "animation_iteration_count",
            Property::AnimationName=> "animation_name",
            Property::AnimationPlayState=> "animation_play_state",
            Property::AnimationTimingFunction=> "animation_timing_function",
            Property::BackfaceVisibility => "backface-visibility",
            Property::Background => "background",
            Property::BackgroundAttachment => "background-attachment",
            Property::BackgroundBlendMode => "background-blend-mode",
            Property::BackgroundClip => "background-clip",
            
            Property::BackgroundImage => "background-image",
            Property::BackgroundOrigin => "background-origin",
            Property::BackgroundPosition => "background-position",
            Property::BackgroundRepeat => "background-repeat",
            Property::BackgroundSize => "background-size",
            Property::BorderCollapse => "border-collapse",
            Property::BorderImage => "border-image",
            Property::BorderImageOutset => "border-image-outset",
            Property::BorderImageRepeat => "border-image-repeat",
            Property::BorderImageSlice => "border-image-slice",
            Property::BorderImageSource => "border-image-source",
            Property::BorderImageWidth => "border-image-width",
            Property::Bottom => "bottom",
            Property::BoxDecorationBreak => "box-decoration-break",
            Property::BoxShadow => "box-shadow",
            Property::BoxSizing => "box-sizing",
            Property::BreakAfter => "break-after",
            Property::BreakBefore => "break-before",
            Property::BreakInside => "break-inside",
            Property::CaptionSide => "caption-side",
            Property::CaretColor => "caret-color",
            Property::Clear => "clear",
            Property::Clip => "clip",
            Property::ClipPath => "clip-path",
            Property::ColumnCount => "column-count",
            Property::ColumnFill => "column-fill",
            Property::ColumnGap => "column-gap",
            Property::ColumnRule => "column-rule",
            Property::ColumnRuleColor => "column-rule-color",
            Property::ColumnRuleStyle => "column-rule-style",
            Property::ColumnRuleWidth => "column-rule-width",
            Property::ColumnSpan => "column-span",
            Property::ColumnWidth => "column-width",
            Property::Columns => "columns",
            Property::Content => "content",
            Property::CounterIncrement => "counter-increment",
            Property::CounterReset => "counter-reset",
            Property::Direction => "direction",
            Property::EmptyCells => "empty-cells",
            Property::Filter => "filter",
            Property::Flex => "flex",
            Property::FlexBasis => "flex-basis",
            Property::FlexDirection => "flex-direction",
            Property::FlexFlow => "flex-flow",
            Property::FlexGrow => "flex-grow",
            Property::FlexShrink => "flex-shrink",
            Property::FlexWrap => "flex-wrap",
            Property::Float => "float",
            Property::FontFeatureSettings => "font-feature-settings",
            Property::FontKerning => "font-kerning",
            Property::FontLanguageOverride => "font-language-override",
            Property::FontSizeAdjust => "font-size-adjust",
            Property::FontStretch => "font-stretch",
            Property::FontSynthesis => "font-synthesis",
            Property::FontVariant => "font-variant",
            Property::FontVariantAlternates => "font-variant-alternates",
            Property::FontVariantCaps => "font-variant-caps",
            Property::FontVariantEastAsian => "font-variant-east-asian",
            Property::FontVariantLigatures => "font-variant-ligatures",
            Property::FontVariantNumeric => "font-variant-numeric",
            Property::FontVariantPosition => "font-variant-position",
            Property::Grid => "grid",
            Property::GridArea => "grid-area",
            Property::GridAutoColumns => "grid-auto-columns",
            Property::GridAutoFlow => "grid-auto-flow",
            Property::GridAutoRows => "grid-auto-rows",
            Property::GridColumn => "grid-column",
            Property::GridColumnEnd => "grid-column-end",
            Property::GridColumnGap => "grid-column-gap",
            Property::GridColumnStart => "grid-column-start",
            Property::GridRow => "grid-row",
            Property::GridRowEnd => "grid-row-end",
            Property::GridRowStart => "grid-row-start",
            Property::GridTemplate => "grid-template",
            Property::GridTemplateAreas => "grid-template-areas",
            Property::GridTemplateColumns => "grid-template-columns",
            Property::GridTemplateRows => "grid-template-rows",
            Property::Hyphens => "hyphens",
            Property::ImageRendering => "image-rendering",
            Property::Isolation => "isolation",
            Property::JustifyContent => "justify-content",
            Property::Left => "left",
            Property::LineBreak => "line-break",
            
            Property::Mask => "mask",
            Property::MaskType => "mask-type",
            Property::MixBlendMode => "mix-blend-mode",
            Property::ObjectFit => "object-fit",
            Property::ObjectPosition => "object-position",
            Property::Opacity => "opacity",
            Property::Order => "order",
            Property::Orphans => "orphans",
            Property::Overflow => "overflow",
            Property::OverflowWrap => "overflow-wrap",
            Property::OverflowX => "overflow-x",
            Property::OverflowY => "overflow-y",
            Property::PageBreakAfter => "page-break-after",
            Property::PageBreakBefore => "page-break-before",
            Property::PageBreakInside => "page-break-inside",
            Property::Perspective => "perspective",
            Property::PerspectiveOrigin => "perspective-origin",
            Property::PlaceContent => "place-content",
            Property::PointerEvents => "pointer-events",
            Property::Position => "position",
            Property::Quotes => "quotes",
            Property::Resize => "resize",
            Property::Right => "right",
            Property::ScrollBehavior => "scroll-behavior",
            Property::ShapeImageThreshold => "shape-image-threshold",
            Property::ShapeMargin => "shape-margin",
            Property::TabSize => "tab-size",
            Property::TableLayout => "table-layout",
            Property::TextAlign => "text-align",
            Property::TextAlignLast => "text-align-last",
            Property::TextCombineUpright => "text-combine-upright",
            Property::TextDecoration => "text-decoration",
            Property::TextDecorationColor => "text-decoration-color",
            Property::TextDecorationLine => "text-decoration-line",
            Property::TextDecorationStyle => "text-decoration-style",
            Property::TextEmphasis => "text-emphasis",
            Property::TextEmphasisColor => "text-emphasis-color",
            Property::TextEmphasisPosition => "text-emphasis-position",
            Property::TextEmphasisStyle => "text-emphasis-style",
            Property::TextIndent => "text-indent",
            Property::TextJustify => "text-justify",
            Property::TextOrientation => "text-orientation",
            Property::TextOverflow => "text-overflow",
            Property::TextShadow => "text-shadow",
            Property::TextTransform => "text-transform",
            Property::TextUnderlinePosition => "text-underline-position",
            Property::Top => "top",
            Property::TouchAction => "touch-action",
            Property::Transform => "transform",
            Property::TransformOrigin => "transform-origin",
            Property::TransformStyle => "transform-style",
            
            Property::TransitionDelay => "transition-delay",
            Property::TransitionDuration => "transition-duration",
            Property::TransitionProperty => "transition-property",
            Property::TransitionTimingFunction => "transition-timing-function",
            Property::UnicodeBidi => "unicode-bidi",
            Property::UserSelect => "user-select",
            Property::VerticalAlign => "vertical-align",
            Property::Visibility => "visibility",
            Property::WhiteSpace => "white-space",
            Property::Widows => "widows",
            
            Property::WillChange => "will-change",
            Property::WordBreak => "word-break",
            Property::WordSpacing => "word-spacing",
            Property::WordWrap => "word-wrap",
            Property::WritingMode => "writing-mode",



         }.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    rules: Vec<Rule>,
    updated_at: Vec<String>,
    pseudo: Pseudo,
    media: Option<String>,
    name: String,
    keyframes: Keyframes,
}

impl Default for Style {

#[track_caller]
    fn default() -> Self {
    Style{
        pseudo: Pseudo::None,
        updated_at: vec![format!("{}", Location::caller())],
        rules: vec![],
        name: "".to_string(),
        media: None,
        keyframes: Keyframes::default(),
        }
    }
}


// pub trait OverloadedProperty<T,R>
// {
//     fn overloaded_property(&self, property: Property, val: T) -> Style where T: Into<R>;
// }


// impl <T> OverloadedProperty<T,CssSpace> for Style {
//     fn overloaded_property(&self, property: Property , val: T ) -> Style where T:Into<CssSpace> {
//         {
//             let val = val.into();
//             let mut new_self = self.clone();
//             new_self.add_rule(property,CssValue::Space(val));
//             new_self
//         }
//     }
// }





use Property::*;

impl Style {
    fn set_space_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssSpace> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::Space(val));
            new_self
        }
    }

    fn set_border_style_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssBorderStyle> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::BorderStyle(val));
            new_self
        }
    }

    fn set_border_width_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssBorderWidth> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::BorderWidth(val));
            new_self
        }
    }


    fn set_size_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssSize> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::Size(val));
            new_self
        }
    }


    fn set_color_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssColor> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::Color(val));
            new_self
        }
    }

    fn set_border_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssBorder> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::Border(val));
            new_self
        }
    }

    fn set_border_radius_property<T>(&self, property: Property , val: T ) -> Style where T:Into<CssBorderRadius> {
        {
            let val = val.into();
            let mut new_self = self.clone();
            new_self.add_rule(property,CssValue::BorderRadius(val));
            new_self
        }
    }





    // SNAKE_CASE(&self, val:T) -> Style where T:Into<CssSUPPERCASE> {
    //     let val = val.into();
    //     let mut new_self = self.clone();
    //     new_self.add_rule(property,CssValue::UPPERCASE(val));
    //     new_self
    // }

    generate_style_accessors!([
        "AlignContent",
        "AlignItems",
        "AlignSelf",
        "Animation",
        "AnimationDelay",
        "AnimationDirection",
        "AnimationDuration",
        "AnimationFillMode",
        "AnimationIterationCount",
        "AnimationName",
        "AnimationPlayState",
        "AnimationTimingFunction",
        "BackfaceVisibility",
        "Background",
        "BackgroundAttachment",
        "BackgroundBlendMode",
        "BackgroundClip",
        "BackgroundImage",
        "BackgroundOrigin",
        "BackgroundPosition",
        "BackgroundRepeat",
        "BackgroundSize",
        "BorderCollapse",
        "BorderImage",
        "BorderImageOutset",
        "BorderImageRepeat",
        "BorderImageSlice",
        "BorderImageSource",
        "BorderImageWidth",
        "Bottom",
        "BoxDecorationBreak",
        "BoxShadow",
        "BoxSizing",
        "BreakAfter",
        "BreakBefore",
        "BreakInside",
        "CaptionSide",
        "CaretColor",
        "Clear",
        "Clip",
        "ClipPath",
        "ColumnCount",
        "ColumnFill",
        "ColumnGap",
        "ColumnRule",
        "ColumnRuleColor",
        "ColumnRuleStyle",
        "ColumnRuleWidth",
        "ColumnSpan",
        "ColumnWidth",
        "Columns",
        "Content",
        "CounterIncrement",
        "CounterReset",
        "Direction",
        "Display",
        "EmptyCells",
        "Filter",
        "Flex",
        "FlexBasis",
        "FlexDirection",
        "FlexFlow",
        "FlexGrow",
        "FlexShrink",
        "FlexWrap",
        "Float",
        "FontSize",
        "FontFamily",
        "FontFeatureSettings",
        "FontKerning",
        "FontLanguageOverride",
        "FontSizeAdjust",
        "FontStretch",
        "FontSynthesis",
        "FontVariant",
        "FontVariantAlternates",
        "FontVariantCaps",
        "FontVariantEastAsian",
        "FontVariantLigatures",
        "FontVariantNumeric",
        "FontVariantPosition",
        "Grid",
        "GridArea",
        "GridAutoColumns",
        "GridAutoFlow",
        "GridAutoRows",
        "GridColumn",
        "GridColumnEnd",
        "GridColumnGap",
        "GridColumnStart",
        "GridRow",
        "GridRowEnd",
        "GridRowStart",
        "GridTemplate",
        "GridTemplateAreas",
        "GridTemplateColumns",
        "GridTemplateRows",
        "Hyphens",
        "ImageRendering",
        "Isolation",
        "JustifyContent",
        "Left",
        "LineBreak",
        "LineHeight",
        "Mask",
        "MaskType",
        "MixBlendMode",
        "ObjectFit",
        "ObjectPosition",
        "Opacity",
        "Order",
        "Orphans",
        "Overflow",
        "OverflowWrap",
        "OverflowX",
        "OverflowY",
        "PageBreakAfter",
        "PageBreakBefore",
        "PageBreakInside",
        "Perspective",
        "PerspectiveOrigin",
        "PlaceContent",
        "PointerEvents",
        "Position",
        "Quotes",
        "Resize",
        "Right",
        "ScrollBehavior",
        "ShapeImageThreshold",
        "ShapeMargin",
        "TabSize",
        "TableLayout",
        "TextAlign",
        "TextAlignLast",
        "TextCombineUpright",
        "TextDecoration",
        "TextDecorationColor",
        "TextDecorationLine",
        "TextDecorationStyle",
        "TextEmphasis",
        "TextEmphasisColor",
        "TextEmphasisPosition",
        "TextEmphasisStyle",
        "TextIndent",
        "TextJustify",
        "TextOrientation",
        "TextOverflow",
        "TextShadow",
        "TextTransform",
        "TextUnderlinePosition",
        "Top",
        "TouchAction",
        "Transform",
        "TransformOrigin",
        "TransformStyle",
        "Transition",
        "TransitionDelay",
        "TransitionDuration",
        "TransitionProperty",
        "TransitionTimingFunction",
        "UnicodeBidi",
        "UserSelect",
        "VerticalAlign",
        "Visibility",
        "WhiteSpace",
        "Widows",
        "WillChange",
        "WordBreak",
        "WordSpacing",
        "WordWrap",
        "WritingMode"
        ]);






    //
    //   CssSpaces
    //
    #[track_caller] pub fn margin<T>(&self, val:T) -> Style where T: Into<CssSpace>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(Margin, val)}
    #[track_caller] pub fn margin_right<T>(&self, val:T) -> Style where T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginRight, val)}
    #[track_caller] pub fn margin_left<T>(&self, val:T) -> Style where T: Into<CssSpace>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginLeft, val)}
    #[track_caller] pub fn margin_top<T>(&self, val:T) -> Style where  T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginTop, val)}
    #[track_caller] pub fn margin_bottom<T>(&self, val:T) -> Style where T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginBottom, val)}

    #[track_caller] pub fn m<T>(&self, val:T) -> Style where  T: Into<CssSpace>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(Margin, val)}
    #[track_caller] pub fn mr<T>(&self, val:T) -> Style where  T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginRight, val)}
    #[track_caller] pub fn ml<T>(&self, val:T) -> Style where  T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginLeft, val)}
    #[track_caller] pub fn mt<T>(&self, val:T) -> Style where T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginTop, val)}
    #[track_caller] pub fn mb<T>(&self, val:T) -> Style where  T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(MarginBottom, val)}
    #[track_caller] pub fn mx<T>(&self, val:T) -> Style where T:Clone+ Into<CssSpace> {  let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.set_space_property(MarginLeft, val.clone()); new_self.set_space_property(MarginRight,val)}
    #[track_caller] pub fn my<T>(&self, val:T) -> Style where T:Clone + Into<CssSpace> {  let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.set_space_property(MarginTop, val.clone()); new_self.set_space_property(MarginBottom,val)}

    #[track_caller] pub fn padding<T>(&self, val:T) -> Style where T: Into<CssSpace>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(Padding, val)}
    #[track_caller] pub fn padding_right<T>(&self, val:T) -> Style where T: Into<CssSpace>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingRight, val)}
    #[track_caller] pub fn padding_left<T>(&self, val:T) -> Style where  T: Into<CssSpace>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingLeft, val)}
    #[track_caller] pub fn padding_top<T>(&self, val:T) -> Style where  T: Into<CssSpace>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingTop, val)}
    #[track_caller] pub fn padding_bottom<T>(&self, val:T) -> Style where T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingBottom, val)}
    #[track_caller] pub fn padding_x<T>(&self, val:T) -> Style where T: Clone + Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.set_space_property(PaddingLeft, val.clone()); new_self.set_space_property(PaddingRight,val)}
    #[track_caller] pub fn padding_y<T>(&self, val:T) -> Style where T: Clone + Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.set_space_property(PaddingTop, val.clone()); new_self.set_space_property(PaddingBottom,val)}

    #[track_caller] pub fn p<T>(&self, val:T) -> Style where  T: Into<CssSpace>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(Padding, val)}
    #[track_caller] pub fn pr<T>(&self, val:T) -> Style where T: Into<CssSpace>, T: Into<CssSpace>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingRight, val)}
    #[track_caller] pub fn pl<T>(&self, val:T) -> Style where T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingLeft, val)}
    #[track_caller] pub fn pt<T>(&self, val:T) -> Style where T: Into<CssSpace>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingTop, val)}
    #[track_caller] pub fn pb<T>(&self, val:T) -> Style where T: Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_space_property(PaddingBottom, val)}
    #[track_caller] pub fn px<T>(&self, val:T) -> Style where T:Clone+  Into<CssSpace> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.set_space_property(PaddingLeft, val.clone()); new_self.set_space_property(PaddingRight,val)}
    #[track_caller] pub fn py<T>(&self, val:T) -> Style where T:Clone+  Into<CssSpace>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));let new_self = new_style.set_space_property(PaddingTop, val.clone()); new_self.set_space_property(PaddingBottom,val)}

    //
    //   CssColors
    //
    #[track_caller] pub fn border_right_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderRightColor, val)}
    #[track_caller] pub fn border_left_color<T>(&self, val:T) -> Style where T: Into<CssColor>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderLeftColor, val)}
    #[track_caller] pub fn border_top_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderTopColor, val)}
    #[track_caller] pub fn border_bottom_color<T>(&self, val:T) -> Style where T: Into<CssColor>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderBottomColor, val)}

    #[track_caller] pub fn outline_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(OutlineColor, val)}
    #[track_caller] pub fn color<T>(&self, val:T) -> Style where T: Into<CssColor>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(Color, val)}
    #[track_caller] pub fn background_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BackgroundColor, val)}
    #[track_caller] pub fn bg_color<T>(&self, val:T) -> Style where T: Into<CssColor>   { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BackgroundColor, val)}
    #[track_caller] pub fn fill<T>(&self, val:T) -> Style where T:Clone + Into<CssColor>  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(Fill, val)}

    #[track_caller] pub fn border_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderColor, val)}
    #[track_caller] pub fn border_r_color<T>(&self, val:T) -> Style where  T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderRightColor, val)}
    #[track_caller] pub fn border_l_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderLeftColor, val)}
    #[track_caller] pub fn border_t_color<T>(&self, val:T) -> Style where T: Into<CssColor> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderTopColor, val)}
    #[track_caller] pub fn border_b_color<T>(&self, val:T) -> Style where T:  Into<CssColor>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_color_property(BorderBottomColor, val)}

    //
    //   CssSizes
    //

    #[track_caller] pub fn width<T>(&self, val:T) -> Style where T:  Into<CssSize>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_size_property(Width, val)}
    #[track_caller] pub fn w<T>(&self, val:T) -> Style where T:  Into<CssSize>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_size_property(Width, val)}
    #[track_caller] pub fn height<T>(&self, val:T) -> Style where T:  Into<CssSize>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_size_property(Height, val)}
    #[track_caller] pub fn h<T>(&self, val:T) -> Style where T:  Into<CssSize>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_size_property(Height, val)}


    //
    //   CssBorderWidths
    //

    #[track_caller] pub fn border_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderWidth, val)}
    #[track_caller] pub fn border_right_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, val)}
    #[track_caller] pub fn border_left_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, val)}
    #[track_caller] pub fn border_top_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, val)}
    #[track_caller] pub fn border_bottom_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, val)}

    #[track_caller] pub fn border_r_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, val)}
    #[track_caller] pub fn border_l_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, val)}
    #[track_caller] pub fn border_t_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, val)}
    #[track_caller] pub fn border_b_width<T>(&self, val:T) -> Style where T:  Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, val)}

    //
    //   CssBorderStyles
    //

    #[track_caller] pub fn border_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderStyle, val)}
    #[track_caller] pub fn border_right_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderRightStyle, val)}
    #[track_caller] pub fn border_left_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderLeftStyle, val)}
    #[track_caller] pub fn border_top_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderTopStyle, val)}
    #[track_caller] pub fn border_bottom_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderBottomStyle, val)}

    #[track_caller] pub fn border_r_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderRightStyle, val)}
    #[track_caller] pub fn border_l_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderLeftStyle, val)}
    #[track_caller] pub fn border_t_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderTopStyle, val)}
    #[track_caller] pub fn border_b_style<T>(&self, val:T) -> Style where T:  Into<CssBorderStyle> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_style_property(BorderBottomStyle, val)}

    //
    // Border
    //

    #[track_caller] pub fn border<T>(&self, val:T) -> Style where T:  Into<CssBorder>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_property(Border, val)}

    #[track_caller] pub fn border_right<T>(&self, val:T) -> Style where T:  Into<CssBorder> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_property(BorderRight, val)}
    #[track_caller] pub fn border_left<T>(&self, val:T) -> Style where T:  Into<CssBorder> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_property(BorderLeft, val)}
    #[track_caller] pub fn border_top<T>(&self, val:T) ->  Style where T:  Into<CssBorder> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_property(BorderTop, val)}
    #[track_caller] pub fn border_bottom<T>(&self, val:T) ->Style  where T:  Into<CssBorder>{ let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_property(BorderBottom, val)}




    // Tw Styles
    #[track_caller] pub fn border_0(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderWidth, "0px")}
    #[track_caller] pub fn border_1(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderWidth, "1px")}
    #[track_caller] pub fn border_2(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderWidth, "2px")}
    #[track_caller] pub fn border_4(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderWidth, "4px")}
    #[track_caller] pub fn border_8(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderWidth, "8px")}

    #[track_caller] pub fn border_r<T>(&self, val:T) ->  Style where T: Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, val)}
    #[track_caller] pub fn border_r_0(&self) ->  Style where { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, "0px")}
    #[track_caller] pub fn border_r_1(&self) ->  Style where { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, "1px")}
    #[track_caller] pub fn border_r_2(&self) ->  Style where { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, "2px")}
    #[track_caller] pub fn border_r_4(&self) ->  Style where { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, "4px")}
    #[track_caller] pub fn border_r_8(&self) ->  Style where { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderRightWidth, "8px")}

    #[track_caller] pub fn border_l  <T>(&self, val:T) ->  Style where T: Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, val)}
    #[track_caller] pub fn border_l_0(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, "0px")}
    #[track_caller] pub fn border_l_1(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, "1px")}
    #[track_caller] pub fn border_l_2(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, "2px")}
    #[track_caller] pub fn border_l_4(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, "4px")}
    #[track_caller] pub fn border_l_8(&self) ->  Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderLeftWidth, "8px")}

    #[track_caller] pub fn border_t <T>(&self, val:T) ->  Style where T: Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, val)}
    #[track_caller] pub fn border_t_0(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, "0px")}
    #[track_caller] pub fn border_t_1(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, "1px")}
    #[track_caller] pub fn border_t_2(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, "2px")}
    #[track_caller] pub fn border_t_4(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, "4px")}
    #[track_caller] pub fn border_t_8(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderTopWidth, "8px")}


    #[track_caller] pub fn border_b  <T>(&self, val:T) ->  Style where T: Into<CssBorderWidth> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, val)}
    #[track_caller] pub fn border_b_0(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, "0px")}
    #[track_caller] pub fn border_b_1(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, "1px")}
    #[track_caller] pub fn border_b_2(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, "2px")}
    #[track_caller] pub fn border_b_4(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, "4px")}
    #[track_caller] pub fn border_b_8(&self) ->  Style  { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_width_property(BorderBottomWidth, "8px")}




    // Border Radius
    //
    #[track_caller] pub fn border_radius<T>(&self, val:T) -> Style  where T: Into<CssBorderRadius> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_radius_property(BorderRadius, val)}
    #[track_caller] pub fn radius<T>(&self, val:T) -> Style  where T: Into<CssBorderRadius> { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller()));new_style.set_border_radius_property(BorderRadius, val)}


    //
    // Css
    //

    #[track_caller] pub fn css(&self, val:&str) -> Style { let mut new_style = self.clone(); new_style.updated_at.push(format!("{}", Location::caller())); new_style.add_rule(Custom, CssValue::StringValue(val.to_string())); new_style }


    //
    // Display
    //


    #[track_caller] pub fn inline_block<'a>(&self) ->  Style {self.display (CssDisplay::InlineBlock)}


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

    fn add_rule(&mut self, property: Property, value: CssValue){
        self.rules.push(
            Rule {
                property,
                value,
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


    pub fn render(&self) -> String {
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

#[derive(Clone, Debug)]
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


#[derive(Default,Clone, Debug)]
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
        // log!(el.attrs);
        let rendered_css = self.render();


        let mut do_it = false;


        if let Some(AtValue::Some(class_string)) = el.attrs.vals.get_mut (&At::Class){

            // if there is a previous
            if let Some(_loc) = class_string.find("seedstyle-") {
                // let (_,last) = class_string.split_at_mut(loc);

                // let  id = if let Some(id_end_loc) = last.find(" ") {
                //     let (id, _) = last.split_at_mut(id_end_loc);
                //     id
                // } else {
                //     // the remaining string is the id that needs replacing

                //     last
                // };

                do_it = true;
            }

            // if need_to_replace {
            //     *class_string = class_string.replace(&replaceit,"");
            // }
        }


        if do_it {

                let styles_in_elem = STYLES_IN_ELEM.with(|styles|  {
                    styles.borrow_mut().push(self.clone());
                    styles.borrow().clone()
                });

                // let styles_slice = &styles_in_elem[..];

                // styles_slice.update_el(el);


                let vec_of_rendered_css = styles_in_elem.iter().map(|s| s.render()).collect::<Vec<String>>();

                let mut s = DefaultHasher::new();
                vec_of_rendered_css.hash(&mut s);
                let revised_variant_hash = s.finish();

                let css_aleady_created = STYLES_USED.with(|css_set_ref| css_set_ref.borrow().contains(&revised_variant_hash));


                if !css_aleady_created {

                    add_css_to_head_unchecked(&rendered_css, revised_variant_hash, &self, &self.name );

                }
                let short_hash = format!("{}{}", &self.name, short_uniq_id(revised_variant_hash));
                let class_name = format!("seedstyle-{}",short_hash);
                C![class_name].update_el(el);



            //



        } else {

        STYLES_IN_ELEM.with(|styles|  {
            styles.borrow_mut().clear();
            styles.borrow_mut().push(self.clone());
        });


        let variant_hash = hash_64(&rendered_css, &self.updated_at);

            let class_name = format!("seedstyle-{}",add_css_to_head(&rendered_css, variant_hash, &self));

            C![class_name].update_el(el);
        }
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
        let class_name = format!("seedstyle-{}",short_hash);

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
        format!("{}{{\n.seedstyle-{}{}{{\n{}}}}}\n",media, short_hash, style.pseudo.render() , css)
    }  else {
        format!("\n.seedstyle-{}{}{{\n{}}}\n", short_hash, style.pseudo.render() , css)
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
    STYLES_USED.with(|css_set_ref|
        css_set_ref.borrow_mut().insert(variant_hash));

    short_hash
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


