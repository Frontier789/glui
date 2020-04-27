#![windows_subsystem = "windows"]
extern crate glui;
extern crate rand;
extern crate rusty;
use gui::*;
use mecs::*;
use rusty::*;
// use tools::*;

mod board;
mod gamestate;
mod ui;
mod ai;
use board::*;
use gamestate::*;
use ui::*;

use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut w: World = World::new_win(Vec2::new(640.0, 480.0), "", Vec3::grey(0.1));
    
    let rt = w.render_target().unwrap();
    
    if let Ok(mut file) = OpenOptions::new().create(true).open("ogl.txt") {
        let _ = write!(file,"OpenGL version: {}",rt.gl_verison);
    };
    
    let mut gui = GuiContext::new(
        rt,
        true,
        game_gui,
        GameData {
            board: Default::default(),
            state: GameState::MainMenu,
        },
    );
    gui.init_gl_res();
    gui.rebuild_gui();
    let id = w.add_actor(gui);
    w.make_actor_ui_aware(id);
    w.run();
}
