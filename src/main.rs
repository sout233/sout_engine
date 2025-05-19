use macroquad::prelude::*;
use mlua::{Lua, UserData, UserDataMethods};
use std::{cell::RefCell, rc::Rc};

mod functions;
mod utils;

#[derive(Clone)]
struct Scene {
    entities: Vec<Entity>,
}

#[derive(Clone)]
struct SceneHandle(Rc<RefCell<Scene>>);

impl UserData for SceneHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add_entity", |_, this, entity: mlua::AnyUserData| {
            if let Ok(player) = entity.borrow::<PlayerHandle>().map(|p| p.clone()) {
                this.0.borrow_mut().entities.push(Entity::Player(player));
            } else if let Ok(pipe) = entity.borrow::<RectangleHandle>().map(|p| p.clone()) {
                this.0.borrow_mut().entities.push(Entity::Rectangle(pipe));
            } else {
                return Err(mlua::Error::external("Unsupported entity type"));
            }

            Ok(())
        });
        methods.add_method("id", |_, this, ()| {
            let id = this.0.borrow().entities.len();
            Ok(id)
        });
    }
}

#[derive(Clone)]
enum Entity {
    Player(PlayerHandle),
    Rectangle(RectangleHandle),
}

#[derive(Clone)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

#[derive(Clone)]
struct RectangleHandle(Rc<RefCell<Rectangle>>);

impl UserData for RectangleHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_x", |_, this, x: f32| {
            this.0.borrow_mut().x = x;
            Ok(())
        });
        methods.add_method_mut("set_y", |_, this, y: f32| {
            this.0.borrow_mut().y = y;
            Ok(())
        });
        methods.add_method("x", |_, this, ()| Ok(this.0.borrow().x));
        methods.add_method("y", |_, this, ()| Ok(this.0.borrow().y));
        methods.add_method("w", |_, this, ()| Ok(this.0.borrow().width));
        methods.add_method("h", |_, this, ()| Ok(this.0.borrow().height));
        methods.add_method_mut("set_size", |_, this, (w, h): (f32, f32)| {
            let mut pipe = this.0.borrow_mut();
            pipe.width = w;
            pipe.height = h;
            Ok(())
        });
        methods.add_method_mut("set_color", |_, this, (r, g, b): (u8, u8, u8)| {
            this.0.borrow_mut().color = Color::from_rgba(r, g, b, 255);
            Ok(())
        });
    }
}

#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
}

#[derive(Clone)]
struct PlayerHandle(Rc<RefCell<Player>>);

impl UserData for PlayerHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("x", |_, this, ()| Ok(this.0.borrow().x));
        methods.add_method_mut("set_x", |_, this, x: f32| {
            this.0.borrow_mut().x = x;
            Ok(())
        });

        methods.add_method("y", |_, this, ()| Ok(this.0.borrow().y));
        methods.add_method_mut("set_y", |_, this, y: f32| {
            this.0.borrow_mut().y = y;
            Ok(())
        });

        methods.add_method("speed", |_, this, ()| Ok(this.0.borrow().speed));
    }
}

#[macroquad::main("sout engine test")]
async fn main() {
    let lua = Lua::new();
    let sout = lua.create_table().unwrap();
    lua.globals().set("sout", &sout).unwrap();

    let scene = Rc::new(RefCell::new(Scene { entities: vec![] }));
    let scene_handle = SceneHandle(scene.clone());

    functions::init_lua_functions(&lua, &sout, &scene_handle);

    let path = "src/test.lua";
    let content = std::fs::read_to_string(path).unwrap();
    lua.load(&content)
        .exec()
        .map_err(|e| {
            println!("Error when loading Lua script");
            match e {
                mlua::Error::SyntaxError {
                    message,
                    incomplete_input: _,
                } => {
                    utils::print_mlua_error_msg(message, path, &content);
                }
                _ => panic!("Error: {:?}", e),
            }
        })
        .unwrap();

    if let Ok(setup) = lua.globals().get::<mlua::Function>("setup") {
        if let Err(e) = setup.call::<()>(()) {
            println!("Error in setup: {:?}", e);
        }
    }

    let process = lua.globals().get::<mlua::Function>("process").ok();

    loop {
        let dt = get_frame_time();

        if let Some(ref func) = process {
            func.call::<()>(dt).unwrap();
        }

        clear_background(DARKGRAY);

        for entity in &scene.borrow().entities {
            match entity {
                Entity::Player(p) => {
                    let p = p.0.borrow();
                    draw_circle(p.x, p.y, p.radius, SKYBLUE);
                }
                Entity::Rectangle(p) => {
                    let p = p.0.borrow();
                    draw_rectangle(p.x, p.y, p.width, p.height, p.color);
                }
            }
        }

        draw_text("sout engine v114514", 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}
