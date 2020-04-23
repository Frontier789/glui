extern crate glui;
extern crate rand;
extern crate rusty;
use rand::Rng;
use std::collections::HashMap;
// use tools::*;

use board::*;
use std::cmp::*;

pub fn ai_move(board: &mut Board) -> GameResult {
    
    // let result;
    // loop {
    //     let x = rand::thread_rng().gen_range(0, MAP_SIZE);
    //     let y = rand::thread_rng().gen_range(0, MAP_SIZE);
    //     if board.cell(x, y) == Cell::Empty {
    //         let res = board.put(x, y);
            
    //         result = res;
    //         break;
    //     }
    // }
    
    let mut ai_board = BoardAsNums::new(Cell::White);
    let mut player_board = BoardAsNums::new(Cell::Black);
    for m in board.moves() {
        ai_board.put(m.0, m.1);
        player_board.put(m.0, m.1);
    }
    
    for n in 0..MAP_SIZE {
        for k in 0..MAP_SIZE {
            if board.cell(n, k) == Cell::Empty {
                let val_ai = ai_board.combos;
                let val_pl = player_board.combos;
                
                ai_board.put(n, k);
                player_board.put(n, k);
                if ai_board.over {
                    board.heat[n][k] = 100.0;
                } else {
                    board.heat[n][k] = minmax(&mut ai_board, &mut player_board, false, 0.51, 1);
                }
                player_board.undo();
                ai_board.undo();
                
                assert_eq!(val_ai, ai_board.combos);
                assert_eq!(val_pl, player_board.combos);
            } else {
                board.heat[n][k] = 0.0;
            }
        }
    }
    
    // println!("heatmap: [");
    // for i in 0..MAP_SIZE {
    //     println!("{:?},",board.heat[i]);
    // }
    // println!("]");
    
    
    let mut max_heat = std::f32::MIN;
    let mut min_heat = std::f32::MAX;
    
    for n in 0..MAP_SIZE {
        for k in 0..MAP_SIZE {
            if board.cell(n, k) == Cell::Empty {
                if max_heat < board.heat[n][k] { max_heat = board.heat[n][k]; }
                if min_heat > board.heat[n][k] { min_heat = board.heat[n][k]; }
            }
        }
    }
    
    let mut cands = Vec::new();
    
    for n in 0..MAP_SIZE {
        for k in 0..MAP_SIZE {
            if board.cell(n, k) == Cell::Empty {
                if board.heat[n][k] == max_heat {
                    cands.push((n,k));
                }
                if min_heat < max_heat {
                    board.heat[n][k] = (board.heat[n][k] - min_heat) / (max_heat - min_heat);
                } else {
                    board.heat[n][k] = 1.0;
                }
            } else {
                board.heat[n][k] = 0.0;
            }
            // board.heat[n][k] = 0.0;
        }
    }
    
    // println!("max: {}, min: {}", max_heat, min_heat);
    // println!("{:#?}\n\n", ai_board.combos);
    
    let p = cands[rand::thread_rng().gen_range(0, cands.len())];
    
    board.put(p.0, p.1)
}

fn minmax(my_board: &mut BoardAsNums, enemy_board: &mut BoardAsNums, my_turn: bool, aggression: f32, depth: u32) -> f32 {
    
    if depth == 0 {
        let my_val = my_board.combos.eval() as f32;
        let enemy_val = enemy_board.combos.eval() as f32;
        
        if my_turn {
            return my_val * aggression - enemy_val * (1.0 - aggression);
        } else {
            return my_val * (1.0 - aggression) - enemy_val * aggression;
        }
    }
    
    let mut mx = std::f32::MIN;
    let mut mn = std::f32::MAX;
    
    for n in 0..MAP_SIZE {
        for k in 0..MAP_SIZE {
            if my_board.cells[n][k] == Cell::Empty {
                my_board.put(n, k);
                enemy_board.put(n, k);
                
                let val = if my_board.over {
                    if my_turn {
                        100.0
                    } else {
                        -100.0
                    }
                } else {
                    minmax(my_board, enemy_board, !my_turn, aggression, depth-1)
                };
                
                if mx < val { mx = val; }
                if mn > val { mn = val; }
                
                enemy_board.undo();
                my_board.undo();
            }
        }
    }
    
    if my_turn {
        mx
    } else {
        mn
    }
}

#[derive(Debug, Default, Copy, Clone, Eq)]
struct Combinations {
    fives: u8,
    open_fours: u8,
    half_open_fours: u8,
    open_threes: u8,
    half_open_threes: u8,
    open_twos: u8,
}

