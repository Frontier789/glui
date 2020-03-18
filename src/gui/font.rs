extern crate font_kit;
extern crate rusttype;
extern crate unicode_normalization;

use self::font_kit::error::*;
use self::font_kit::family_name::*;
use self::font_kit::handle::Handle;
use self::font_kit::properties::*;
use self::font_kit::source::SystemSource;
use self::rusttype::gpu_cache;
use self::unicode_normalization::UnicodeNormalization;

use std::collections::HashMap;
use std::fs::File;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::sync::Arc;
use tools::RgbaTexture;
use tools::Rect;
use tools::Vec2;

#[derive(Debug)]
pub enum FontLoaderError {
    IoError(IoError),
    NotFound,
    UnrecognizedFormat,
    IllFormed,
}

impl From<IoError> for FontLoaderError {
    fn from(e: IoError) -> FontLoaderError {
        FontLoaderError::IoError(e)
    }
}

impl From<FontError> for FontLoaderError {
    fn from(e: FontError) -> FontLoaderError {
        match e {
            FontError::IoError(e) => e.into(),
            FontError::RusttypeError(e) => e.into(),
        }
    }
}

impl From<rusttype::Error> for FontLoaderError {
    fn from(e: rusttype::Error) -> FontLoaderError {
        match e {
            rusttype::Error::IllFormed => FontLoaderError::IllFormed,
            rusttype::Error::UnrecognizedFormat => FontLoaderError::UnrecognizedFormat,
            _ => FontLoaderError::NotFound,
        }
    }
}

pub struct FontLoader {
    fkit_srouce: SystemSource,
    file_cache: HashMap<PathBuf, rusttype::FontCollection<'static>>,
    font_cache: HashMap<(PathBuf, usize), Font>,
    name_cache: HashMap<String, (PathBuf, usize)>,
    mem_cache:  HashMap<String, Font>,
}

impl FontLoader {
    pub fn new() -> FontLoader {
        FontLoader {
            fkit_srouce: SystemSource::new(),
            file_cache: HashMap::new(),
            font_cache: HashMap::new(),
            name_cache: HashMap::new(),
            mem_cache:  HashMap::new(),
        }
    }
    pub fn all_families(&self) -> Vec<String> {
        self.fkit_srouce.all_families().unwrap()
    }
    fn to_family_name(&self, name: String) -> Result<font_kit::handle::Handle, SelectionError> {
        let props = Properties::new();
        let fkit_name = if name == "serif".to_owned() {
            FamilyName::Serif
        } else if name == "sans".to_owned() || name == "sans-serif".to_owned() || name == "sansserif".to_owned() {
            FamilyName::SansSerif
        } else if name == "mono".to_owned() || name == "monospace".to_owned() {
            FamilyName::Monospace
        } else if name == "cursive".to_owned() {
            FamilyName::Cursive
        } else {
            FamilyName::Title(name.clone())
        };
        let mut result = self.fkit_srouce.select_best_match(&[fkit_name], &props);
        if let Err(_) = result {
            let families = self.all_families();
            for fam in families {
                if fam.to_lowercase().contains(&name) {
                    result = self
                        .fkit_srouce
                        .select_best_match(&[FamilyName::Title(fam)], &props);
                    break;
                }
            }
        }
        result
    }
    pub fn font_file(&mut self, name: &str) -> Option<String> {
        let name = name.to_lowercase();
        let result = self.to_family_name(name);
        match result {
            Ok(Handle::Path { path, .. }) => Some(path.to_str().unwrap().to_owned()),
            _ => None,
        }
    }
    fn vec_to_collection(data: Vec<u8>) -> Result<rusttype::FontCollection<'static>, rusttype::Error> {
        let arc_data: Arc<[u8]> = data.into();
        
        Ok(rusttype::FontCollection::from_bytes(arc_data)?)
    }
    
    pub fn font_family<'a>(&'a mut self, name: &str) -> Result<&'a mut Font, FontLoaderError> {
        let name = name.to_lowercase();
        if self.name_cache.contains_key(&name) {
            let entry = &self.name_cache[&name];
            return Ok(self.font_cache.get_mut(entry).unwrap());
        }
        
        if self.mem_cache.contains_key(&name) {
            return Ok(self.mem_cache.get_mut(&name).unwrap());
        }
    
        let result = self.to_family_name(name.clone());
        match result {
            Ok(p) => {
                let font = match p {
                    Handle::Path { path, font_index } => {
                        if self.font_cache.contains_key(&(path.clone(), font_index as usize)) {
                            self.name_cache.insert(name, (path.clone(), font_index as usize));
                            return Ok(self.font_cache.get_mut(&(path, font_index as usize)).unwrap());
                        } else {
                            if !self.file_cache.contains_key(&path) {
                                let data = std::fs::read(path.clone())?;
                                let collection = FontLoader::vec_to_collection(data)?;
                                self.file_cache.insert(path.clone(), collection);
                            }
                            
                            let font = Font::from_collection(&self.file_cache[&path], font_index as usize)?;
                            
                            self.font_cache.insert((path.clone(), font_index as usize), font);
                            self.font_cache.get_mut(&(path, font_index as usize)).unwrap()
                        }
                    }
                    Handle::Memory { bytes, font_index } => {
                        let data = Vec::<u8>::clone(&bytes);
                        let collection = FontLoader::vec_to_collection(data)?;
                        let font = Font::from_collection(&collection, font_index as usize)?;
                        
                        self.mem_cache.insert(name.clone(), font);
                        self.mem_cache.get_mut(&name).unwrap()
                    }
                };
                
                Ok(font)
            }
            Err(_) => Err(FontLoaderError::NotFound),
        }
    }
    pub fn font_exists(&mut self, name: &str) -> bool {
        let name = name.to_lowercase();
        
        self.name_cache.contains_key(&name) || self.mem_cache.contains_key(&name) || self.to_family_name(name.clone()).is_ok()
    }
}

