extern crate glui;
extern crate rand;
extern crate rusty;
use gui::*;
use rusty::*;

use board::*;
use gamestate::*;
use ai::*;

#[derive(Clone, Debug, PartialEq)]
pub struct GameData {
    pub board: Board,
    pub state: GameState,
}

#[glui::builder(GameData)]
pub fn game_gui(data: GameData) {
    match data.state {
        GameState::MainMenu => {
            main_menu_gui();
        }
        GameState::Playing(black_player, white_player) => {
            let human_comes = data.board.human_comes(black_player, white_player);
            board_gui(data.board, human_comes);
        }
        GameState::Exiting => {
            std::process::exit(0);
        }
        GameState::Finished(r) => {
            finished_gui(r, data.board);
        }
    }
}

#[glui::builder(GameData)]
fn finished_gui(res: GameResult, board: Board) {
    board_gui(board, false);
    Overlay {
        color: Vec4::new(1.0, 1.0, 1.0, 0.21),
        children: {
            Text {
                text: res.win_text(),
                font_size: FontSize::Em(2.0),
                color: Vec4::BLACK,
                align: font::align(HAlign::Center, VAlign::Top),
            };
            Button {
                text: "Main menu".to_owned(),
                background: ButtonBckg::None,
                callback: |data| {
                    data.state = GameState::MainMenu;
                    data.board = Board::default();
                    ai_new_game();
                },
            };
        },
    };
}

#[glui::builder(GameData)]
fn main_menu_gui() {
    Square {
        children: {
            Image {
                name: "images/wood.jpg".to_owned(),
            };
            GridLayout {
                col_widths: vec![1.0; 1],
                row_heights: vec![1.0, 1.0, 2.0],
                children: {
                    Button {
                        callback: |data| {
                            data.state = GameState::Playing(PlayerInt::Human, PlayerInt::Human)
                        },
                        text: "Human v human".to_owned(),
                        background: ButtonBckg::Fill(Vec4::new(1.0, 1.0, 1.0, 0.1)),
                    };
                    Button {
                        callback: |data| {
                            data.state = GameState::Playing(PlayerInt::AI, PlayerInt::Human);
                            ai_move(&mut data.board);
                        },
                        text: "Human v AI".to_owned(),
                        background: ButtonBckg::Fill(Vec4::new(1.0, 1.0, 1.0, 0.1)),
                    };
                    Button {
                        callback: |data| data.state = GameState::Exiting,
                        text: "Exit".to_owned(),
                        background: ButtonBckg::Fill(Vec4::new(1.0, 1.0, 1.0, 0.1)),
                    };
                },
            };
        },
    };
}

#[glui::builder(GameData)]
fn board_gui(board: Board, active: bool) {
    Square {
        children: {
            Image {
                name: "images/board.jpg".to_owned(),
                children: {
                    GridLayout {
                        col_widths: vec![1.0; MAP_SIZE],
                        row_heights: vec![1.0; MAP_SIZE],
                        children: {
                            let black_turn = board.next_color() == Cell::Black;
                            let map = board.move_to_id_map();
                            
                            for n in 0..MAP_SIZE {
                                for k in 0..MAP_SIZE {
                                    let cell = board.cell(n,k);
                                    let heat = board.heat[n][k];
                                    if cell == Cell::Empty {
                                        cell_gui(cell, (n, k), 0, black_turn, heat, active);
                                    } else {
                                        cell_gui(cell, (n, k), *map.get(&(n,k)).unwrap(), black_turn, heat, active);
                                    }
                                }
                            }
                        },
                    };
                },
            };
        },
    };
}

#[glui::builder(GameData)]
fn cell_gui(cell: Cell, p: (usize, usize), n: usize, black_turn: bool, heat: f32, active: bool) {
    Padding {
        left: 2.0,
        right: 2.0,
        top: 2.0,
        bottom: 2.0,
        children: {
            Overlay {
                color: Vec4::new(1.0,0.0,0.0,heat),
            };
            match cell {
                Cell::Empty if active => {
                    Button {
                        callback: |data| {
                            let res = data.board.put(p.0,p.1);
                            if res != GameResult::NotFinished {
                                data.state = GameState::Finished(res);
                            }
                            if let GameState::Playing(black_player, white_player) = data.state {
                                if !data.board.human_comes(black_player, white_player) {
                                    let res = ai_move(&mut data.board);
                                    if res != GameResult::NotFinished {
                                        data.state = GameState::Finished(res);
                                    }
                                }
                            }
                        },
                        background: ButtonBckg::Image(
                            if black_turn {
                                "images/black.png"
                            } else {
                                "images/white.png"
                            }
                            .to_owned(),
                            Vec4::new(1.0, 1.0, 1.0, 0.0),
                            Vec4::new(1.0, 1.0, 1.0, 0.5),
                            Vec4::new(1.0, 1.0, 1.0, 0.9),
                        ),
                    };
                }
                Cell::White => {
                    cell_img("images/white.png", Vec4::BLACK, n);
                }
                Cell::Black => {
                    cell_img("images/black.png", Vec4::WHITE, n);
                }
                _ => {}
            }
        },
    };
}

#[glui::builder(GameData)]
fn cell_img(name: &str, clr: Vec4, n: usize) {
    Image {
        name: name.to_owned(),
        children: {
            Text {
                text: format!("{}", n),
                color: clr,
                font_size: FontSize::relative_steps(0.5, (6.0, 32.0), 4.0),
            };
        },
    };
}
