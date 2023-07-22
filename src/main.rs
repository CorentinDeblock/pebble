use gameobject::Gameobject;
use ggez::graphics::Image;
use notify::Watcher;
use core::Core;
use std::path::Path;

mod core;
mod gameobject;
mod lua;
mod assets;

user_data!(
    #[derive(Clone, Debug, Default)]
    pub struct Vector {
        x: f32,
        y: f32
    }
);

user_data!(
    #[derive(Clone, Debug, Default)]
    pub struct Transform {
        position: Vector,
        rotation: f32,
        scale: Vector
    }
);

user_data!(
    #[derive(Clone, Debug, Default)]
    pub struct Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    }
);

user_data!(
    #[derive(Clone, Debug, Default)]
    pub struct Texture {
        data: Vec<u8>
    }
);

user_data!(
    #[derive(Clone, Debug, Default)]
    pub struct Material {
        albedo: Color,
        texture: Option<String>
    }
);

fn main() {
    let mut core = Core::new();

    core.add_gameobject(Gameobject::new("main.lua").unwrap());

    core.run().unwrap();
    // CORE.lock().unwrap().init();
    // CORE.lock().unwrap().update();
}
