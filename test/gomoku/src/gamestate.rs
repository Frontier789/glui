extern crate glui;
extern crate rand;
extern crate rusty;

use board::GameResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayerInt {
    Human,
    AI,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    MainMenu,
    Playing(PlayerInt, PlayerInt),
    Finished(GameResult),
    Exiting,
}
