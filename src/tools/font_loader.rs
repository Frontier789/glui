use font_kit::error::*;
use font_kit::family_name::*;
use font_kit::handle::Handle;
use font_kit::properties::*;
use font_kit::source::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tools::{Font, FontLoaderError};

pub struct FontLoader {
    fkit_srouce: SystemSource,
    file_cache: HashMap<PathBuf, rusttype::FontCollection<'static>>,
    font_cache: HashMap<(PathBuf, usize), Font>,
    name_cache: HashMap<String, (PathBuf, usize)>,
    mem_cache: HashMap<String, Font>,
}

impl FontLoader {
    pub fn new() -> FontLoader {
        FontLoader {
            fkit_srouce: SystemSource::new(),
            file_cache: HashMap::new(),
            font_cache: HashMap::new(),
            name_cache: HashMap::new(),
            mem_cache: HashMap::new(),
        }
    }
    pub fn all_families(&self) -> Vec<String> {
        self.fkit_srouce.all_families().unwrap()
    }
    fn to_family_name(&self, name: &str) -> Result<font_kit::handle::Handle, SelectionError> {
        let props = Properties::new();
        let fkit_name = if name == "serif".to_owned() {
            FamilyName::Serif
        } else if name == "sans".to_owned()
            || name == "sans-serif".to_owned()
            || name == "sansserif".to_owned()
        {
            FamilyName::SansSerif
        } else if name == "mono".to_owned() || name == "monospace".to_owned() {
            FamilyName::Monospace
        } else if name == "cursive".to_owned() {
            FamilyName::Cursive
        } else {
            FamilyName::Title(name.to_owned())
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
        let name = &name.to_lowercase();
        let result = self.to_family_name(name);
        match result {
            Ok(Handle::Path { path, .. }) => Some(path.to_str().unwrap().to_owned()),
            _ => None,
        }
    }
    fn vec_to_collection(
        data: Vec<u8>,
    ) -> Result<rusttype::FontCollection<'static>, rusttype::Error> {
        let arc_data: Arc<[u8]> = data.into();

        Ok(rusttype::FontCollection::from_bytes(arc_data)?)
    }

    pub fn font_family(&mut self, name: &str) -> Result<&mut Font, FontLoaderError> {
        let name = &name.to_lowercase();
        if self.name_cache.contains_key(name) {
            let entry = &self.name_cache[name];
            return Ok(self.font_cache.get_mut(entry).unwrap());
        }

        if self.mem_cache.contains_key(name) {
            return Ok(self.mem_cache.get_mut(name).unwrap());
        }

        let result = self.to_family_name(name);
        match result {
            Ok(p) => {
                let font = match p {
                    Handle::Path { path, font_index } => {
                        if self
                            .font_cache
                            .contains_key(&(path.clone(), font_index as usize))
                        {
                            self.name_cache
                                .insert(name.clone(), (path.clone(), font_index as usize));
                            return Ok(self
                                .font_cache
                                .get_mut(&(path, font_index as usize))
                                .unwrap());
                        } else {
                            if !self.file_cache.contains_key(&path) {
                                let data = std::fs::read(path.clone())?;
                                let collection = FontLoader::vec_to_collection(data)?;
                                self.file_cache.insert(path.clone(), collection);
                            }

                            let font = Font::from_collection(
                                &self.file_cache[&path],
                                font_index as usize,
                            )?;

                            self.font_cache
                                .insert((path.clone(), font_index as usize), font);
                            self.font_cache
                                .get_mut(&(path, font_index as usize))
                                .unwrap()
                        }
                    }
                    Handle::Memory { bytes, font_index } => {
                        let data = Vec::<u8>::clone(&bytes);
                        let collection = FontLoader::vec_to_collection(data)?;
                        let font = Font::from_collection(&collection, font_index as usize)?;

                        self.mem_cache.insert(name.clone(), font);
                        self.mem_cache.get_mut(name).unwrap()
                    }
                };

                Ok(font)
            }
            Err(_) => Err(FontLoaderError::NotFound),
        }
    }
    pub fn font_exists(&mut self, name: &str) -> bool {
        let name = &name.to_lowercase();

        self.name_cache.contains_key(name)
            || self.mem_cache.contains_key(name)
            || self.to_family_name(name).is_ok()
    }
}
