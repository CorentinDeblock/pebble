use std::{error::Error, sync::mpsc::{channel, Receiver, Sender}, path::Path};

use mlua::Function;
use notify::{Watcher, Event, EventKind};
use uuid::Uuid;

use crate::lua::Script;

pub struct Gameobject {
    script: Script,
    name: String,
    receiver: Receiver<Result<Event, notify::Error>>,
    watcher: notify::INotifyWatcher
}

impl Gameobject {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let script = Script::from_file(name)?;
        script.run()?;

        let (sender, receiver) = channel();

        let mut watcher = notify::recommended_watcher(sender)?;

        watcher.watch(Path::new(name), notify::RecursiveMode::Recursive)?;

        Ok(Self {
            name: name.to_string(),
            script,
            receiver,
            watcher
        })
    }

    pub fn get_script(&self) -> &Script {
        &self.script
    }

    pub fn update(&mut self, delta: f32) {
        match self.script.get::<_, Function>("Update").unwrap().call::<f32, ()>(delta) {
            Ok(_) => {},
            Err(_) => {},
        }

        match self.receiver.try_recv() {
            Ok(event) => { 
                let event_type = event.unwrap().kind;
                if event_type == EventKind::Access(notify::event::AccessKind::Close(notify::event::AccessMode::Write)) {
                    self.script = Script::from_file(&self.name).unwrap();
                    self.script.run().unwrap();
                }
            },
            Err(_) => {},
        };

    }
}