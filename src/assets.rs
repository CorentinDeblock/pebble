use std::{collections::HashMap, any::Any, error::Error, io::ErrorKind};

use crate::user_data;

user_data!(
    #[derive(Clone, Debug, Default)]
    pub struct LuaAsset {
        filename: String,
        file_type: String
    }
);

pub struct Asset<T> {
    lua: LuaAsset,
    pub value: T
}

/// Load file receive from lua script
pub struct AssetsLoader {
    textures: HashMap<String, Asset<ggez::graphics::Image>>
}

impl AssetsLoader {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    pub fn add(&mut self, lua: LuaAsset, ctx: &ggez::Context) -> Result<(), Box<dyn Error>> {
        let filename = lua.filename.clone();
        
        if lua.file_type == "image" {
            let value = ggez::graphics::Image::from_path(ctx, format!("/{}", filename))?;
            self.textures.insert(filename, Asset { lua, value });
        } else {
            panic!("File type is not recognized");
        }

        Ok(())
    }

    pub fn get_texture(&self, filename: &str) -> Option<&Asset<ggez::graphics::Image>> {
        self.textures.get(filename)
    }
}
