use super::measures::*;
use crate::style::theme::*;
use tw_seed_hooks::{ generate_display_for_values};
#[derive(Clone, Debug)]
pub enum CssValue {
    // Themeable and shared s, i.e. Color is used for all Css color properties
    Color(CssColor),
    Size(CssSize),
    Border(CssBorder),
    BorderWidth(CssBorderWidth),
    BorderStyle(CssBorderStyle),
    Space(CssSpace),
    Transition(CssTransition),
    LineHeight(CssLineHeight),
    LetterSpacing(CssLetterSpacing),
    BorderRadius(CssBorderRadius),
    Font(CssFont),
    FontSize(CssFontSize),
    ZIndex(CssZIndex),
     // Back up for any property value not covered
    StringValue(String),

    // 'Single Property s, those values that apply to one property only.
    FontFamily(CssFontFamily),
    Display(CssDisplay),
    AlignContent(CssAlignContent),
    AlignItems(CssAlignItems),
    AlignSelf(CssAlignSelf),
    AnimationDelay(CssAnimationDelay),
    AnimationDirection(CssAnimationDirection),
    AnimationDuration(CssAnimationDuration),
    AnimationFillMode(CssAnimationFillMode),
    AnimationIterationCount(CssAnimationIterationCount),
    AnimationName(CssAnimationName),
    AnimationPlayState(CssAnimationPlayState),
    AnimationTimingFunction(CssAnimationTimingFunction),
    Animation(CssAnimation),
    BackfaceVisibility(CssBackfaceVisibility),
    Background(CssBackground),
    BackgroundAttachment(CssBackgroundAttachment),
    BackgroundBlendMode(CssBackgroundBlendMode),
    BackgroundClip(CssBackgroundClip),
    BackgroundColor(CssBackgroundColor),
    BackgroundImage(CssBackgroundImage),
    BackgroundOrigin(CssBackgroundOrigin),
    BackgroundPosition(CssBackgroundPosition),
    BackgroundRepeat(CssBackgroundRepeat),
    BackgroundSize(CssBackgroundSize),
    BorderCollapse(CssBorderCollapse),
    BorderImage(CssBorderImage),
    BorderImageOutset(CssBorderImageOutset),
    BorderImageRepeat(CssBorderImageRepeat),
    BorderImageSlice(CssBorderImageSlice),
    BorderImageSource(CssBorderImageSource),
    BorderImageWidth(CssBorderImageWidth),
    Bottom(CssBottom),
    BoxDecorationBreak(CssBoxDecorationBreak),
    BoxShadow(CssBoxShadow),
    BoxSizing(CssBoxSizing),
    BreakAfter(CssBreakAfter),
    BreakBefore(CssBreakBefore),
    BreakInside(CssBreakInside),
    CaptionSide(CssCaptionSide),
    CaretColor(CssCaretColor),
    Clear(CssClear),
    Clip(CssClip),
    ClipPath(CssClipPath),
    ColumnCount(CssColumnCount),
    ColumnFill(CssColumnFill),
    ColumnGap(CssColumnGap),
    ColumnRule(CssColumnRule),
    ColumnRuleColor(CssColumnRuleColor),
    ColumnRuleStyle(CssColumnRuleStyle),
    ColumnRuleWidth(CssColumnRuleWidth),
    ColumnSpan(CssColumnSpan),
    ColumnWidth(CssColumnWidth),
    Columns(CssColumns),
    Content(CssContent),
    CounterIncrement(CssCounterIncrement),
    CounterReset(CssCounterReset),
    Direction(CssDirection),
    EmptyCells(CssEmptyCells),
    Filter(CssFilter),
    Flex(CssFlex),
    FlexBasis(CssFlexBasis),
    FlexDirection(CssFlexDirection),
    FlexFlow(CssFlexFlow),
    FlexGrow(CssFlexGrow),
    FlexShrink(CssFlexShrink),
    FlexWrap(CssFlexWrap),
    Float(CssFloat),
    FontWeight(CssFontWeight),
    FontFeatureSettings(CssFontFeatureSettings),
    FontKerning(CssFontKerning),
    FontLanguageOverride(CssFontLanguageOverride),
    FontSizeAdjust(CssFontSizeAdjust),
    FontStretch(CssFontStretch),
    FontSynthesis(CssFontSynthesis),
    FontVariant(CssFontVariant),
    FontVariantAlternates(CssFontVariantAlternates),
    FontVariantCaps(CssFontVariantCaps),
    FontVariantEastAsian(CssFontVariantEastAsian),
    FontVariantLigatures(CssFontVariantLigatures),
    FontVariantNumeric(CssFontVariantNumeric),
    FontVariantPosition(CssFontVariantPosition),
    Grid(CssGrid),
    GridArea(CssGridArea),
    GridAutoColumns(CssGridAutoColumns),
    GridAutoFlow(CssGridAutoFlow),
    GridAutoRows(CssGridAutoRows),
    GridColumn(CssGridColumn),
    GridColumnEnd(CssGridColumnEnd),
    GridColumnGap(CssGridColumnGap),
    GridColumnStart(CssGridColumnStart),
    GridRow(CssGridRow),
    GridRowEnd(CssGridRowEnd),
    GridRowStart(CssGridRowStart),
    GridTemplate(CssGridTemplate),
    GridTemplateAreas(CssGridTemplateAreas),
    GridTemplateColumns(CssGridTemplateColumns),
    GridTemplateRows(CssGridTemplateRows),
    Hyphens(CssHyphens),
    ImageRendering(CssImageRendering),
    Isolation(CssIsolation),
    JustifyContent(CssJustifyContent),
    Left(CssLeft),
    LineBreak(CssLineBreak),
    Mask(CssMask),
    MaskType(CssMaskType),
    MixBlendMode(CssMixBlendMode),
    ObjectFit(CssObjectFit),
    ObjectPosition(CssObjectPosition),
    Opacity(CssOpacity),
    Order(CssOrder),
    Orphans(CssOrphans),
    Overflow(CssOverflow),
    OverflowWrap(CssOverflowWrap),
    OverflowX(CssOverflowX),
    OverflowY(CssOverflowY),
    PageBreakAfter(CssPageBreakAfter),
    PageBreakBefore(CssPageBreakBefore),
    PageBreakInside(CssPageBreakInside),
    Perspective(CssPerspective),
    PerspectiveOrigin(CssPerspectiveOrigin),
    PlaceContent(CssPlaceContent),
    PointerEvents(CssPointerEvents),
    Position(CssPosition),
    Quotes(CssQuotes),
    Resize(CssResize),
    Right(CssRight),
    ScrollBehavior(CssScrollBehavior),
    ShapeImageThreshold(CssShapeImageThreshold),
    ShapeMargin(CssShapeMargin),
    TabSize(CssTabSize),
    TableLayout(CssTableLayout),
    TextAlign(CssTextAlign),
    TextAlignLast(CssTextAlignLast),
    TextCombineUpright(CssTextCombineUpright),
    TextDecoration(CssTextDecoration),
    TextDecorationColor(CssTextDecorationColor),
    TextDecorationLine(CssTextDecorationLine),
    TextDecorationStyle(CssTextDecorationStyle),
    TextEmphasis(CssTextEmphasis),
    TextEmphasisColor(CssTextEmphasisColor),
    TextEmphasisPosition(CssTextEmphasisPosition),
    TextEmphasisStyle(CssTextEmphasisStyle),
    TextIndent(CssTextIndent),
    TextJustify(CssTextJustify),
    TextOrientation(CssTextOrientation),
    TextOverflow(CssTextOverflow),
    TextShadow(CssTextShadow),
    TextTransform(CssTextTransform),
    TextUnderlinePosition(CssTextUnderlinePosition),
    Top(CssTop),
    TouchAction(CssTouchAction),
    Transform(CssTransform),
    TransformOrigin(CssTransformOrigin),
    TransformStyle(CssTransformStyle),
    TransitionDelay(CssTransitionDelay),
    TransitionDuration(CssTransitionDuration),
    TransitionProperty(CssTransitionProperty),
    TransitionTimingFunction(CssTransitionTimingFunction),
    UnicodeBidi(CssUnicodeBidi),
    UserSelect(CssUserSelect),
    VerticalAlign(CssVerticalAlign),
    Visibility(CssVisibility),
    WhiteSpace(CssWhiteSpace),
    Widows(CssWidows),
    Width(CssWidth),
    WillChange(CssWillChange),
    WordBreak(CssWordBreak),
    WordSpacing(CssWordSpacing),
    WordWrap(CssWordWrap),
    WritingMode(CssWritingMode),
}

