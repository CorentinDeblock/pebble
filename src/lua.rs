use std::{error::Error, marker::PhantomData};

use mlua::{FromLua, ToLua, Function};

#[derive(std::default::Default)]
pub struct Empty {}

pub trait LuaComponent<'lua> {
    fn from_lua_table(table: mlua::Table<'lua>) -> Result<Self, Box<dyn Error>> where Self: Sized;
}

#[macro_export]
macro_rules! user_data {
    (
        #[derive($($derived: ident),*)]
        $struct_access: vis struct $ty: ident<'a> {
            $($field_access: vis $value: ident: $value_type: ty),*
        }
    ) => {
        #[derive($($derived),*)]
        $struct_access struct $ty<'a> {
            $($field_access $value: $value_type),*,
            phantom: &'a std::marker::PhantomData<crate::lua::Empty>
        }

        impl<'lua> mlua::FromLua<'lua> for $ty<'lua> {
            fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let table = match lua_value {
                    mlua::Value::Table(table) => Ok(table),
                    mlua::Value::Error(err) => Err(err),
                    mlua::Value::Nil => Err(mlua::Error::RuntimeError(String::from("Value is Nil"))),
                    _ => Err(mlua::Error::RuntimeError(String::from("Only table are supported")))
                }.unwrap();
        
                Ok(Self {
                    $($value: table.get(stringify!($value)).unwrap()),*,
                    phantom: &std::marker::PhantomData
                })
            }
        }

        impl<'lua> crate::lua::LuaComponent<'lua> for $ty<'lua> {
            fn from_lua_table(table: mlua::Table<'lua>) -> Result<Self, Box<dyn Error>> {
                Ok(Self {
                    $($value: table.get(stringify!($value))?),*,
                    phantom: &std::marker::PhantomData,
                })
            }
        }
    };
}

pub struct LuaArray<'a, T : mlua::FromLua<'a> + Clone> {
    pub count: i32,
    pub data: mlua::Table<'a>,
    phantom: &'a PhantomData<T>
}

impl<'a, T : mlua::FromLua<'a> + Clone> LuaArray<'a, T> {
    pub fn iter(&self) -> mlua::TablePairs<'_, String, T> {
        self.data.clone().pairs::<std::string::String, T>()
    }
}

impl<'a, T : mlua::FromLua<'a> + Clone> mlua::FromLua<'a> for LuaArray<'a, T> {
    fn from_lua(lua_value: mlua::Value<'a>, _: &'a mlua::Lua) -> mlua::Result<Self> {
        let table = match lua_value {
            mlua::Value::Table(table) => Ok(table),
            mlua::Value::Error(err) => Err(err),
            _ => Err(mlua::Error::RuntimeError("Not a array".to_string()))
        }?;

        Ok(Self {
            count: table.get("count")?,
            data: table.get("data")?,
            phantom: &PhantomData
        })
    }
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

    pub fn get_name(&self) -> &String {
        &self.name
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