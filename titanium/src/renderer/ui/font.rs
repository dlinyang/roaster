use glium::texture::Texture2d;
use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, Font, PositionedGlyph, Rect, Scale};
use crate::renderer::base::{Position,alias::Vec4f};

pub struct FontState<'a> {
    pub name: String,
    pub font: Font<'a>,
}

#[macro_export]
macro_rules! font {
    ($file: expr) => {
        let font_data = include!($file);
        let font = Font::from_bytes(font_data as &[u8]).unwrap();
        font
    };
}

pub struct Text {
    pub text: String,
    pub scale: Scale,
    pub width: u32,
    pub color: Vec4f,
}

use std::borrow::Cow;

fn layout_paragraph<'a>(font: &'a Font, scale: Scale, width: u32, text: &str) -> Vec<PositionedGlyph<'a>> {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                },
                '\n' => {},
                _ => {},
            }
            continue;
        }
        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}

pub fn load_text (text: Text,font: &Font, dpi: f32, display: &glium::Display) -> (Vec<Position>, Texture2d){

    let (w, h) = ((512.0 * dpi) as u32, (512.0 * dpi) as u32);

    let mut cache = Cache::builder().dimensions(w, h).build();

    let cache_tex = Texture2d::with_format(
        display,
        glium::texture::RawImage2d {
            data: Cow::Owned(vec![128u8; (w as usize) * (h as usize)]),
            width: w,
            height: h,
            format: glium::texture::ClientFormat::U8,
        },
        glium::texture::UncompressedFloatFormat::U8,
        glium::texture::MipmapsOption::NoMipmap,
    ).unwrap();

    let glyphs = layout_paragraph(font, text.scale, text.width, text.text.as_str());

    for glyph in &glyphs {
        cache.queue_glyph(0, glyph.clone());
    }

    cache.cache_queued(|rect, data| {
        cache_tex.main_level().write(
            glium::Rect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height(),
            },
            glium::texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: glium::texture::ClientFormat::U8,
            },
        )
    }).unwrap();
    
    let uniforms = uniform! {
        tex: cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
    };

    let (screen_width, screen_height) = {
        let (w,h) = display.get_framebuffer_dimensions();
        (w as f32, h as f32)
    };

    let origin = point(0.0, 0.0);

    let vertices: Vec<Position> = glyphs
        .iter()
        .flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: origin
                        + (vector(
                            screen_rect.min.x as f32 / screen_width - 0.5,
                            1.0 - screen_rect.min.y as f32 / screen_height - 0.5
                            )) * 2.0,
                    max: origin
                        + (vector(
                            screen_rect.max.x as f32 / screen_width - 0.5, 
                            1.0 - screen_rect.max.y as f32 / screen_height - 0.5
                            )) * 2.0
                };

                vec![Position::new([gl_rect.min.x,gl_rect.max.y], [uv_rect.min.x,uv_rect.max.y]),
                     Position::new([gl_rect.min.x,gl_rect.min.y], [uv_rect.min.x,uv_rect.min.y]),
                     Position::new([gl_rect.max.x,gl_rect.min.y], [uv_rect.max.x,uv_rect.min.y]),
                     Position::new([gl_rect.max.x,gl_rect.max.y], [uv_rect.max.x,uv_rect.max.y])]
            } 
            else {
                Vec::new()
            }
        })
        .collect();

        (vertices,cache_tex)
}