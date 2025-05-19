use std::{cell::RefCell, rc::Rc};

use macroquad::{
    color::BLUE,
    input::{KeyCode, is_key_down, is_key_pressed},
    window::{screen_height, screen_width},
};
use mlua::{Lua, Table, Variadic};

use crate::{Player, PlayerHandle, Rectangle, RectangleHandle, SceneHandle};

pub fn init_lua_functions(lua: &Lua, sout: &Table, scene_handle: &SceneHandle) {
    let println_rs = lua
        .create_function(|_, args: Variadic<String>| {
            println!("From Lua: {}", args.join(" "));
            Ok(())
        })
        .unwrap();
    sout.set("println", println_rs).unwrap();

    let print_rs = lua
        .create_function(|_, args: Variadic<String>| {
            print!("From Lua: {}", args.join(" "));
            Ok(())
        })
        .unwrap();
    sout.set("print", print_rs).unwrap();

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
            Ok(keycode.map_or(false, |k| is_key_pressed(k)))
        })
        .unwrap();
    sout.set("key_pressed", key_pressed_fn).unwrap();

    let key_down_fn = lua
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
    sout.set("key_down", key_down_fn).unwrap();

    // sout.get_scene()
    {
        let scene_handle = scene_handle.clone();
        let get_scene_fn = lua
            .create_function(move |lua, ()| {
                let ud = lua.create_userdata(scene_handle.clone())?;
                Ok(ud)
            })
            .unwrap();
        sout.set("get_scene", get_scene_fn).unwrap();
    }

    // sout.new_player()
    {
        let new_player_fn = lua
            .create_function(|lua, ()| {
                let player = Player {
                    x: screen_width() / 2.0,
                    y: screen_height() / 2.0,
                    radius: 15.0,
                    speed: 200.0,
                };
                let player_handle = PlayerHandle(Rc::new(RefCell::new(player)));
                let ud = lua.create_userdata(player_handle)?;
                Ok(ud)
            })
            .unwrap();
        sout.set("new_player", new_player_fn).unwrap();
    }

    // sout.new_entity()
    {
        let new_rectangle_fn = lua
            .create_function(|lua, name: String| match name.as_str() {
                "player" => {
                    let player = Player {
                        x: screen_width() / 2.0,
                        y: screen_height() / 2.0,
                        radius: 15.0,
                        speed: 200.0,
                    };
                    let handle = PlayerHandle(Rc::new(RefCell::new(player)));
                    lua.create_userdata(handle).map(mlua::Value::UserData)
                }
                "rect" => {
                    let pipe = Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 50.0,
                        height: 50.0,
                        color: BLUE,
                    };
                    let handle = RectangleHandle(Rc::new(RefCell::new(pipe)));
                    lua.create_userdata(handle).map(mlua::Value::UserData)
                }
                _ => Err(mlua::Error::external("Unknown entity type")),
            })
            .unwrap();
        sout.set("new_entity", new_rectangle_fn).unwrap();
    }

    // sout.viewport()
    {
        let viewport_fn = lua
            .create_function(|_, ()| {
                let (width, height) = (screen_width(), screen_height());
                Ok((width, height))
            })
            .unwrap();
        sout.set("viewport", viewport_fn).unwrap();
    }
}
