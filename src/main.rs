use std::{rc::Rc, cell::RefCell};
use mlua::{Lua, UserData, UserDataMethods};
use macroquad::prelude::*;

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

    let rust_print = lua
        .create_function(|_, args: mlua::Variadic<String>| {
            println!("From Lua: {}", args.join(" "));
            Ok(())
        })
        .unwrap();
    lua.globals().set("print_rs", rust_print).unwrap();

    let player = Rc::new(RefCell::new(Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        radius: 15.0,
        speed: 200.0,
    }));

    {
        let player_handle = PlayerHandle(player.clone());
        let get_player_fn = lua
            .create_function(move |lua, ()| {
                let ud = lua.create_userdata(player_handle.clone())?;
                Ok(ud)
            })
            .unwrap();
        lua.globals().set("get_player", get_player_fn).unwrap();
    }

    let key_pressed_fn = lua
        .create_function(|_, key: String| {
            let keycode = match key.to_lowercase().as_str() {
                "space" => Some(KeyCode::Space),
                "a" => Some(KeyCode::A),
                "d" => Some(KeyCode::D),
                "w" => Some(KeyCode::W),
                "s" => Some(KeyCode::S),
                _ => None,
            };
            Ok(keycode.map_or(false, |k| is_key_down(k)))
        })
        .unwrap();
    lua.globals().set("key_pressed", key_pressed_fn).unwrap();

    let content = include_str!("test.lua");
    lua.load(content).exec().unwrap();

    if let Ok(setup) = lua.globals().get::<mlua::Function>("setup") {
        setup.call::<()>(()).unwrap();
    }

    let process = lua.globals().get::<mlua::Function>("process").ok();

    loop {
        let dt = get_frame_time();

        if let Some(ref func) = process {
            func.call::<()>(dt).unwrap();
        }

        let p = player.borrow();

        clear_background(DARKGRAY);
        draw_circle(p.x, p.y, p.radius, SKYBLUE);
        draw_text("sout engine v114514", 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}