impl std::fmt::Display for CssValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssValue::AnimationDelay(val)=>  write!(f, "{}", val),
            CssValue::AnimationDirection(val)=>  write!(f, "{}", val),
            CssValue::AnimationDuration(val)=>  write!(f, "{}", val),
            CssValue::AnimationFillMode(val)=>  write!(f, "{}", val),
            CssValue::AnimationIterationCount(val)=>  write!(f, "{}", val),
            CssValue::AnimationName(val)=>  write!(f, "{}", val),
            CssValue::AnimationPlayState(val)=>  write!(f, "{}", val),
            CssValue::AnimationTimingFunction(val)=>  write!(f, "{}", val),
            CssValue::Border(val)=>  write!(f, "{}", val),
            CssValue::BorderRadius(val)=>  write!(f, "{}", val),
            CssValue::BorderStyle(val)=>  write!(f, "{}", val),
            CssValue::BorderWidth(val)=>  write!(f, "{}", val),
            CssValue::Color(val)=>  write!(f, "{}", val),
            CssValue::Font(val)=>  write!(f, "{}", val),
            CssValue::FontFamily(val)=>  write!(f, "{}", val),
            CssValue::FontSize(val)=>  write!(f, "{}", val),
            CssValue::LetterSpacing(val)=>  write!(f, "{}", val),
            CssValue::LineHeight(val)=>  write!(f, "{}", val),
            CssValue::Size(val)=>  write!(f, "{}", val),
            CssValue::Space(val)=>  write!(f, "{}", val),
            CssValue::StringValue(val) =>   write!(f, "{}", val),
            CssValue::Display(val) =>   write!(f, "{}", val),
            CssValue::AlignContent(val) => write!(f,"{}",val),
            CssValue::AlignItems(val) => write!(f,"{}",val),
            CssValue::AlignSelf(val) => write!(f,"{}",val),
            CssValue::Animation(val) => write!(f,"{}",val),
            CssValue::BackfaceVisibility(val) => write!(f,"{}",val),
            CssValue::Background(val) => write!(f,"{}",val),
            CssValue::BackgroundAttachment(val) => write!(f,"{}",val),
            CssValue::BackgroundBlendMode(val) => write!(f,"{}",val),
            CssValue::BackgroundClip(val) => write!(f,"{}",val),
            CssValue::BackgroundColor(val) => write!(f,"{}",val),
            CssValue::BackgroundImage(val) => write!(f,"{}",val),
            CssValue::BackgroundOrigin(val) => write!(f,"{}",val),
            CssValue::BackgroundPosition(val) => write!(f,"{}",val),
            CssValue::BackgroundRepeat(val) => write!(f,"{}",val),
            CssValue::BackgroundSize(val) => write!(f,"{}",val),
            CssValue::BorderCollapse(val) => write!(f,"{}",val),
            CssValue::BorderImage(val) => write!(f,"{}",val),
            CssValue::BorderImageOutset(val) => write!(f,"{}",val),
            CssValue::BorderImageRepeat(val) => write!(f,"{}",val),
            CssValue::BorderImageSlice(val) => write!(f,"{}",val),
            CssValue::BorderImageSource(val) => write!(f,"{}",val),
            CssValue::BorderImageWidth(val) => write!(f,"{}",val),
            CssValue::Bottom(val) => write!(f,"{}",val),
            CssValue::BoxDecorationBreak(val) => write!(f,"{}",val),
            CssValue::BoxShadow(val) => write!(f,"{}",val),
            CssValue::BoxSizing(val) => write!(f,"{}",val),
            CssValue::BreakAfter(val) => write!(f,"{}",val),
            CssValue::BreakBefore(val) => write!(f,"{}",val),
            CssValue::BreakInside(val) => write!(f,"{}",val),
            CssValue::CaptionSide(val) => write!(f,"{}",val),
            CssValue::CaretColor(val) => write!(f,"{}",val),
            CssValue::Clear(val) => write!(f,"{}",val),
            CssValue::Clip(val) => write!(f,"{}",val),
            CssValue::ClipPath(val) => write!(f,"{}",val),
            CssValue::ColumnCount(val) => write!(f,"{}",val),
            CssValue::ColumnFill(val) => write!(f,"{}",val),
            CssValue::ColumnGap(val) => write!(f,"{}",val),
            CssValue::ColumnRule(val) => write!(f,"{}",val),
            CssValue::ColumnRuleColor(val) => write!(f,"{}",val),
            CssValue::ColumnRuleStyle(val) => write!(f,"{}",val),
            CssValue::ColumnRuleWidth(val) => write!(f,"{}",val),
            CssValue::ColumnSpan(val) => write!(f,"{}",val),
            CssValue::ColumnWidth(val) => write!(f,"{}",val),
            CssValue::Columns(val) => write!(f,"{}",val),
            CssValue::Content(val) => write!(f,"{}",val),
            CssValue::CounterIncrement(val) => write!(f,"{}",val),
            CssValue::CounterReset(val) => write!(f,"{}",val),
            CssValue::Direction(val) => write!(f,"{}",val),
            CssValue::EmptyCells(val) => write!(f,"{}",val),
            CssValue::Filter(val) => write!(f,"{}",val),
            CssValue::Flex(val) => write!(f,"{}",val),
            CssValue::FlexBasis(val) => write!(f,"{}",val),
            CssValue::FlexDirection(val) => write!(f,"{}",val),
            CssValue::FlexFlow(val) => write!(f,"{}",val),
            CssValue::FlexGrow(val) => write!(f,"{}",val),
            CssValue::FlexShrink(val) => write!(f,"{}",val),
            CssValue::FlexWrap(val) => write!(f,"{}",val),
            CssValue::Float(val) => write!(f,"{}",val),
            CssValue::FontFeatureSettings(val) => write!(f,"{}",val),
            CssValue::FontWeight(val)=>  write!(f, "{}", val),
            CssValue::FontKerning(val) => write!(f,"{}",val),
            CssValue::FontLanguageOverride(val) => write!(f,"{}",val),
            CssValue::FontSizeAdjust(val) => write!(f,"{}",val),
            CssValue::FontStretch(val) => write!(f,"{}",val),
            CssValue::FontSynthesis(val) => write!(f,"{}",val),
            CssValue::FontVariant(val) => write!(f,"{}",val),
            CssValue::FontVariantAlternates(val) => write!(f,"{}",val),
            CssValue::FontVariantCaps(val) => write!(f,"{}",val),
            CssValue::FontVariantEastAsian(val) => write!(f,"{}",val),
            CssValue::FontVariantLigatures(val) => write!(f,"{}",val),
            CssValue::FontVariantNumeric(val) => write!(f,"{}",val),
            CssValue::FontVariantPosition(val) => write!(f,"{}",val),
            CssValue::Grid(val) => write!(f,"{}",val),
            CssValue::GridArea(val) => write!(f,"{}",val),
            CssValue::GridAutoColumns(val) => write!(f,"{}",val),
            CssValue::GridAutoFlow(val) => write!(f,"{}",val),
            CssValue::GridAutoRows(val) => write!(f,"{}",val),
            CssValue::GridColumn(val) => write!(f,"{}",val),
            CssValue::GridColumnEnd(val) => write!(f,"{}",val),
            CssValue::GridColumnGap(val) => write!(f,"{}",val),
            CssValue::GridColumnStart(val) => write!(f,"{}",val),
            CssValue::GridRow(val) => write!(f,"{}",val),
            CssValue::GridRowEnd(val) => write!(f,"{}",val),
            CssValue::GridRowStart(val) => write!(f,"{}",val),
            CssValue::GridTemplate(val) => write!(f,"{}",val),
            CssValue::GridTemplateAreas(val) => write!(f,"{}",val),
            CssValue::GridTemplateColumns(val) => write!(f,"{}",val),
            CssValue::GridTemplateRows(val) => write!(f,"{}",val),
            CssValue::Hyphens(val) => write!(f,"{}",val),
            CssValue::ImageRendering(val) => write!(f,"{}",val),
            CssValue::Isolation(val) => write!(f,"{}",val),
            CssValue::JustifyContent(val) => write!(f,"{}",val),
            CssValue::Left(val) => write!(f,"{}",val),
            CssValue::LineBreak(val) => write!(f,"{}",val),
            CssValue::Mask(val) => write!(f,"{}",val),
            CssValue::MaskType(val) => write!(f,"{}",val),
            CssValue::MixBlendMode(val) => write!(f,"{}",val),
            CssValue::ObjectFit(val) => write!(f,"{}",val),
            CssValue::ObjectPosition(val) => write!(f,"{}",val),
            CssValue::Opacity(val) => write!(f,"{}",val),
            CssValue::Order(val) => write!(f,"{}",val),
            CssValue::Orphans(val) => write!(f,"{}",val),
            CssValue::Overflow(val) => write!(f,"{}",val),
            CssValue::OverflowWrap(val) => write!(f,"{}",val),
            CssValue::OverflowX(val) => write!(f,"{}",val),
            CssValue::OverflowY(val) => write!(f,"{}",val),
            CssValue::PageBreakAfter(val) => write!(f,"{}",val),
            CssValue::PageBreakBefore(val) => write!(f,"{}",val),
            CssValue::PageBreakInside(val) => write!(f,"{}",val),
            CssValue::Perspective(val) => write!(f,"{}",val),
            CssValue::PerspectiveOrigin(val) => write!(f,"{}",val),
            CssValue::PlaceContent(val) => write!(f,"{}",val),
            CssValue::PointerEvents(val) => write!(f,"{}",val),
            CssValue::Position(val) => write!(f,"{}",val),
            CssValue::Quotes(val) => write!(f,"{}",val),
            CssValue::Resize(val) => write!(f,"{}",val),
            CssValue::Right(val) => write!(f,"{}",val),
            CssValue::ScrollBehavior(val) => write!(f,"{}",val),
            CssValue::ShapeImageThreshold(val) => write!(f,"{}",val),
            CssValue::ShapeMargin(val) => write!(f,"{}",val),
            CssValue::TabSize(val) => write!(f,"{}",val),
            CssValue::TableLayout(val) => write!(f,"{}",val),
            CssValue::TextAlign(val) => write!(f,"{}",val),
            CssValue::TextAlignLast(val) => write!(f,"{}",val),
            CssValue::TextCombineUpright(val) => write!(f,"{}",val),
            CssValue::TextDecoration(val) => write!(f,"{}",val),
            CssValue::TextDecorationColor(val) => write!(f,"{}",val),
            CssValue::TextDecorationLine(val) => write!(f,"{}",val),
            CssValue::TextDecorationStyle(val) => write!(f,"{}",val),
            CssValue::TextEmphasis(val) => write!(f,"{}",val),
            CssValue::TextEmphasisColor(val) => write!(f,"{}",val),
            CssValue::TextEmphasisPosition(val) => write!(f,"{}",val),
            CssValue::TextEmphasisStyle(val) => write!(f,"{}",val),
            CssValue::TextIndent(val) => write!(f,"{}",val),
            CssValue::TextJustify(val) => write!(f,"{}",val),
            CssValue::TextOrientation(val) => write!(f,"{}",val),
            CssValue::TextOverflow(val) => write!(f,"{}",val),
            CssValue::TextShadow(val) => write!(f,"{}",val),
            CssValue::TextTransform(val) => write!(f,"{}",val),
            CssValue::TextUnderlinePosition(val) => write!(f,"{}",val),
            CssValue::Top(val) => write!(f,"{}",val),
            CssValue::TouchAction(val) => write!(f,"{}",val),
            CssValue::Transform(val) => write!(f,"{}",val),
            CssValue::TransformOrigin(val) => write!(f,"{}",val),
            CssValue::TransformStyle(val) => write!(f,"{}",val),
            CssValue::Transition(val) => write!(f,"{}",val),
            CssValue::TransitionDelay(val) => write!(f,"{}",val),
            CssValue::TransitionDuration(val) => write!(f,"{}",val),
            CssValue::TransitionProperty(val) => write!(f,"{}",val),
            CssValue::TransitionTimingFunction(val) => write!(f,"{}",val),
            CssValue::UnicodeBidi(val) => write!(f,"{}",val),
            CssValue::UserSelect(val) => write!(f,"{}",val),
            CssValue::VerticalAlign(val) => write!(f,"{}",val),
            CssValue::Visibility(val) => write!(f,"{}",val),
            CssValue::WhiteSpace(val) => write!(f,"{}",val),
            CssValue::Widows(val) => write!(f,"{}",val),
            CssValue::Width(val) => write!(f,"{}",val),
            CssValue::WillChange(val) => write!(f,"{}",val),
            CssValue::WordBreak(val) => write!(f,"{}",val),
            CssValue::WordSpacing(val) => write!(f,"{}",val),
            CssValue::WordWrap(val) => write!(f,"{}",val),
            CssValue::WritingMode(val) => write!(f,"{}",val),
            CssValue::ZIndex(val) => write!(f,"{}",val),
        }
    }
}
#[derive(Clone, Debug)]
pub enum CssBackgroundAttachement {
    Scroll,
    Fixed,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssBackgroundImage{
    Uri(String),
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssBackgroundPosition {
    PercentX(Percent),
    LengthX(ExactLength),
    PercentY(Percent),
    LengthY(ExactLength),
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft, 
    BottomCenter,
    BottomRight,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssBackgroundRepeat{
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssBorderCollapse{
    Collapse,
    Separate,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssBorderSpacing {
    LengthX,
    LengthY,
    Inherit,
}
#[derive(Clone, Debug)]
pub enum CssPos {
    Length(ExactLength),
    Percent(Percent),
    Auto,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssCaptionSide {
    Top,
    Bottom,
    Inherit,
    StringValue(String),
}


#[derive(Clone, Debug)]
pub enum CssClear {
    None,
    Left,
    Right,
    Both,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssClip{
    Shape(String),
    Auto,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssContent {
    Normal,  
    None, 
    OpenQuote,
    CloseQuote,
    NoOpenQuote,
    NoCloseQuote,
    Inherit ,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssCounterIncrement{
    Identifier(String),
    Integer(i32),
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssCursor{
    Uri(String),
    Auto,
    Crosshair,
    Default,
    Pointer,
    Move,
    EResize,
    NeResize,
    NwResize,
    NResize,
    SeResize,
    SwResize,
    SResize,
    WResize,
    Text,
    Wait,
    Help,
    Progress,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssDirection {
    Ltr,
    Rtl,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssEmptyCells{
    Show,
    Hide,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssFloat {
    Left,
    Right,
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssFontStyle {
    Normal,
    Italic,
    Oblique,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssFontVariant{
    Normal,
    SmallCaps,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssFontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    V100,
    V200,
    V300,
    V400,
    V500,
    V600,
    V700,
    V800,
    V900,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssListStyleImage {
    Url(String),
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssListStylePosition {
    Inside,
    Outside,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssListStyleType {
    Disc,
    Circle,
    Decimal,
    DecimalLeadingZero,
    LowerRoman,
    UpperRoman,
    LowerGreek,
    LowerLatin,
    UpperLatin,
    Armenian,
    Georgian,
    LowerAlpha,
    UpperAlpha,
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssListStyle {
    ListStyle(CssListStyleType, CssListStylePosition,CssListStyleImage),
    StringValue(String),
    Inherit,
}
#[derive(Clone, Debug)]
pub enum CssOrphans {
    Integer(i32),
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssOverflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssPageBreak {
    Auto,
    Always,
    Avoid,
    Left,
    Right,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssPosition {
    Static,
    Relative,
    Absolulte,
    Fixed,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssQuotes{
    Quotes(String,String),
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssTableLayout {
    Auto,
    Fixed,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssTextAlign {
    Left,
    Right,
    Center,
    Justify,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssTextDecoration{
    None,
    Underline,
    Overline,
    LineThrough,
    Blink,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssTextIndent {
    Length(ExactLength),
    Percentage(Percent),
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssTextTransform {
    Capitalize,
    Uppercase,
    Lowercase,
    None,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssUnicodeBidi{
    Normal,
    Embed,
    BidiOverride,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssVerticalAlign{
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    Percentage(Percent),
    Length(ExactLength),
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssVisiblilty{
    Visible,
    Hiddem,
    Collapse,
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssWhiteSpace{
    Normal,
    Pre,
    NoWrap,
    PreWrap,
    PreLine,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssWidows {
    Integer(i32),
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssWordSpacing {
    Normal,
    Length(ExactLength),
    Inherit,
    StringValue(String),
}
#[derive(Clone, Debug)]
pub enum CssZIndex {
    Auto,
    Integer(i32),
    Inherit,
    StringValue(String),
}

 




#[derive(Clone, Debug)]
pub enum CssBorder{
    Border(CssBorderWidth, CssBorderStyle, CssColor),
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssBorderWidth{
    Medium,
    Thin,
    Thick,
    Length(ExactLength),
    Initial,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssBorderStyle{
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    Initial,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssSpace{
    Length(ExactLength),
    Percentage(Percent),
    Auto,
    Inherit,
    StringValue(String),
}


#[derive(Clone, Debug)]
pub enum CssTransition{
    StringValue(String),
    Inherit,
}


#[derive(Clone, Debug)]
pub enum CssLetterSpacing{
    Normal,
    Length(ExactLength),
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssLineHeight{
    Normal,
    Number(u32),
    Length(ExactLength),
    Percentage(Percent),
    Inherit,
    StringValue(String),
}


#[derive(Clone, Debug)]
pub enum CssBorderRadius{
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssFont{
    Font(CssFontStyle,CssFontVariant, CssFontWeight,CssFontSize,CssLineHeight,CssFontFamily),
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssFontSize{
    Size(ExactLength),
    Percentage(Percent),
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssSize {
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    Inherit,
    StringValue(String),
}

#[derive(Clone, Debug)]
pub enum CssColor {
    Rgba(f64,f64,f64,f64),
    Hex3(String),
    Hex6(String),
    StringValue(String),
    Inherit,
}


#[derive(Clone, Debug)] pub enum CssAnimationDelay{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationDirection{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationDuration{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationFillMode{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationIterationCount{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationName{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationPlayState{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimationTimingFunction{ Inherit, StringValue(String), }

#[derive(Clone, Debug)] pub enum CssAlignContent{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAlignItems{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAlignSelf{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssAnimation{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackfaceVisibility{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackground{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackgroundAttachment{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackgroundBlendMode{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackgroundClip{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackgroundColor{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackgroundOrigin{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBackgroundSize{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBorderImage{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBorderImageOutset{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBorderImageRepeat{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBorderImageSlice{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBorderImageSource{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBorderImageWidth{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBottom{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBoxDecorationBreak{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBoxShadow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBoxSizing{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBreakAfter{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBreakBefore{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssBreakInside{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssCaretColor{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssClipPath{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnCount{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnFill{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnGap{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnRule{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnRuleColor{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnRuleStyle{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnRuleWidth{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnSpan{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumnWidth{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssColumns{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssCounterReset{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFilter{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFlex{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFlexBasis{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFlexFlow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFlexDirection{ Inherit, StringValue(String),}
#[derive(Clone, Debug)] pub enum CssFlexGrow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFlexShrink{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFlexWrap{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontFamily{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontFeatureSettings{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontKerning{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontLanguageOverride{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontSizeAdjust{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontStretch{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontSynthesis{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontVariantAlternates{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontVariantCaps{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontVariantEastAsian{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontVariantLigatures{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontVariantNumeric{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssFontVariantPosition{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGrid{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridArea{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridAutoColumns{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridAutoFlow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridAutoRows{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridColumn{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridColumnEnd{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridColumnGap{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridColumnStart{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridRow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridRowEnd{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridRowStart{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridTemplate{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridTemplateAreas{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridTemplateColumns{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssGridTemplateRows{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssHyphens{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssImageRendering{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssIsolation{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssJustifyContent{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssLeft{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssLineBreak{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssMask{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssMaskType{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssMixBlendMode{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssObjectFit{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssObjectPosition{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssOpacity{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssOrder{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssOverflowWrap{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssOverflowX{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssOverflowY{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPageBreakAfter{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPageBreakBefore{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPageBreakInside{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPerspective{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPerspectiveOrigin{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPlaceContent{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssPointerEvents{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssResize{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssRight{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssScrollBehavior{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssShapeImageThreshold{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssShapeMargin{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTabSize{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextAlignLast{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextCombineUpright{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextDecorationColor{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextDecorationLine{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextDecorationStyle{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextEmphasis{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextEmphasisColor{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextEmphasisPosition{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextEmphasisStyle{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextJustify{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextOrientation{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextOverflow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextShadow{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTextUnderlinePosition{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTop{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTouchAction{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransform{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransformOrigin{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransformStyle{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransitionDelay{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransitionDuration{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransitionProperty{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssTransitionTimingFunction{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssUserSelect{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssVisibility{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssWidth{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssWillChange{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssWordBreak{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssWordWrap{ Inherit, StringValue(String), }
#[derive(Clone, Debug)] pub enum CssWritingMode{ Inherit, StringValue(String), }



#[derive(Clone, Debug)]
pub enum CssDisplay{
    Inline,
    Block,
    Contents,
    Flex,
    Grid,
    InlineBlock,
    InlineFlex,
    InlineGrid,
    InlineTable,
    ListItem,
    RunIn,
    Table,
    TableCaption,
    TableColumnGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRowGroup,
    TableCell,
    TableColumn,
    TableRow,
    None,
    Initial,
    Inherit,
    StringValue(String),
}



// pub trait BorderRadiusTrait{}
// pub trait BorderStyleTrait{}
// pub trait BorderTrait{}
// pub trait BorderWidthTrait{}
// pub trait ColorTrait{}
// pub trait DisplayTrait{}
// pub trait FontSizeTrait{}
// pub trait FontTrait{}
// pub trait FontWeightTrait{}
// pub trait LineHeightTrait{}
// pub trait LetterSpacingTrait{}
// pub trait SizeTrait{}
// pub trait SpaceTrait{}
// pub trait TransitionTrait{}
// pub trait ZIndexTrait{}

// impl BorderRadiusTrait for CssBorderRadius{}
// impl BorderStyleTrait for CssBorderStyle{}
// impl BorderTrait for CssBorder{}
// impl BorderWidthTrait for CssBorderWidth{}
// impl ColorTrait for CssColor{}
// impl DisplayTrait for CssDisplay{}
// impl FontSizeTrait for CssFontSize{}
// impl FontWeightTrait for CssFontWeight{}
// impl FontTrait for CssFont{}
// impl LetterSpacingTrait for CssLetterSpacing{}
// impl LineHeightTrait for CssLineHeight{}
// impl SizeTrait for CssFontSize{}
// impl SpaceTrait for CssSpace{}
// impl TransitionTrait for CssTransition{}
// impl ZIndexTrait for CssZIndex{}

// impl BorderRadiusTrait for &str{}
// impl BorderStyleTrait for &str{}
// impl BorderTrait for &str{}
// impl BorderWidthTrait for &str{}
// impl ColorTrait for &str{}
// impl DisplayTrait for &str{}
// impl FontSizeTrait for &str{}
// impl FontTrait for &str{}
// impl LetterSpacingTrait for &str{}
// impl LineHeightTrait for &str{}
// impl SizeTrait for &str{}
// impl SpaceTrait for &str{}
// impl TransitionTrait for &str{}
// impl ZIndexTrait for &str{}



// impl<T> BorderRadiusTrait for  (T, CssBorderRadius){}
// impl<T> BorderStyleTrait for  (T, CssBorderStyle){}
// impl<T> BorderTrait for  (T, CssBorder){}
// impl<T> BorderWidthTrait for  (T, CssBorderWidth){}
// impl<T> ColorTrait for  (T, CssColor){}
// impl<T> DisplayTrait for (T,  CssDisplay){}
// impl<T> FontSizeTrait for (T,  CssFontSize){}
// impl<T> FontWeightTrait for  (T, CssFontWeight){}
// impl<T> FontTrait for  (T, CssFont){}
// impl<T> LetterSpacingTrait for  (T, CssLetterSpacing){}
// impl<T> LineHeightTrait for (T,  CssLineHeight){}
// impl<T> SizeTrait for  (T, CssFontSize){}
// impl<T> SpaceTrait for  (T, CssSpace){}
// impl<T> TransitionTrait for (T,  CssTransition){}
// impl<T> ZIndexTrait for (T,  CssZIndex){}



impl From<ExactLength> for CssSize {
    fn from(length: ExactLength) -> Self {
        CssSize::Length(length)
    }
}

impl From<Percent> for CssSize {
    fn from(v:Percent) -> Self {
        CssSize::Percentage(v)
    }
}

impl From<&str> for CssSize {
    fn from(v:&str) -> Self {
        CssSize::StringValue(v.to_string())
    }
}


impl From<ExactLength> for CssSpace {
    fn from(length: ExactLength) -> Self {
        CssSpace::Length(length)
    }
}

impl From<Percent> for CssSpace {
    fn from(v:Percent) -> Self {
        CssSpace::Percentage(v)
    }
}

impl From<&str> for CssSpace {
    fn from(v:&str) -> Self {
        CssSpace::StringValue(v.to_string())
    }
}



impl From<&str> for CssColor {
    fn from(v:&str) -> Self {
        CssColor::StringValue(v.to_string())
    }
}




impl <T>From<(T,CssBorderWidth)> for CssBorderWidth where T: BorderWidthTheme + 'static {
    fn from(v:(T,CssBorderWidth)) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        if let Some(theme_value) = borrowed_theme.get::<T,CssBorderWidth>(v.0){
            theme_value.clone()
        } else {
            v.1
        }
    }
}

impl <T> From<T> for CssBorderWidth where T:BorderWidthTheme + 'static{
    fn from(v: T) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        borrowed_theme.get::<T,CssBorderWidth>(v).unwrap()
    }
}

impl From<ExactLength> for CssBorderWidth {
    fn from(length: ExactLength) -> Self {
        CssBorderWidth::Length(length)
    }
}

impl From<&str> for CssBorderWidth {
    fn from(v:&str) -> Self {
        CssBorderWidth::StringValue(v.to_string())
    }
}




impl <T>From<(T,CssBorderStyle)> for CssBorderStyle where T: BorderStyleTheme + 'static {
    fn from(v:(T,CssBorderStyle)) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        if let Some(theme_value) = borrowed_theme.get::<T,CssBorderStyle>(v.0){
            theme_value.clone()
        } else {
            v.1
        }
    }
}

impl <T> From<T> for CssBorderStyle where T:BorderStyleTheme + 'static{
    fn from(v: T) -> Self {
        let theme = get_theme();
        let borrowed_theme = theme.borrow();
        borrowed_theme.get::<T,CssBorderStyle>(v).unwrap()
    }
}


impl From<&str> for CssBorderStyle {
    fn from(v:&str) -> Self {
        CssBorderStyle::StringValue(v.to_string())
    }
}




impl std::fmt::Display for CssBorderRadius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Length(val) => write!(f,"{}", val),
            Self::Percentage(val) => write!(f,"{}", val),
            Self::Initial => write!(f,"initial"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }
    }
}


impl std::fmt::Display for CssZIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
        Self::Auto => write!(f,"auto"),
        Self::Integer(val) => write!(f,"{}",val),
        Self::Inherit => write!(f,"inherit"),
        Self::StringValue(val) => write!(f,"{}",val),
    }
}
}


impl std::fmt::Display for CssCounterIncrement{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(val)=> write!(f, "{}",val),
            Self::Integer(val)=> write!(f, "{}",val),
            Self::None=> write!(f, "none"),
            Self::Inherit=> write!(f, "inherit"),
            Self::StringValue(val)=> write!(f, "{}",val),
        }
    }
}
impl std::fmt::Display for CssTextIndent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Length(val)=> write!(f, "{}",val),
            Self::Percentage(val)=> write!(f, "{}",val),
            Self::Inherit=> write!(f, "inherit"),
            Self::StringValue(val)=> write!(f, "{}",val),
        }
    }
}

impl std::fmt::Display for CssTextDecoration{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineThrough=> write!(f, "line-though"),
            Self::Blink=> write!(f, "blink"),
            Self::Overline=> write!(f, "Overline"),
            Self::Underline=> write!(f, "underline"),
            Self::None=> write!(f, "none"),
            Self::Inherit=> write!(f, "inherit"),
            Self::StringValue(val)=> write!(f, "{}",val),
        }
    }
}

impl std::fmt::Display for CssQuotes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quotes(val,val2) => write!(f, "{} {}", val , val2),
            Self::None => write!(f, "none"),
            Self::Inherit => write!(f, "inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}
impl std::fmt::Display for CssSpace{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Length(val) => write!(f, "{}", val),
            Self::Percentage(val) => write!(f, "{}", val),
            Self::Auto => write!(f, "auto"),
            Self::Inherit => write!(f, "inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}
impl std::fmt::Display for CssBorderCollapse{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Collapse=> write!(f,"collapse"),
            Self::Separate=> write!(f,"separate"),
            Self::Inherit=> write!(f,"inherit"),
            Self::StringValue(val)=> write!(f,"{}",val),

        }
    }
}


impl std::fmt::Display for CssCaptionSide{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Top=> write!(f,"top"),
            Self::Bottom=> write!(f,"bottom"),
            Self::Inherit=> write!(f,"inherit"),
            Self::StringValue(val)=> write!(f,"{}",val),

        }
    }
}


impl std::fmt::Display for CssLetterSpacing{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Length(val) => write!(f, "{}", val),
            Self::Inherit => write!(f, "inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}


impl std::fmt::Display for CssBackgroundRepeat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Repeat=> write!(f,"repeat"),
            Self::RepeatX=> write!(f,"repeat-x"),
            Self::RepeatY=> write!(f,"repeat-y"),
            Self::NoRepeat=> write!(f,"no-repeat"),
            Self::Inherit=> write!(f,"inherit"),
            Self::StringValue(val)=> write!(f,"{}",val),

        }}}


impl std::fmt::Display for CssPosition{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     match self {
        Self::Static => write!(f, "static"),
        Self::Relative => write!(f, "relative"),
        Self::Absolulte => write!(f, "absolute"),
        Self::Fixed => write!(f, "fixed"),
        Self::Inherit => write!(f, "inherit"),
        Self::StringValue(val) => write!(f, "{}",val),
}}}

impl std::fmt::Display for CssBorderStyle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     match self {
        Self::None => write!(f,"none"),
        Self::Hidden => write!(f,"hidden"),
        Self::Dotted => write!(f,"dotted"),
        Self::Dashed => write!(f,"dashed"),
        Self::Solid => write!(f,"solid"),
        Self::Double => write!(f,"double"),
        Self::Groove => write!(f,"groove"),
        Self::Ridge => write!(f,"ridge"),
        Self::Inset => write!(f,"inset"),
        Self::Outset => write!(f,"outset"),
        Self::Initial => write!(f,"initial"),
        Self::Inherit => write!(f,"inherit"),
        Self::StringValue(val) => write!(f,"{}", val),
     }
    }
}

impl std::fmt::Display for CssBorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        Self::Border(bw, bs, col) => write!(f,"{} {} {}", bw,bs,col),
        Self::Inherit => write!(f,"inherit"),
        Self::StringValue(val) => write!(f,"{}", val),

    }
}
}

impl std::fmt::Display for CssBorderWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Medium =>  write!(f,"medium"),
                Self::Thin =>  write!(f,"thin"),
                Self::Thick =>  write!(f,"thick"),
                Self::Length(length) =>  write!(f,"{}", length),
                Self::Initial =>  write!(f,"initial"),
                Self::StringValue(val) => write!(f, "{}" ,val),
                Self::Inherit => write!(f,"inherit"),
            }

    }
}



impl std::fmt::Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rgba(a,b,c,d) => write!(f, "rgba({},{},{},{})",a,b,c,d),
            Self::Hex3(val) => write!(f, "{}", val),
            Self::Hex6(val) => write!(f, "{}" ,val),
            Self::StringValue(val) => write!(f, "{}" ,val),
            Self::Inherit => write!(f, "inherit"),
        }

    }
}


impl std::fmt::Display for CssDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Inline=> write!(f,"inline"),
                Self::Block=> write!(f,"block"),
                Self::Contents=> write!(f,"contents"),
                Self::Flex=> write!(f,"flex"),
                Self::Grid=> write!(f,"grid"),
                Self::InlineBlock=> write!(f,"inline-block"),
                Self::InlineFlex=> write!(f,"inline-flex"),
                Self::InlineGrid=> write!(f,"inline-grid"),
                Self::InlineTable=> write!(f,"inline-table"),
                Self::ListItem=> write!(f,"list-item"),
                Self::RunIn=> write!(f,"run-in"),
                Self::Table=> write!(f,"table"),
                Self::TableCaption=> write!(f,"table-caption"),
                Self::TableColumnGroup=> write!(f,"table-column-group"),
                Self::TableHeaderGroup=> write!(f,"table-header-group"),
                Self::TableFooterGroup=> write!(f,"table-footer-group"),
                Self::TableRowGroup=> write!(f,"table-row-group"),
                Self::TableCell=> write!(f,"table-cell"),
                Self::TableColumn=> write!(f,"table-column"),
                Self::TableRow=> write!(f,"table-row"),
                Self::None=> write!(f,"none"),
                Self::Initial=> write!(f,"initial"),
                Self::Inherit=> write!(f,"inherit"),
                Self::StringValue(val) => write!(f,"{}", val),
            }

    }
}


impl std::fmt::Display for CssFontSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Size(val) => write!(f, "{}", val),
            Self::Percentage(val) => write!(f, "{}", val),
            Self::Inherit => write!(f, "inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }

    }
}

impl std::fmt::Display for CssFontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f,"normal"),
            Self::Bold => write!(f,"bold"),
            Self::Bolder => write!(f,"bolder"),
            Self::Lighter => write!(f,"lighter"),
            Self::V100 => write!(f,"100"),
            Self::V200 => write!(f,"200"),
            Self::V300 => write!(f,"300"),
            Self::V400 => write!(f,"400"),
            Self::V500 => write!(f,"500"),
            Self::V600 => write!(f,"600"),
            Self::V700 => write!(f,"700"),
            Self::V800 => write!(f,"800"),
            Self::V900 => write!(f,"900"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}",val),
        }

    }
}