impl PartialEq for Combinations {
    fn eq(&self, other: &Self) -> bool {
        if min(self.fives, other.fives) > 0 {
            return self.fives == other.fives;
        }
        
        if min(self.open_fours, other.open_fours) > 0 {
            return self.open_fours == other.open_fours;
        }
        
        self.eval() == other.eval()
    }
}
impl PartialOrd for Combinations {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if min(self.fives, other.fives) > 0 {
            return self.fives.partial_cmp(&other.fives);
        }
        
        if min(self.open_fours, other.open_fours) > 0 {
            return self.open_fours.partial_cmp(&other.open_fours);
        }
        
        self.eval().partial_cmp(&other.eval())
    }
}

impl std::ops::AddAssign for Combinations {
    fn add_assign(&mut self, rhs: Combinations) {
        self.fives += rhs.fives;
        self.open_fours += rhs.open_fours;
        self.half_open_fours += rhs.half_open_fours;
        self.open_threes += rhs.open_threes;
        self.half_open_threes += rhs.half_open_threes;
        self.open_twos += rhs.open_twos;
    }
}

impl std::ops::SubAssign for Combinations {
    fn sub_assign(&mut self, rhs: Combinations) {
        self.fives -= rhs.fives;
        self.open_fours -= rhs.open_fours;
        self.half_open_fours -= rhs.half_open_fours;
        self.open_threes -= rhs.open_threes;
        self.half_open_threes -= rhs.half_open_threes;
        self.open_twos -= rhs.open_twos;
    }
}

impl Combinations {
    fn eval(&self) -> f64 {
        self.fives as f64            * 100.0 + 
        self.open_fours as f64       * 10.0 + 
        self.half_open_fours as f64  * 5.0 + 
        self.open_threes as f64      * 1.0 + 
        self.half_open_threes as f64 * 0.1 +
        self.open_twos as f64        * 0.01
    }
}

#[derive(Debug, Default)]
struct BoardAsNums {
    rows: [u32; MAP_SIZE],
    cols: [u32; MAP_SIZE],
    rds:  [u32; MAP_SIZE*2-1],
    rus:  [u32; MAP_SIZE*2-1],
    cells: [[Cell; MAP_SIZE]; MAP_SIZE],
    moves: Vec<(usize,usize)>,
    combos: Combinations,
    player: Cell,
    over: bool,
    num_to_combos: [HashMap<u32, Combinations>; MAP_SIZE + 1],
}

fn proj_rds(x: usize,y: usize) -> (i32,i32) {
    if x < y {
        (0, (y-x) as i32)
    } else {
        ((x-y) as i32, 0)
    }
}
fn len_rds(x: usize,y: usize) -> usize {
    MAP_SIZE - (x as i32 - y as i32).abs() as usize
}
fn index_rds(x: usize,y: usize) -> usize {
    (x as i32 - y as i32 + MAP_SIZE as i32 - 1) as usize
}
fn adv_rds(x: usize,y: usize) -> usize {
    min(x,y)
}

fn proj_rus(x: usize,y: usize) -> (i32,i32) {
    if x+y < MAP_SIZE-1 {
        (0, (y+x) as i32)
    } else {
        ((x+y-(MAP_SIZE-1)) as i32, (MAP_SIZE-1) as i32)
    }
}
fn len_rus(x: usize,y: usize) -> usize {
    MAP_SIZE - ((x + y) as i32 - (MAP_SIZE-1) as i32).abs() as usize
}
fn index_rus(x: usize,y: usize) -> usize {
    x + y
}
fn adv_rus(x: usize,y: usize) -> usize {
    min(x,MAP_SIZE-1-y)
}

impl BoardAsNums {
    pub fn new(player: Cell) -> BoardAsNums {
        BoardAsNums {
            player,
            ..Default::default()
        }
    }
    
    pub fn put(&mut self, x: usize, y: usize) {
        let colmod = (self.moves.len()%2) as u32 + 1;
        self.cells[x][y] = if self.moves.len()%2==0 {Cell::Black} else {Cell::White};
        self.moves.push((x,y));
        
        let sub = self.evaluate((0,y as i32), (1,0), self.cols[y],MAP_SIZE);
        self.combos -= sub;
        self.cols[y] += 3u32.pow(x as u32) * colmod;
        let add = self.evaluate((0,y as i32), (1,0), self.cols[y],MAP_SIZE);
        self.combos += add;
        
        let sub = self.evaluate((x as i32,0), (0,1), self.rows[x],MAP_SIZE);
        self.combos -= sub;
        self.rows[x] += 3u32.pow(y as u32) * colmod;
        let add = self.evaluate((x as i32,0), (0,1), self.rows[x],MAP_SIZE);
        self.combos += add;
        
        let sub = self.evaluate(proj_rds(x,y), (1,1), self.rds[index_rds(x,y)],len_rds(x,y));
        self.combos -= sub;
        self.rds[index_rds(x,y)] += 3u32.pow(adv_rds(x,y) as u32) * colmod;
        let add = self.evaluate(proj_rds(x,y), (1,1), self.rds[index_rds(x,y)],len_rds(x,y));
        self.combos += add;
        
        let sub = self.evaluate(proj_rus(x,y), (1,-1), self.rus[index_rus(x,y)],len_rus(x,y));
        self.combos -= sub;
        self.rus[index_rus(x,y)] += 3u32.pow(adv_rus(x,y) as u32) * colmod;
        let add = self.evaluate(proj_rus(x,y), (1,-1), self.rus[index_rus(x,y)],len_rus(x,y));
        self.combos += add;
    }
    
