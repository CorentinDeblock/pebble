use std::{error::Error, path::Path, sync::mpsc::channel};

use mlua::{UserData, FromLua, ToLua, Function};

#[macro_export]
macro_rules! user_data {
    (
        #[derive($($derived: ident),*)]
        pub struct $ty: ident {
            $($value: ident: $value_type: ty),*
        }
    ) => {
        #[derive($($derived),*)]
        pub struct $ty {
            $($value: $value_type),*
        }

        impl<'lua> mlua::FromLua<'lua> for $ty {
            fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let table = match lua_value {
                    mlua::Value::Table(table) => Ok(table),
                    mlua::Value::Error(err) => Err(err),
                    _ => Err(mlua::Error::RuntimeError(String::from("Only table are supported")))
                }.unwrap();
        
                Ok(Self {
                    $($value: table.get(stringify!($value)).unwrap()),*
                })
            }
        }
    };
}

/// Script handle reading, debugging, storing and running lua code
pub struct Script {
    name: String,
    state: mlua::Lua,
    content: String
}

impl Script {
    pub fn new(name: &str, content: &str) -> Self {
        Self { state: mlua::Lua::new(), content: content.to_string(), name: name.to_string() }
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>>  {
        Ok(Self::new(filename, &std::fs::read_to_string(filename)?))
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_state(&self) -> &mlua::Lua {
        &self.state
    }

    pub fn run(&self) -> Result<(), mlua::Error> {
        self.state.load(&self.content).exec()?;
        Ok(())
    }

    pub fn get<'lua, K: ToLua<'lua>, T: FromLua<'lua>>(&'lua self, name: K) -> Result<T, Box<dyn std::error::Error>> {
        Ok(self.state.globals().get::<K, T>(name)?)
    }

    pub fn call_function(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let func : Function = self.get(name)?;
        Ok(func.call::<_, ()>(())?)
    }

    pub fn call_method<'lua, T: FromLua<'lua>>(&'lua self, name: &str) -> Result<T, Box<dyn Error>> {
        let func : Function = self.get(name)?;
        Ok(func.call::<_, T>(())?)
    }
}