impl std::fmt::Display for CssFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Font(style, variant, weight, size, line_height, family) => write!(f,"{} {} {} {}/{} {}",style,variant,weight,size,line_height,family),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }

    }
}


impl std::fmt::Display for CssFontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
    Self::Normal => write!(f,"normal"),
    Self::Italic => write!(f,"italic"),
    Self::Oblique => write!(f,"oblique"),
    Self::Inherit => write!(f,"inherit"),
    Self::StringValue(val) => write!(f,"{}", val)
}}}


impl std::fmt::Display for CssClip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape(val) => write!(f,"{}",val),
            Self::Auto => write!(f,"auto"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }
    }
}

impl std::fmt::Display for CssTextAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f,"left"),
            Self::Right => write!(f,"right"),
            Self::Center => write!(f,"center"),
            Self::Justify => write!(f,"justify"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }
    }
}


impl std::fmt::Display for CssEmptyCells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Show => write!(f,"show"),
            Self::Hide => write!(f,"hide"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }
    }
}


impl std::fmt::Display for CssOrphans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(val) => write!(f,"{}", val),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }
    }
}

impl std::fmt::Display for CssOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Visible => write!(f,"visible"),
            Self::Hidden => write!(f,"hidden"),
            Self::Auto => write!(f,"auto"),
            Self::Scroll => write!(f,"scroll"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f,"{}", val),
        }
    }
}






