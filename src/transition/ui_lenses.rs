use crate::{define_lens, define_lens_transparent};
use bevy::prelude::*;

define_lens!(StyleWidthLens, Style, Val, width);

define_lens!(StyleHeightLens, Style, Val, height);

define_lens!(StyleTopLens, Style, Val, top);

define_lens!(StyleBottomLens, Style, Val, bottom);

define_lens!(StyleLeftLens, Style, Val, left);
define_lens!(StyleRightLens, Style, Val, right);

define_lens_transparent!(BackgroundColorLens, BackgroundColor, Color);
define_lens_transparent!(BorderColorLens, BorderColor, Color);
