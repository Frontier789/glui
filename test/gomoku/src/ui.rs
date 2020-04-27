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
            board_gui(data.board, human_comes, GameResult::NotFinished);
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
    board_gui(board, false, res.clone());
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
                row_heights: vec![1.0, 1.0, 1.0, 3.0],
                children: {
                    Button {
                        callback: |data| {
                            data.state = GameState::Playing(PlayerInt::Human, PlayerInt::Human);
                            data.board = Board::default();
                        },
                        text: "Human v human".to_owned(),
                        background: ButtonBckg::Fill(Vec4::new(1.0, 1.0, 1.0, 0.1)),
                    };
                    Button {
                        callback: |data| {
                            data.state = GameState::Playing(PlayerInt::AI, PlayerInt::Human);
                            data.board = Board::default();
                            ai_new_game();
                            ai_move(&mut data.board);
                        },
                        text: "AI v Human".to_owned(),
                        background: ButtonBckg::Fill(Vec4::new(1.0, 1.0, 1.0, 0.1)),
                    };
                    Button {
                        callback: |data| {
                            data.state = GameState::Playing(PlayerInt::Human, PlayerInt::AI);
                            data.board = Board::default();
                            ai_new_game();
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
fn board_gui(board: Board, active: bool, result: GameResult) {
    Square {
        children: {
            Image {
                name: "images/board.jpg".to_owned(),
                children: {
                    Padding {
                        children: {
                            GridLayout {
                                col_widths: vec![1.0; MAP_SIZE],
                                row_heights: vec![1.0; MAP_SIZE],
                                children: {
                                    let black_turn = board.next_color() == Cell::Black;
                                    let over = result.over();
                                    let move_count = board.moves().len();
                                    let map = board.move_to_id_map();
                                    
                                    for n in 0..MAP_SIZE {
                                        for k in 0..MAP_SIZE {
                                            let cell = board.cell(n,k);
                                            let heat = board.heat[n][k];
                                            
                                            if cell == Cell::Empty {
                                                cell_gui(cell, (n, k), false, 0, black_turn, heat, active);
                                            } else {
                                                let id = *map.get(&(n,k)).unwrap();
                                                let mut highlighted = id == move_count || id+1 == move_count;
                                                
                                                if over {
                                                    highlighted = false;
                                                    match result.clone() {
                                                        GameResult::BlackWon(pts) | GameResult::WhiteWon(pts) => {
                                                            for p in pts {
                                                                if p.0 == n && p.1 == k {
                                                                    highlighted = true;
                                                                }
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                
                                                cell_gui(cell, (n, k), highlighted, id, black_turn, heat, active);
                                            }
                                        }
                                    }
                                },
                            };
                        },
                        ..Padding::relative(1.0 / 32.0)
                    };
                },
            };
        },
    };
}

#[glui::builder(GameData)]
fn cell_gui(cell: Cell, p: (usize, usize), highlight: bool, n: usize, black_turn: bool, heat: f32, active: bool) {
    Padding {
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
                    cell_img(if highlight {"images/white_highlighted.png"} else {"images/white.png"}, Vec4::BLACK, n, !highlight);
                }
                Cell::Black => {
                    cell_img(if highlight {"images/black_highlighted.png"} else {"images/black.png"}, Vec4::WHITE, n, !highlight);
                }
                _ => {}
            }
        },
        .. Padding::absolute(2.0)
    };
}

#[glui::builder(GameData)]
fn cell_img(name: &str, clr: Vec4, n: usize, show_number: bool) {
    Image {
        name: name.to_owned(),
        children: {
            if show_number {
                Text {
                    text: format!("{}", n),
                    color: clr,
                    font_size: FontSize::relative_steps(0.5, (6.0, 32.0), 4.0),
                };
            }
        },
    };
}