impl std::fmt::Display for CssSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Self::Auto => write!(f,"auto"),
            Self::Length(length) => write!(f,"{}",length),
            Self::Percentage(pc) => write!(f,"{}",pc),
            Self::Initial => write!(f,"initial"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }

    }
}

impl std::fmt::Display for CssBackgroundImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Self::Uri(val) => write!(f,"{}",val),
            Self::None => write!(f,"none"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }

    }
}


impl std::fmt::Display for CssClear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Self::Left => write!(f,"left"),
            Self::Right => write!(f,"right"),
            Self::Both => write!(f,"both"),
            Self::None => write!(f,"none"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }

    }
}
impl std::fmt::Display for CssFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Self::Left => write!(f,"left"),
            Self::Right => write!(f,"right"),
            Self::None => write!(f,"none"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }

    }
}

impl std::fmt::Display for CssTextTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Self::Capitalize => write!(f,"capitalize"),
            Self::Uppercase => write!(f,"uppercase"),
            Self::Lowercase => write!(f,"lowercase"),
            Self::None => write!(f,"none"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }

    }
}



impl std::fmt::Display for CssContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f,"normal"),
            Self::None=> write!(f,"none"),
            Self::OpenQuote=> write!(f,"open-quote"),
            Self::CloseQuote=> write!(f,"close-quote"),
            Self::NoOpenQuote=> write!(f,"no-open-quote"),
            Self::NoCloseQuote=> write!(f,"no-close-quote"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val)=> write!(f,"{}", val),
        }
    }
}


