mod _utils;
use _utils::*;

mod aa_test;
mod component_rendering_000;
mod component_rendering_128;
mod component_rendering_255;
mod lion;
mod lion_cw;
mod lion_cw_aa;
mod lion_cw_aa_srgba;
mod lion_outline;
mod lion_outline_width1;
mod lion_png;
mod outline;
mod outline_aa;
mod rasterizers;
mod rasterizers2;
mod rasterizers2_pre; // can use freetype
mod rasterizers_gamma;
mod rounded_rect;
mod t00_example;
mod t01_rendering_buffer;
mod t02_pixel_formats;
mod t03_solar_spectrum;
mod t04_solar_spectrum_alpha;
mod t05_solar_spectrum_alpha;
mod t11;
mod t12;
mod t13;
mod t14;
mod t15;
mod t16;
mod t21_line_join;
mod t22_inner_join;
#[cfg(feature = "freetype")]
mod t23_font;
