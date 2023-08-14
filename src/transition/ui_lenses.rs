use crate::{define_lens, define_lens_transparent, transition::prelude::*};
use bevy::prelude::*;

define_lens!(StyleWidthLens, Style, Val, width);

define_lens!(StyleHeightLens, Style, Val, height);

define_lens!(StyleTopLens, Style, Val, top);

define_lens!(StyleBottomLens, Style, Val, bottom);

define_lens!(StyleLeftLens, Style, Val, left);
define_lens!(StyleRightLens, Style, Val, right);

define_lens_transparent!(BackgroundColorLens, BackgroundColor, Color);
define_lens_transparent!(BorderColorLens, BorderColor, Color);

define_lens!(TextStyleColorLens, TextStyle, Color, color);

pub type TextColorLens<const SECTION: usize> = Prism2<TextStyleLens<SECTION>, TextStyleColorLens>;

#[derive(Debug, Clone)]
pub struct TextStyleLens<const SECTION: usize>;

impl<const SECTION: usize> Lens for TextStyleLens<SECTION> {
    type Object = Text;
    type Value = TextStyle;
}

impl<const SECTION: usize> GetRefLens for TextStyleLens<SECTION> {
    fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
        object.sections.get(SECTION).map(|x| &x.style)
    }
}

impl<const SECTION: usize> GetMutLens for TextStyleLens<SECTION> {
    fn try_get_mut(object: &mut Self::Object) -> Option<&mut Self::Value> {
        object.sections.get_mut(SECTION).map(|x| &mut x.style)
    }
}