impl std::fmt::Display for CssFontVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f,"normal"),
            Self::SmallCaps => write!(f,"small-caps"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val)=> write!(f,"{}", val),

        }

    }
}


impl std::fmt::Display for CssLineHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Length(val) => write!(f, "{}",val),
            Self::Percentage(val) => write!(f, "{}",val),
            Self::Inherit => write!(f, "inherit"),
            Self::Normal => write!(f,"normal"),
            Self::Number(f64) => write!(f,"{}", f64),
            Self::StringValue(val)=> write!(f,"{}", val),
        }

    }
}

impl std::fmt::Display for CssUnicodeBidi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f,"normal"),
            Self::Embed => write!(f,"embed"),
            Self::Inherit => write!(f, "inherit"),
            Self::BidiOverride => write!(f,"bidi-override"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}



impl std::fmt::Display for CssDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ltr => write!(f,"ltr"),
            Self::Rtl => write!(f,"rtl"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}


impl std::fmt::Display for CssTableLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f,"auto"),
            Self::Fixed => write!(f,"fixed"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}





impl std::fmt::Display for CssVerticalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Baseline => write!(f,"baseline"),
            Self::Sub => write!(f,"sub"),

           Self::Super => write!(f, "super"),
           Self::Top => write!(f, "top"),
           Self::TextTop => write!(f, "text-top"),
           Self::Middle => write!(f, "middle"),
           Self::Bottom => write!(f, "bottom"),
           Self::Percentage(val) => write!(f, "{}",val),
           Self::Length(val) => write!(f, "{}", val),
       
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}



