use std::{marker::PhantomData, path::Path};

use ggez::{ContextBuilder, event::{EventHandler, self}, Context, graphics::{self, Color, Rect, DrawParam, FillOptions, BlendMode}, glam::Vec2, conf::Conf};
use ggegui::{egui, Gui};
use mlua::{UserData, Lua, Table, Function};

use crate::{gameobject::Gameobject, Transform, Material, assets::{AssetsLoader, LuaAsset}};

pub struct Core {
    gameobjects: Vec<Gameobject>
}

impl Core {
    pub fn new() -> Self {
        Self {
            gameobjects: Vec::new()
        }
    }

    pub fn add_gameobject(&mut self, gameobject: Gameobject) -> &mut Self {
        self.gameobjects.push(gameobject);

        self
    }

    pub fn run(self) -> Result<Self, Box<dyn std::error::Error>> {
        let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Corentin deblock")
            .build()?;

        ctx.fs.mount(Path::new("./assets/"), true);

        let game_loop = Loop::new(self, &ctx);

        event::run(ctx, event_loop, game_loop);
    }
}

struct Loop {
    assets_loader: AssetsLoader,
    core: Core,
    gui: Gui
}

impl Loop {
    pub fn new(core: Core, ctx: &Context) -> Self {
        let mut assets_loader = AssetsLoader::new();

        for go in &core.gameobjects {
            let assets : Table = go.get_script().get("Assets").unwrap();
            let files : Table = assets.get("files").unwrap();

            for data in files.pairs() {
                let unwraped = data.unwrap();

                let _ : String = unwraped.0;
                let value : LuaAsset = unwraped.1;

                assets_loader.add(value, ctx).unwrap();
            }
        }

        Self {
            assets_loader,
            core,
            gui: Gui::new(ctx)
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

impl EventHandler for Loop {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        for i in self.core.gameobjects.iter_mut() {
            i.update(_ctx.time.delta().as_secs_f32());
        }

        egui::Window::new("Log").show(&self.gui.ctx(), |ui| {
            
        });

        self.gui.update(_ctx);

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);

        canvas.draw(&self.gui, DrawParam::default().dest(Vec2::ZERO));

        for go in self.core.gameobjects.iter() {
            let transform: Transform = go.get_script().get("Transform").unwrap_or_default();
            let material: Material = go.get_script().get("Material").unwrap_or_default();
            let albedo = material.albedo;
            let texture = material.texture;

            let mesh = rect(_ctx, Rect::new(-16.0, -16.0, 32.0, 32.0));

            let draw_param = 
                graphics::DrawParam::new()
                    .dest(Vec2::new(transform.position.x, transform.position.y))
                    .rotation(transform.rotation)
                    .scale(Vec2::new(transform.scale.x, transform.scale.y))
                    .color(Color { r: albedo.r / 255.0, g: albedo.g / 255.0, b: albedo.b / 255.0, a: albedo.a / 255.0 });

            match texture {
                Some(filename) => {
                    let texture = self.assets_loader.get_texture(&filename).unwrap();
                    canvas.draw_textured_mesh(mesh, texture.value.clone(), draw_param);
                },
                None => canvas.draw(&mesh, draw_param),
            };
        }

        canvas.finish(_ctx)
    }
}