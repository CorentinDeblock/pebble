#[macro_use]
extern crate log;

use chrono::Utc;
use env_logger::{Builder, Logger};

use ggez::glam::Vec2;
use log::{SetLoggerError, Log, Level};
use lua::Script;
use mlua::UserData;
use once_cell::sync::Lazy;
use core::Core;
use std::{error::Error, marker::PhantomData};

mod core;
mod gameobject;
mod lua;
mod assets;
mod state;

pub enum PebbleMode {
    Release,
    Debug
}

struct Config {
    mode: PebbleMode
}

impl Config {
    fn new() -> Self {
        let pebble_mode_str = std::env::var("PebbleMode").unwrap_or("".to_string());
        let mut mode: PebbleMode = PebbleMode::Release;

        if pebble_mode_str == "debug" {
            mode = PebbleMode::Debug
        }

        Self {
            mode
        }
    }
}
 
static PEBBLE_CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

user_data!(
    #[derive(Clone, Debug)]
    pub struct Vector<'a> {
        x: f32,
        y: f32
    }
);

user_data!(
    #[derive(Clone, Debug)]
    pub struct Transform<'a> {
        position: Vector<'a>,
        rotation: f32,
        scale: Vector<'a>
    }
);

user_data!(
    #[derive(Clone, Debug)]
    pub struct Color<'a> {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    }
);

user_data!(
    #[derive(Clone, Debug)]
    pub struct Material<'a> {
        albedo: Color<'a>,
        texture: Option<String>
    }
);

user_data!(
    #[derive(Clone, Debug)]
    pub struct Component<'a> {
        pub c_type: String,
        pub data: mlua::Table<'a>
    }
);

pub struct CoreLogger {
    pub line: u32,
    pub data: String,
    pub level: Level,
    file: String,
    date: chrono::DateTime<Utc>
}

static mut LOG : Lazy<Vec<CoreLogger>> = Lazy::new(|| Vec::new()); 

struct MyLogger {
    inner: Logger
}

const FILTER_ENV: &str = "trace";

impl MyLogger {
    fn new() -> Self {
        let mut builder = Builder::from_env(FILTER_ENV);

        builder.filter_level(log::LevelFilter::Error);
        
        match PEBBLE_CONFIG.mode {
            PebbleMode::Release => builder.filter(Some("pebble"), log::LevelFilter::Info),
            PebbleMode::Debug => builder.filter(Some("pebble"), log::LevelFilter::Trace)
        };

        Self {
            inner: builder.build(),
        }
    }

    fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        if self.inner.matches(record) {
            unsafe {
                LOG.push(CoreLogger {
                    date: Utc::now(),
                    file: record.file().unwrap().to_string(),
                    line: record.line().unwrap(), 
                    data: record.args().to_string(), 
                    level: record.level() 
                });
            }
        }
    }

    fn flush(&self) {}
}

impl<'a> Material<'a> {
    pub fn new() -> Self {
        Self {
            albedo: Color { r: 222.0, g: 222.0, b: 222.0, a: 255.0, phantom: &PhantomData },
            texture: None,
            phantom: &PhantomData,
        }
    }
}

impl<'a> Vector<'a> {
    pub fn to(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}

struct LuaLog {
    name: String
}

fn format_log(data: String, lua: &mlua::Lua, log: &LuaLog) -> String {
    let debug = lua.inspect_stack(2).unwrap();
    let mut filename = String::from_utf8(debug.source().short_src.unwrap().to_vec()).unwrap();

    if filename.contains(".rs") {
        filename = log.name.clone()
    }

    filename = filename.replace("./", "");

    format!("{} on [{}:{}]", data, filename, debug.curr_line())
}

impl UserData for LuaLog {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_method("info", |lua, this, data: String| Ok(info!("{}", format_log(data, lua, this))));
        _methods.add_method("warn", |lua, this, data: String| Ok(warn!("{}", format_log(data, lua, this))));
        _methods.add_method("error", |lua, this, data: String| Ok(error!("{}", format_log(data, lua, this))));
        _methods.add_method("debug", |lua, this, data: String| Ok(debug!("{}", format_log(data, lua, this))));
        _methods.add_method("trace", |lua, this, data: String| Ok(trace!("{}", format_log(data, lua, this))));
    }
}

fn add_core_library(script: &Script) {
    script.get_state().globals().set("rust_log", LuaLog {
        name: script.get_name().clone()
    }).unwrap();
}

fn main() {
    let mut core = Core::new();

    MyLogger::init().unwrap();

    core.add_gameobject("main.lua");
    core.add_middleware(add_core_library);

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    core.run().unwrap();
}
