use std::{error::Error, sync::mpsc::{channel, Receiver}, path::Path};

use ggez::graphics::{self, DrawParam, Rect};
use mlua::Function;
use notify::{Watcher, Event, EventKind};
use crate::{lua::{Script, LuaArray, LuaComponent}, Component, Material, Transform, core::MiddlewareStorage, assets::AssetsLoader};

/// Gameobject are object that interact with the world, gameobject are like a character, ennemy, map, etc...
pub struct Gameobject {
    script: Script,
    name: String,
    receiver: Receiver<Result<Event, notify::Error>>,
    watcher: notify::INotifyWatcher,
    reload_asset: bool,
    middlewares: MiddlewareStorage,
    pub is_loaded: bool
}

impl Gameobject {
    pub fn new(name: &str, middlewares: MiddlewareStorage) -> Result<Self, Box<dyn Error>> {
        let script = Script::from_file(name)?;

        let (sender, receiver) = channel();
        
        let mut watcher = notify::recommended_watcher(sender)?;

        watcher.watch(Path::new(name), notify::RecursiveMode::Recursive)?;

        Ok(Self {
            name: name.to_string(),
            script,
            receiver,
            watcher,
            middlewares,
            is_loaded: false,
            reload_asset: true
        })
    }

    pub fn init(&mut self) {
        for middleware in self.middlewares.borrow().iter() {
            middleware(&self.script)
        }

        match self.script.run() {
            Ok(_) => self.loaded(),
            Err(err) => self.failed(err),
        }
    }

    pub fn get_script(&self) -> &Script {
        &self.script
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_loaded {
            match self.script.get::<_, Function>("Update").unwrap().call::<f32, ()>(delta) {
                Ok(_) => {},
                Err(_) => {},
            }
        }
    }

    pub fn watch(&mut self) {
        match self.receiver.try_recv() {
            Ok(event) => { 
                let event_type = event.unwrap().kind;
                if event_type == EventKind::Access(notify::event::AccessKind::Close(notify::event::AccessMode::Write)) {
                    self.script = Script::from_file(&self.name).unwrap();
                    self.init();
                }
            },
            Err(_) => {},
        };
    }

    fn loaded(&mut self) {
        self.is_loaded = true;
        self.reload_asset = true;
        debug!("Reloaded script {}", self.script.get_name());
    }

    fn failed(&mut self, err: mlua::Error) {
        self.is_loaded = false;
        error!("{}", err)
    }

    pub fn is_assets_reloaded(&mut self) -> bool {
        let store = self.reload_asset;
        self.reload_asset = false;
        store.clone()
    }

    pub fn render(&self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas, asset_loader: &AssetsLoader) {
        let components : LuaArray<Component> = self.script.get("Components").unwrap();
        let mesh = rect(ctx, Rect::new(-16.0, -16.0, 32.0, 32.0));

        let mut draw_param = DrawParam::default();
        let mut texture : Option<&ggez::graphics::Image> = None;

        for result in components.iter() {
            let (_, component) : (String, Component) = result.unwrap();
            if component.c_type == "Material" {
                let material = Material::from_lua_table(component.data).unwrap();

                draw_param = draw_param.color(ggez::graphics::Color{ 
                    r: material.albedo.r / 255.0, 
                    g: material.albedo.g / 255.0, 
                    b: material.albedo.b / 255.0, 
                    a: material.albedo.a / 255.0
                });

                if let Some(texture_str) = material.texture {
                    texture = asset_loader.get_texture(&texture_str);
                };
                
            } else if component.c_type == "Transform" {
                let transform = Transform::from_lua_table(component.data).unwrap();

                draw_param = draw_param
                    .scale(transform.scale.to())
                    .rotation(transform.rotation)
                    .dest(transform.position.to());
            }
        }

        match texture {
            Some(texture) => canvas.draw_textured_mesh(mesh, texture.clone(), draw_param),
            None => canvas.draw(&mesh, draw_param),
        }
    }
}

pub fn rect(ctx: &ggez::Context, rect: Rect) -> graphics::Mesh {
    let vertices = vec![
        graphics::Vertex { 
            position: [rect.x, rect.y], 
            uv: [0.0, 0.0], 
            color: [1.0, 1.0, 1.0, 1.0] 
        },
        graphics::Vertex { 
            position: [rect.x + rect.w, rect.y], 
            uv: [1.0, 0.0], 
            color: [1.0, 1.0, 1.0, 1.0] 
        },
        graphics::Vertex { 
            position: [rect.x + rect.w, rect.y + rect.h], 
            uv: [1.0, 1.0], 
            color: [1.0, 1.0, 1.0, 1.0] 
        },
        graphics::Vertex { 
            position: [rect.x, rect.y + rect.h], 
            uv: [0.0, 1.0], 
            color: [1.0, 1.0, 1.0, 1.0] 
        }
    ];

    let indices = vec![0,1,2,2,3,0];

    graphics::Mesh::from_data(ctx, graphics::MeshData { vertices: &vertices, indices: &indices })
}