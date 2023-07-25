use chrono::{DateTime, Utc, Timelike};
use ggegui::{Gui, egui};
use ggez::{Context, event::EventHandler, graphics::{self, DrawParam, Color}, glam::Vec2};
use mlua::Table;

use crate::{assets::{AssetsLoader, LuaAsset}, core::Core, LOG};


/// The state of the engine. Handle gameobject, assets loading, rendering, gameloop, gui, etc...
pub struct State {
    assets_loader: AssetsLoader,
    core: Core,
    gui: Gui
}

impl State {
    pub fn new(core: Core, ctx: &Context) -> Self {
        Self {
            assets_loader: AssetsLoader::new(),
            core,
            gui: Gui::new(ctx)
        }
    }

    pub fn init(&mut self) {
        self.assets_loader.load_ui_texture("warning.png").unwrap();
        self.assets_loader.load_ui_texture("info.png").unwrap();
        self.assets_loader.load_ui_texture("trace.png").unwrap();
        self.assets_loader.load_ui_texture("error.png").unwrap();
        self.assets_loader.load_ui_texture("debug.png").unwrap();

        for go in self.core.gameobjects.iter_mut() {
            go.init();
        }
    }
}

fn date_to_string(date: DateTime<Utc>) -> String {
    format!("{}:{}:{}", date.hour(), date.minute(), date.second())
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        for go in self.core.gameobjects.iter_mut() {
            if go.is_loaded {
                if go.is_assets_reloaded() {
                    let assets : Table = go.get_script().get("Assets").unwrap();
                    let files : Table = assets.get("textures").unwrap();
            
                    for data in files.pairs() {
                        let unwraped = data.unwrap();
            
                        let _ : String = unwraped.0;
                        let value : LuaAsset = unwraped.1;
            
                        match self.assets_loader.load_texture(&value.get_filename(), &_ctx) {
                            Ok(_) => {},
                            Err(err) => error!("Error when loading file {} : {}", value.get_filename(), err),
                        }
                    }
                }
    
                go.update(_ctx.time.delta().as_secs_f32());
            }

            go.watch();
        }

        egui::Window::new("Log").show(&self.gui.ctx(), |window: &mut egui::Ui| {
            window.set_width(400.0);
            window.set_height(300.0);


            let panel = egui::scroll_area::ScrollArea::new([false, true]);

            panel.show(window, |ui| {
                unsafe {
                    for log in LOG.iter() {
                        ui.horizontal(|ui|{
                            let texture = match log.level {
                                log::Level::Error => self.assets_loader.get_ui_texture("error.png").unwrap(),
                                log::Level::Warn => self.assets_loader.get_ui_texture("warning.png").unwrap(),
                                log::Level::Info => self.assets_loader.get_ui_texture("info.png").unwrap(),
                                log::Level::Debug => self.assets_loader.get_ui_texture("debug.png").unwrap(),
                                log::Level::Trace => self.assets_loader.get_ui_texture("trace.png").unwrap(),
                            };

                            ui.image(texture.texture_id(ui.ctx()), egui::vec2(16.0, 16.0));

                            if log.level == log::Level::Debug || log.level == log::Level::Trace {
                                ui.label(format!("[{}] - {} on [{}:{}]", 
                                    date_to_string(log.date),
                                    log.data, 
                                    log.file,
                                    log.line
                                ));
                            } else {
                                ui.label(format!("[{}] - {}", 
                                    date_to_string(log.date),
                                    log.data,
                                ));
                            }
                        });
                    }
                }
            });
        });

        self.gui.update(_ctx);

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);

        for go in self.core.gameobjects.iter() {
            if go.is_loaded {
                go.render(_ctx, &mut canvas, &self.assets_loader)
            }
        }

        canvas.draw(&self.gui, DrawParam::default().dest(Vec2::ZERO));

        canvas.finish(_ctx)
    }
}