    pub fn undo(&mut self) {
        if let Some((x,y)) = self.moves.pop() {
            let colmod = (self.moves.len()%2) as u32 + 1;
            self.cells[x][y] = Cell::Empty;
            self.over = false;
            
            let sub = self.evaluate((0,y as i32), (1,0), self.cols[y],MAP_SIZE);
            self.combos -= sub;
            self.cols[y] -= 3u32.pow(x as u32) * colmod;
            let add = self.evaluate((0,y as i32), (1,0), self.cols[y],MAP_SIZE);
            self.combos += add;
            
            let sub = self.evaluate((x as i32,0), (0,1), self.rows[x],MAP_SIZE);
            self.combos -= sub;
            self.rows[x] -= 3u32.pow(y as u32) * colmod;
            let add = self.evaluate((x as i32,0), (0,1), self.rows[x],MAP_SIZE);
            self.combos += add;
            
            let sub = self.evaluate(proj_rds(x,y), (1,1), self.rds[index_rds(x,y)],len_rds(x,y));
            self.combos -= sub;
            self.rds[index_rds(x,y)] -= 3u32.pow(adv_rds(x,y) as u32) * colmod;
            let add = self.evaluate(proj_rds(x,y), (1,1), self.rds[index_rds(x,y)],len_rds(x,y));
            self.combos += add;
            
            let sub = self.evaluate(proj_rus(x,y), (1,-1), self.rus[index_rus(x,y)],len_rus(x,y));
            self.combos -= sub;
            self.rus[index_rus(x,y)] -= 3u32.pow(adv_rus(x,y) as u32) * colmod;
            let add = self.evaluate(proj_rus(x,y), (1,-1), self.rus[index_rus(x,y)],len_rus(x,y));
            self.combos += add;
        }
    }
    
    pub fn evaluate(&mut self, p: (i32, i32), v: (i32, i32), num: u32, len: usize) -> Combinations {
        if let Some(v) = self.num_to_combos[len].get(&num) {
            *v
        } else {
            let line = self.line(p,v).collect::<Vec<Cell>>();
            let opponent = self.player.opponent();
            let mut ret = Combinations::default();
            let player = self.player;
            
            for i in 0..len {
                if i + 5 <= len && line[i] == player && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == player {
                    ret.fives += 1;
                    self.over = true;
                }
                if i + 6 <= len && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == player && line[i+5] == Cell::Empty {
                    ret.open_fours += 1;
                }
                if i + 6 <= len && line[i] == opponent && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == player && line[i+5] == Cell::Empty {
                    ret.half_open_fours += 1;
                }
                if i + 6 <= len && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == player && line[i+5] == opponent {
                    ret.half_open_fours += 1;
                }
                if len >= 5 && i == 0 && line[i] == player && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == Cell::Empty {
                    ret.half_open_fours += 1;
                }
                if len >= 5 && i == len-5 && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == player {
                    ret.half_open_fours += 1;
                }
                if i + 5 <= len && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == Cell::Empty {
                    ret.open_threes += 1;
                }
                if i + 5 <= len && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == opponent {
                    ret.half_open_threes += 1;
                }
                if i + 5 <= len && line[i] == opponent && line[i+1] == player && line[i+2] == player && line[i+3] == player && line[i+4] == Cell::Empty {
                    ret.half_open_threes += 1;
                }
                if len >= 4 && i == 0 && line[i] == player && line[i+1] == player && line[i+2] == player && line[i+3] == Cell::Empty {
                    ret.half_open_threes += 1;
                }
                if len >= 4 && i == len-4 && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == player {
                    ret.half_open_threes += 1;
                }
                if i + 4 <= len && line[i] == Cell::Empty && line[i+1] == player && line[i+2] == player && line[i+3] == Cell::Empty {
                    ret.open_twos += 1;
                }
            }
            
            self.num_to_combos[len].insert(num, ret);
            ret
        }
    }
    
    pub fn line<'a>(&'a self, pos: (i32, i32), dir: (i32, i32)) -> BoardLineIterator<'a> {
        BoardLineIterator {
            p: pos,
            v: dir,
            cells: &self.cells,
        }
    }
}
