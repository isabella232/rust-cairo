/* automatically generated by rust-bindgen */
use libc::*;

extern mod freetype;
extern mod fontconfig;

use freetype::freetype::FT_Face;
use fontconfig::fontconfig::FcPattern;

use cairo::{
    cairo_font_face_t, cairo_scaled_font_t, cairo_font_options_t
};

#[link_name="cairo"]
pub extern mod bindgen {

pub fn cairo_ft_font_face_create_for_ft_face(++face: FT_Face, ++load_flags: c_int) -> *cairo_font_face_t;

pub fn cairo_ft_scaled_font_lock_face(++scaled_font: *cairo_scaled_font_t) -> FT_Face;

pub fn cairo_ft_scaled_font_unlock_face(++scaled_font: *cairo_scaled_font_t);

pub fn cairo_ft_font_face_create_for_pattern(++pattern: *FcPattern) -> *cairo_font_face_t;

pub fn cairo_ft_font_options_substitute(++options: *cairo_font_options_t, ++pattern: *FcPattern);

}
