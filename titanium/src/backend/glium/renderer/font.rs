use glium::texture::Texture2d;
use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, Font, PositionedGlyph, Rect, Scale};
use crate::base::{Position,alias::Vec4f};

#[macro_export]
macro_rules! font {
    ($file: expr) => {
        let font_data = include_byte!($file);
        let font = Font::from_bytes(font_data as &[u8]).unwrap();
        font
    };
}

use crate::canvas::text::Text;

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

use super::super::LayerData;
pub fn load_text (text: &Text,font: &Font, display: &glium::Display, name: String) -> LayerData {
    let scale = display.gl_window().window().scale_factor();
    let (width,_):(f32,_) = display.gl_window().window().inner_size().into();
    let (w, h) = ((512.0 * scale) as u32, (512.0 * scale) as u32);
    let mut cache = Cache::builder().dimensions(w, h).build();

    let cache_tex = Texture2d::with_format(
        display,
        glium::texture::RawImage2d {
            data: Cow::Owned(vec![0u8; (w as usize) * (h as usize)]),
            width: w,
            height: h,
            format: glium::texture::ClientFormat::U8,
        },
        glium::texture::UncompressedFloatFormat::U8,
        glium::texture::MipmapsOption::NoMipmap,
    ).unwrap();

    let glyphs = layout_paragraph(font, 
                                  Scale{
                                      x:text.size.width, 
                                      y: text.size.height
                                    }, 
                                  (text.width * 2.0 * width) as u32, 
                                  text.context.as_str());

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
    
    let (screen_width, screen_height) = {
        let (w,h) = display.get_framebuffer_dimensions();
        (w as f32, h as f32)
    };

    let origin = point(text.position[0], -text.position[1]*2.0);
    let vertices: Vec<Position> = glyphs
        .iter()
        .flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: origin
                        + (vector(
                            (screen_rect.min.x as f32 / screen_width) - 0.5,
                            1.0 - (screen_rect.min.y as f32 / screen_height) - 0.5
                            )) * 2.0,
                    max: origin
                        + (vector(
                            screen_rect.max.x as f32 / screen_width - 0.5, 
                            1.0 - screen_rect.max.y as f32 / screen_height - 0.5
                            )) * 2.0,
                };
                vec![Position::new([gl_rect.min.x,gl_rect.min.y], [uv_rect.min.x,uv_rect.min.y]),
                     Position::new([gl_rect.min.x,gl_rect.max.y], [uv_rect.min.x,uv_rect.max.y]),
                     Position::new([gl_rect.max.x,gl_rect.max.y], [uv_rect.max.x,uv_rect.max.y]),
                     Position::new([gl_rect.max.x,gl_rect.min.y], [uv_rect.max.x,uv_rect.min.y])]
            } 
            else {
                Vec::new()
            }
        })
        .collect();

        let len = vertices.len() as u32;
        let mut index_buffer = Vec::new();

        let mut x = 0;
        while x < len {
            index_buffer.append(&mut vec![x,x+1,x+2,x,x+2,x+3]);
            x = x + 4;
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_buffer).unwrap();
        use super::super::shader::ShaderSource;
        
        let shader = ShaderSource{
            vertex: VERTEX.to_string(),
            fragment: FRAGMENT.to_string(),
            geometry: None,
        }.bind_shader(display);
        
        LayerData {
            name,
            vertex_buffer,
            texture: Some(cache_tex),
            indices,
            shader,
            color: text.color,
        }
}

const VERTEX: &'static str = "#version 460

                              in layout(location = 0) vec2 position;
                              in layout(location = 1) vec2 tex_coordinate;
                              
                              out vec2 v_tex_coordinate;
                              
                              void main() {
                                  gl_Position = vec4(position, 0.0, 1.0);
                                  v_tex_coordinate = tex_coordinate;
                              }";
                              
const FRAGMENT: &'static str = "#version 460

                                uniform sampler2D tex;
                                uniform vec4 color;
                                
                                in vec2 v_tex_coordinate;
                                
                                out vec4 f_color;
                                
                                void main() {
                                    f_color = color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coordinate).r);
                                }";