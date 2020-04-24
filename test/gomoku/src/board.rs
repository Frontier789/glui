extern crate glui;
extern crate rand;
extern crate rusty;
use std::collections::HashMap;
use gamestate::PlayerInt;

pub const MAP_SIZE: usize = 15;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Cell {
    pub fn opponent(self) -> Cell {
        match self {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => Cell::Empty,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Board {
    cells: [[Cell; MAP_SIZE]; MAP_SIZE],
    pub heat: [[f32; MAP_SIZE]; MAP_SIZE],
    moves: Vec<(usize, usize)>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameResult {
    NotFinished,
    BlackWon,
    WhiteWon,
    Draw,
}

impl GameResult {
    pub fn or(self, res: GameResult) -> GameResult {
        use self::GameResult::*;
        match (self, res) {
            (NotFinished, Draw) => NotFinished,
            (NotFinished, r) => r,
            (Draw, r) => r,
            (r, _) => r,
        }
    }
    pub fn win_text(&self) -> String {
        match self {
            GameResult::BlackWon => "Black won",
            GameResult::WhiteWon => "White won",
            GameResult::Draw => "It's a draw",
            _ => "",
        }
        .to_owned()
    }
}

pub struct BoardLineIterator<'a> {
    pub p: (i32, i32),
    pub v: (i32, i32),
    pub cells: &'a [[Cell; MAP_SIZE]; MAP_SIZE],
}

impl<'a> Iterator for BoardLineIterator<'a> {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.p.0 < 0
            || self.p.1 < 0
            || self.p.0 >= MAP_SIZE as i32
            || self.p.1 >= MAP_SIZE as i32
        {
            return None;
        }
        let cell = self.cells[self.p.0 as usize][self.p.1 as usize];
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        Some(cell)
    }
}

impl Board {
    pub fn moves(&self) -> &Vec<(usize, usize)> {
        &self.moves
    }
    
    pub fn next_color(&self) -> Cell {
        let n = self.moves.len();
        if n % 2 == 0 {
            Cell::Black
        } else {
            Cell::White
        }
    }
    pub fn move_to_id_map(&self) -> HashMap<(usize, usize), usize> {
        let mut map = HashMap::new();
                            
        for i in 0..self.moves.len() {
            map.insert(self.moves[i], i+1);
        }
        
        map
    }
    pub fn human_comes(&self, black_player: PlayerInt, white_player: PlayerInt) -> bool {
        [black_player, white_player][self.moves.len() % 2] == PlayerInt::Human
    }
    pub fn cell(&self, x: usize, y: usize) -> Cell {
        self.cells[x][y]
    }
    pub fn put(&mut self, x: usize, y: usize) -> GameResult {
        self.cells[x][y] = self.next_color();
        self.moves.push((x,y));
        self.result()
    }
    // pub fn undo(&mut self) {
    //     if let Some(p) = self.moves.pop() {
    //         self.cells[p.0][p.1] = Cell::Empty;
    //     }
    // }
    fn check_line(line: BoardLineIterator) -> GameResult {
        use Cell::*;
        let mut stack = Vec::<Cell>::new();
        let mut has_space = false;
        for cell in line {
            match (cell, stack.last()) {
                (Empty, _) => {
                    stack.clear();
                    has_space = true;
                }
                (Black, Some(Black)) | (Black, None) => {
                    stack.push(Black);
                    if stack.len() == 5 {
                        return GameResult::BlackWon;
                    }
                }
                (Black, _) => {
                    stack.clear();
                    stack.push(Black);
                }
                (White, Some(White)) | (White, None) => {
                    stack.push(White);
                    if stack.len() == 5 {
                        return GameResult::WhiteWon;
                    }
                }
                (White, _) => {
                    stack.clear();
                    stack.push(White);
                }
            }
        }
        if has_space {
            GameResult::NotFinished
        } else {
            GameResult::Draw
        }
    }
    pub fn line<'a>(&'a self, pos: (i32, i32), dir: (i32, i32)) -> BoardLineIterator<'a> {
        BoardLineIterator {
            p: pos,
            v: dir,
            cells: &self.cells,
        }
    }
    pub fn result(&self) -> GameResult {
        let mut res = GameResult::Draw;
        for n in 0..MAP_SIZE {
            res = res.or(Self::check_line(self.line((0, n as i32), (1, 0))));
            res = res.or(Self::check_line(self.line((n as i32, 0), (0, 1))));

            res = res.or(Self::check_line(self.line((0, n as i32), (1, 1))));
            res = res.or(Self::check_line(self.line((0, n as i32), (1, -1))));
        }
        for n in 1..MAP_SIZE {
            res = res.or(Self::check_line(self.line((n as i32, 0), (1, 1))));
            res = res.or(Self::check_line(
                self.line((n as i32, (MAP_SIZE - 1) as i32), (1, -1)),
            ));
        }
        res
    }
}
