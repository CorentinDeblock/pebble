use std::{collections::HashMap, error::Error};

use crate::user_data;

user_data!(
    #[derive(Clone, Debug)]
    pub struct LuaAsset<'a> {
        filename: String,
        file_type: String
    }
);

impl<'a> LuaAsset<'a> {
    pub fn get_filename(&self) -> &String {
        &self.filename
    }
}

struct AssetStorage<T> {
    storage: HashMap<String, T>
}

impl<T> AssetStorage<T> {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, value: T) {
        if !self.storage.contains_key(name) {
            self.storage.insert(name.to_string(), value);
        }
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.storage.get(&name.to_string())
    }
}

/// Load file receive from lua script
pub struct AssetsLoader {
    storage: AssetStorage<Vec<u8>>,
    textures: AssetStorage<ggez::graphics::Image>,
    ui_textures: AssetStorage<egui_extras::RetainedImage>
}

impl AssetsLoader {
    pub fn new() -> Self {
        Self { storage: AssetStorage::new(), textures: AssetStorage::new(), ui_textures: AssetStorage::new() }
    }

    pub fn load_file(&mut self, filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        self.storage.add(filename, std::fs::read(format!("./assets/{}", filename))?);
        debug!("Successfully loaded file {}", filename);

        Ok(self.storage.get(filename).unwrap().clone())
    }

    pub fn load_texture(&mut self, filename: &str, ctx: &ggez::Context) -> Result<&ggez::graphics::Image, Box<dyn Error>> {
        let file = self.load_file(filename)?;
        self.textures.add(filename, ggez::graphics::Image::from_bytes(ctx, &file)?);
        debug!("Successfully loaded texture {}", filename);

        Ok(self.textures.get(filename).unwrap())
    }

    pub fn load_ui_texture(&mut self, filename: &str) -> Result<&egui_extras::RetainedImage, Box<dyn Error>> {
        let file = self.load_file(filename)?;
        self.ui_textures.add(filename, egui_extras::RetainedImage::from_image_bytes(filename, &file)?);
        debug!("Successfully loaded ui texture {}", filename);

        Ok(self.ui_textures.get(filename).unwrap())
    }

    pub fn get_file(&self, filename: &str) -> Option<&Vec<u8>> {
        self.storage.get(filename)
    }

    pub fn get_texture(&self, filename: &str) -> Option<&ggez::graphics::Image> {
        self.textures.get(filename)
    }

    pub fn get_ui_texture(&self, filename: &str) -> Option<&egui_extras::RetainedImage> {
        self.ui_textures.get(filename)
    }
}