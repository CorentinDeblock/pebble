use std::{path::Path, rc::Rc, cell::RefCell};

use ggez::{ContextBuilder, event::{self}};

use crate::{gameobject::Gameobject, lua::Script, state::State};

pub type Middleware = fn(&Script);
pub type MiddlewareStorage = Rc<RefCell<Vec<Middleware>>>;
pub type GameobjectStorage = Vec<Gameobject>;

pub struct Core {
    pub gameobjects: GameobjectStorage,
    middlewares: MiddlewareStorage
}

impl Core {
    pub fn new() -> Self {
        Self {
            gameobjects: Vec::new(),
            middlewares: Rc::new(RefCell::new(Vec::new()))
        }
    }

    pub fn add_middleware(&mut self, middleware: Middleware) -> &mut Self {
        self.middlewares.borrow_mut().push(middleware);
        
        self
    }

    pub fn add_gameobject(&mut self, name: &str) -> &mut Self {
        self.gameobjects.push(Gameobject::new(name,Rc::clone(&self.middlewares)).unwrap());

        self
    }

    pub fn run(self) -> Result<Self, Box<dyn std::error::Error>> {
        let (ctx, event_loop) = ContextBuilder::new("my_game", "Corentin deblock")
            .build()?;

        ctx.fs.mount(Path::new("./assets/"), true);

        let mut game_loop = State::new(self, &ctx);

        game_loop.init();

        event::run(ctx, event_loop, game_loop);
    }
}