impl std::fmt::Display for CssWordSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f,"normal"),
            Self::Length(val) => write!(f,"{}", val),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}


impl std::fmt::Display for CssWidows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(val)=> write!(f, "{}",val),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}

impl std::fmt::Display for CssWhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f,"normal"),
            Self::Pre => write!(f,"pre"),
            Self::PreWrap => write!(f,"pre-wrap"),
            Self::PreLine => write!(f,"pre-line"),
            Self::NoWrap => write!(f,"no-wrap"),
            Self::Inherit => write!(f,"inherit"),
            Self::StringValue(val) => write!(f, "{}", val),
        }
    }
}



impl std::fmt::Display for CssBackgroundPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
           Self::PercentX(val) => write!(f,"{}",val),
           Self::LengthX(val)=> write!(f,"{}",val),
           Self::PercentY(val)=> write!(f,"{}",val),
           Self::LengthY(val)=> write!(f,"{}",val),
           Self::TopLeft=> write!(f,"top left"),
           Self::TopCenter=> write!(f,"top eenter"),
           Self::TopRight=> write!(f,"top right"),
           Self::CenterLeft=> write!(f,"center left"),
           Self::Center=> write!(f,"center center"),
           Self::CenterRight=> write!(f,"top right"),
           Self::BottomLeft=> write!(f,"bottom left"),
           Self::BottomCenter=> write!(f,"bottom center"),
           Self::BottomRight=> write!(f,"bottom right"),
           Self::Inherit=> write!(f,"inherit"),
           Self::StringValue(val)=> write!(f,"{}",val),
        }

    }
}








generate_display_for_values!([
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
"BackgroundColor",
"BackgroundOrigin",
"BackgroundSize",
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
"CaretColor",
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
"CounterReset",
"Filter",
"Flex",
"FlexBasis",
"FlexDirection",
"FlexFlow",
"FlexGrow",
"FlexShrink",
"FlexWrap",
"FontFamily",
"FontFeatureSettings",
"FontKerning",
"FontLanguageOverride",
"FontSizeAdjust",
"FontStretch",
"FontSynthesis",
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
"Mask",
"MaskType",
"MixBlendMode",
"ObjectFit",
"ObjectPosition",
"Opacity",
"Order",
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
"Resize",
"Right",
"ScrollBehavior",
"ShapeImageThreshold",
"ShapeMargin",
"TabSize",
"TextAlignLast",
"TextCombineUpright",
"TextDecorationColor",
"TextDecorationLine",
"TextDecorationStyle",
"TextEmphasis",
"TextEmphasisColor",
"TextEmphasisPosition",
"TextEmphasisStyle",
"TextJustify",
"TextOrientation",
"TextOverflow",
"TextShadow",
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
"UserSelect",
"Visibility",
"Width",
"WillChange",
"WordBreak",
"WordWrap",
"WritingMode",



]);