pub struct Font {
    rt_font: rusttype::Font<'static>,
    cache: gpu_cache::Cache<'static>,
    pub tex: RgbaTexture,
}

#[derive(Debug)]
enum FontError {
    RusttypeError(rusttype::Error),
    IoError(std::io::Error),
}

impl From<std::io::Error> for FontError {
    fn from(e: std::io::Error) -> FontError {
        FontError::IoError(e)
    }
}

impl From<rusttype::Error> for FontError {
    fn from(e: rusttype::Error) -> FontError {
        FontError::RusttypeError(e)
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug,PartialEq,Eq)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

type Align = (HAlign, VAlign);

impl Font {
    fn from_collection(collection: &rusttype::FontCollection<'static>, index: usize) -> Result<Font, FontError> {
        Ok(Font {
            rt_font: collection.font_at(index)?,
            cache: gpu_cache::Cache::builder().dimensions(1024, 1024).build(),
            tex: RgbaTexture::new(1024, 1024),
        })
    }

    pub fn layout_paragraph(&mut self, text: &str, scale_x: f32, scale_y: f32, align: Align, size: Vec2) -> (Vec<Rect>, Vec<Rect>) {
        let scale = rusttype::Scale { x: scale_x, y: scale_y };
        let mut glyphs = Vec::new();
        let v_metrics = self.rt_font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = rusttype::point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;
        let mut row_widths: Vec<f32> = vec![0.0];
        
        for c in text.nfc() {
            if c.is_control() {
                match c {
                    '\n' => {
                        caret = rusttype::point(0.0, caret.y + advance_height);
                        row_widths.push(0.0);
                    }
                    _ => {}
                }
                continue;
            }
            let base_glyph = self.rt_font.glyph(c);
            if let Some(id) = last_glyph_id.take() {
                caret.x += self.rt_font.pair_kerning(scale, id, base_glyph.id());
            }
            last_glyph_id = Some(base_glyph.id());
            let mut glyph = base_glyph.scaled(scale).positioned(caret);
            if let Some(bb) = glyph.pixel_bounding_box() {
                if bb.max.x > size.x as i32 {
                    caret = rusttype::point(0.0, caret.y + advance_height);
                    glyph.set_position(caret);
                    last_glyph_id = None;
                }
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            let w = row_widths.last_mut().unwrap();
            *w = caret.x; 
            glyphs.push(glyph);
        }
        
        let all_height = caret.y;
        
        let mut cur_y = v_metrics.ascent;
        let mut row_i = 0;
        
        for g in &mut glyphs {
            if g.position().y != cur_y {
                row_i += 1;
                cur_y = g.position().y;
            }
            
            let w = row_widths[row_i];
            let p_old = g.position();
            let mut p = g.position();
            
            match align.0 {
                HAlign::Left => {},
                HAlign::Center => p.x += (size.x - w) / 2.0,
                HAlign::Right => p.x += size.x - w,
            };
            
            match align.1 {
                VAlign::Top => {},
                VAlign::Center => p.y += (size.y - all_height) / 2.0,
                VAlign::Bottom => p.y += size.y - all_height,
            };
            
            if p_old != p {
                g.set_position(p);
            }
        }
        
        
        let cache = &mut self.cache;
        let tex = &mut self.tex;
        
        for g in &glyphs {
            cache.queue_glyph(0, g.clone());
        }
        
        cache
            .cache_queued(|rect, data| {
                let mut data_a: Vec<u8> = vec![255u8; data.len()*4];
                for i in 0..data.len() {
                    data_a[i*4+3] = data[i];
                }
                
                tex.update_u8(
                    rect.min.x as usize,
                    rect.min.y as usize,
                    (rect.max.x - rect.min.x) as usize,
                    (rect.max.y - rect.min.y) as usize,
                    gl::RGBA,
                    data_a.as_ptr(),
                );
            })
            .unwrap();
        
        let mut uv_rects = Vec::<Rect>::with_capacity(glyphs.len());
        let mut bb_rects = Vec::<Rect>::with_capacity(glyphs.len());
        
        for g in glyphs {
            let (uv,bb) = cache.rect_for(0, &g).unwrap().unwrap();
            
            uv_rects.push(Rect{left: uv.min.x, right: uv.max.x, up: uv.min.y, down: uv.max.y, });
            bb_rects.push(Rect{left: bb.min.x as f32, right: bb.max.x as f32, up: bb.min.y as f32, down: bb.max.y as f32, });
        }
        
        (bb_rects, uv_rects)
    }
}