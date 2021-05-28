//Tri Le
//CS410 RUST
use std::cmp::PartialEq;
use std::collections::HashMap;
// use std::ops::{Index, IndexMut};
pub const MAX: usize = 20;
pub const DISTANCE: usize = 3;
pub const MAX_DEPTH: usize = 6;
pub const MAX_WIDTH: usize = 10;
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub side: u8,
    pub point_dic: Vec<Point>,
    pub total_score: i32,
}

impl Player {
    pub fn new(side: u8) -> Self {
        Self {
            side,
            point_dic: Vec::new(),
            total_score: 0,
        }
    }

    // fn update(&mut self, point: Point, count: u16) {
    //     if self.point_dic.contains_key(&point) {
    //         let change = count as i32 - self.point_dic[&point] as i32;
    //         self.total_score += change;
    //         self.point_dic.entry(point).and_modify(|e| *e = count);
    //     } else {
    //         self.point_dic.insert(point, count);
    //         self.total_score += count as i32;
    //     }
    // }
    




    fn check_horizon(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test: &mut i32,
    ) -> bool {
        let mut adjacent_count = 1;
        let mut count = 0;
        let mut count_adjacent_flag = true;
        let mut count_flag = true;
        let mut count_space = 0;
        let mut first_space_count = 0;
        for c in 1..5 {
            if point.y + c == MAX {
                break;
            }
            if matrix[point.x][point.y + c] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x][point.y + c] != self.opposite_side() {
                if count_flag {
                    if matrix[point.x][point.y + c] == self.side {
                        count += 1;
                    } else {
                        count_space+=1;
                        if c == 1 {
                            first_space_count += 1;
                        }
                    }
                }
            } else {
                count_flag = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }
        count_adjacent_flag = true;
        let mut count_flag2 = true;
        count_space = 0;
        for c in 1..5 {
            if point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x][point.y - c] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x][point.y - c] != self.opposite_side() {
                if count_flag2 {
                    if matrix[point.x][point.y - c] == self.side {
                        count += 1;
                    } else {
                        count_space +=1;
                        if c == 1 {
                            first_space_count +=1;
                        }
                    }
                }
            } else {
                count_flag2 = false;
            }
            if (count_adjacent_flag == false && count_flag2 == false) || count_space == 2{
                break;
            }
        }
        if *test == std::i32::MAX {          
            if (count_flag == false || count_flag2 == false)  && count < 4 {
                count = 0;
            }
            if first_space_count == 2 {
                count -= 1;
            }
            *test = count;
        }
        if adjacent_count >= 5 {
            return true;
        }
        false
    }

    fn check_vertical(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test: &mut i32,
    ) -> bool {
        let mut adjacent_count = 1;
        let mut count = 0;
        let mut count_adjacent_flag = true;
        let mut count_flag = true;
        let mut count_space = 0;
        let mut first_space_count = 0;
        for c in 1..5 {
            if point.x + c == MAX {
                break;
            }
            if matrix[point.x + c][point.y] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x + c][point.y] != self.opposite_side() {
                if count_flag {
                    if matrix[point.x + c][point.y] == self.side {
                        count += 1;
                    } else {
                        count_space +=1;
                        if c == 1 {
                            first_space_count += 1;
                        }
                    }
                }
            } else {
                count_flag = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }
        count_adjacent_flag = true;
        let mut count_flag2 = true;
        count_space = 0;
        for c in 1..5 {
            if point.x as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x - c][point.y] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x - c][point.y] != self.opposite_side() {
                if count_flag2 {
                    if matrix[point.x - c][point.y] == self.side {
                        count += 1;
                    } else {
                        count_space +=1;
                        if c == 1 { 
                            first_space_count += 1
                        }
                    }
                }
            } else {
                count_flag2 = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }
        if *test == std::i32::MAX {
            if (count_flag == false || count_flag2 == false) && count < 4 {
                count = 0;
            }
            if first_space_count == 2 {
                count -=1;
            }
            if count_flag && count_flag2 && count > 1 {
                count += 1;
            }
            *test = count;
            // if count > 1 {
            //     *test += adjacent_count;
            // }
        }
        if adjacent_count >= 5 {
            return true;
        }
        false
    }

    fn check_cross_down(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test: &mut i32,
    ) -> bool {
        let mut adjacent_count = 1;
        let mut count = 0;
        let mut count_adjacent_flag = true;
        let mut count_flag = true;
        let mut count_space = 0;
        let mut first_space_count = 0;
        for c in 1..5 {
            if point.x + c == MAX || point.y + c == MAX {
                break;
            }
            if matrix[point.x + c][point.y + c] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }

            if matrix[point.x + c][point.y + c] != self.opposite_side() {
                if count_flag {
                    if matrix[point.x + c][point.y + c] == self.side {
                        count += 1;
                    } else {
                        count_space += 1;
                        if c  == 1 {
                            first_space_count +=1;
                        } 
                    }
                }
            } else {
                count_flag = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }

        count_adjacent_flag = true;
        let mut count_flag2 = true;
        count_space = 0;
        for c in 1..5 {
            if point.x as i16 - c as i16 == -1 || point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x - c][point.y - c] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x - c][point.y - c] != self.opposite_side() {
                if count_flag2 {
                    if matrix[point.x - c][point.y - c] == self.side {
                        count += 1;
                    } else {
                        count_space += 1;
                        if c == 1 {
                            first_space_count +=1;
                        }
                    }
                }
            } else {
                count_flag2 = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }
        if *test == std::i32::MAX {
            if (count_flag == false || count_flag2 == false) && count < 4 {
                count = 0;
            }
            if first_space_count == 2 {
                count -= 1;
            }
            if count_flag && count_flag2 && count > 1 {
                count += 1;
            }
            *test = count;
            // if count > 1 {
            //     *test += adjacent_count;
            // }
        }
        if adjacent_count >= 5 {
            return true;
        }
        false
    }

    fn check_cross_up(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test: &mut i32,
    ) -> bool {
        let mut adjacent_count = 1;
        let mut count = 0;
        let mut count_adjacent_flag = true;
        let mut count_flag = true;
        let mut count_space = 0;
        let mut first_space_count = 0;
        for c in 1..5 {
            if point.x as i16 - c as i16 == -1 || point.y + c == MAX {
                break;
            }
            if matrix[point.x - c][point.y + c] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x - c][point.y + c] != self.opposite_side() {
                if count_flag && matrix[point.x - c][point.y + c] == self.side {
                    if matrix[point.x - c][point.y + c] == self.side {
                        count += 1;
                    } else {
                        count_space += 1;
                        if c == 1 {
                            first_space_count +=1;
                        }
                    }
                }
            } else {
                count_flag = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }

        count_adjacent_flag = true;
        let mut count_flag2 = true;
        count_space = 0;
        for c in 1..5 {
            if point.x + c == MAX || point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x + c][point.y - c] == self.side {
                if count_adjacent_flag {
                    adjacent_count += 1;
                }
            } else {
                count_adjacent_flag = false;
            }
            if matrix[point.x + c][point.y - c] != self.opposite_side() {
                if count_flag2 {
                    if matrix[point.x + c][point.y - c] == self.side {
                        count += 1;
                    } else {
                        count_space +=1;
                        if c == 1 {
                            first_space_count +=1;
                        }
                    }
                }
            } else {
                count_flag2 = false;
            }
            if (count_adjacent_flag == false && count_flag == false) || count_space == 2 {
                break;
            }
        }
        if *test == std::i32::MAX {
            if (count_flag == false || count_flag2 == false ) && count < 4 {
                count = 0;
            }
            if first_space_count == 2 {
                count -=1;
            }
            if count_flag && count_flag2 && count > 1 {
                count += 1;
            }
            *test = count;
        }
        if adjacent_count >= 5 {
            return true;
        }
        false
    }

    //Return true -> this player is the winner
    pub fn add_new_point(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test: &mut i32,
    ) -> bool {
        let mut yes_test = false;
        if *test == std::i32::MAX {
            yes_test = true;
        }
        let origin_test = *test;
        let mut test_temp = 0;
        matrix[point.x][point.y] = self.side;

        if self.check_horizon(Point::new(point.x, point.y), matrix, test) {
            if yes_test {
                matrix[point.x][point.y] = 0;
            } else {
                self.point_dic.push(point);
            }
            return true;
        }

        if yes_test {
            if *test > 0 {
                test_temp += *test + 1;
            }    
            *test = std::i32::MAX;
        }

        if self.check_vertical(Point::new(point.x, point.y), matrix, test) {
            if *test == std::i32::MAX {
                matrix[point.x][point.y] = 0;
            } else {
                self.point_dic.push(point);
            }
            return true;
        }

        if yes_test {
            if *test > 0 {
                test_temp += *test + 1;
            }
            *test = std::i32::MAX;
        }

        if self.check_cross_down(Point::new(point.x, point.y), matrix, test) {
            if *test == std::i32::MAX {
                matrix[point.x][point.y] = 0;
            } else {
                self.point_dic.push(point);
            }
            return true;
        }

        if yes_test {
            if *test > 0 {
                test_temp += *test + 1;
            }
            *test = std::i32::MAX;
        }

        if self.check_cross_up(Point::new(point.x, point.y), matrix, test) {
            if *test == std::i32::MAX {
                matrix[point.x][point.y] = 0;
            } else {
                self.point_dic.push(point);
            }
            return true;
        }

        if yes_test {
            matrix[point.x][point.y] = 0;
            if *test > 0 {
                test_temp += *test + 1;
            }
            *test = test_temp;
        } else {
            *test = origin_test;
            self.point_dic.push(point);
        }
        false
    }

    fn opposite_side(&self) -> u8 {
        if self.side == 1 {
            2
        } else {
            1
        }
    }
}
pub const WIN: i16 = 10;
pub const LOSE: i16 = -10;
pub const DRAW: i16 = 0;
pub const UNKNOWN: i16 = std::i16::MAX;

pub fn find_best_move(
    mut ai: Player,
    mut user: Player,
    mut matrix: [[u8; MAX]; MAX],
) -> Option<Point> {
    let mut best_move = None;
    let mut hash: HashMap<Point, i32> = HashMap::new();
    get_all_board_move(&mut hash, ai.point_dic.clone(), &matrix);
    get_all_board_move(&mut hash, user.point_dic.clone(), &matrix);
    let new_board = hash.keys().cloned().collect::<Vec<Point>>();
    let mut max = std::i16::MIN;
    let mut final_board: Vec<(Point, i32)> = Vec::new();
    for point in new_board {
        let mut test_for_ai = std::i32::MAX;
        let mut test_for_user = std::i32::MAX;
        if ai.add_new_point(point.clone(), &mut matrix, &mut test_for_ai) {
            return Some(point.clone());
        }
        if user.add_new_point(point.clone(), &mut matrix, &mut test_for_user) {
            return Some(point.clone());
        }
        if test_for_user >= test_for_ai {
            final_board.push((point, test_for_user + 1));
        } else {
            final_board.push((point, test_for_ai));
        }
    }
    final_board.sort_by(|a, b| a.1.cmp(&b.1));
    final_board.reverse();
    println!("Considering points:");
    for (point, score) in &final_board {
        print!("({},{})={} ", point.x, point.y, score);
    }
    println!("");
    // for (point, _) in &final_board {
    //     if matrix[point.x][point.y] == 0 {
    //         let value = minimax(
    //             0,
    //             ai.side.clone(),
    //             point.clone(),
    //             ai.clone(),
    //             user.clone(),
    //             final_board.len() as i32,
    //             matrix,
    //         );
    //         if value != UNKNOWN && value > max {
    //             best_move = Some(point.clone());
    //             max = value;
    //         }
    //     }
    // }
    if final_board.len() > 1 {
        let (result, _) = &final_board[0];
        return Some(result.clone());
    }
    best_move
}

pub fn check_state(
    depth: u8,
    turn: u8,
    cur_move: Point,
    ai: &mut Player,
    user: &mut Player,
    board_len: i32,
    matrix: &mut [[u8; MAX]; MAX],
) -> i16 {
    let mut test = 0; // no test

    if board_len == 0 {
        return DRAW;
    }
    if turn == ai.side {
        if ai.add_new_point(cur_move, matrix, &mut test) {
            return WIN - depth as i16;
        }
    } else if user.add_new_point(cur_move, matrix, &mut test) {
        return LOSE + depth as i16;
    }
    UNKNOWN
}

pub fn minimax(
    depth: u8,
    mut turn: u8,
    cur_move: Point,
    mut ai: Player,
    mut user: Player,
    board_len: i32,
    mut matrix: [[u8; MAX]; MAX],
) -> i16 {
    let check = check_state(
        depth,
        turn,
        cur_move,
        &mut ai,
        &mut user,
        board_len,
        &mut matrix,
    );
    if check != UNKNOWN || depth == 4 {
        return check;
    }

    let mut hash: HashMap<Point, i32> = HashMap::new();
    get_all_board_move(&mut hash, ai.point_dic.clone(), &matrix);
    get_all_board_move(&mut hash, user.point_dic.clone(), &matrix);
    let new_board = hash.keys().cloned().collect::<Vec<Point>>();

    //change turn
    if turn == ai.side {
        turn = ai.opposite_side();
    } else {
        turn = ai.side;
    }

    if turn == ai.side {
        //FIND MAX
        let mut max = std::i16::MIN;
        let mut final_board: Vec<(Point, i32)> = Vec::new();
        for point in new_board {
            let mut test_for_ai = std::i32::MAX;
            ai.add_new_point(point.clone(), &mut matrix, &mut test_for_ai);
            final_board.push((point, test_for_ai));
        }
        final_board.sort_by(|a, b| a.1.cmp(&b.1));
        final_board.reverse();
        let mut count_width = 0;
        for (point, _) in &final_board {
            if matrix[point.x][point.y] == 0 {
                let value = minimax(
                    depth + 1,
                    turn,
                    point.clone(),
                    ai.clone(),
                    user.clone(),
                    final_board.len() as i32,
                    matrix.clone(),
                );
                if count_width > MAX_WIDTH {
                    return value;
                }
                max = std::cmp::max(max, value);
                count_width += 1;
            }
        }
        max
    } else {
        //FIND MIN
        let mut min = std::i16::MIN;
        let mut final_board: Vec<(Point, i32)> = Vec::new();
        for point in new_board {
            let mut test_for_user = std::i32::MAX;
            user.add_new_point(point.clone(), &mut matrix, &mut test_for_user);
            final_board.push((point, test_for_user));
        }
        final_board.sort_by(|a, b| a.1.cmp(&b.1));
        final_board.reverse();
        let mut count_width = 0;

        for (point, _) in &final_board {
            if matrix[point.x][point.y] == 0 {
                let value = minimax(
                    depth + 1,
                    turn,
                    point.clone(),
                    ai.clone(),
                    user.clone(),
                    final_board.len() as i32,
                    matrix.clone(),
                );
                if count_width > MAX_WIDTH {
                    return value;
                }
                count_width += 1;
                min = std::cmp::min(min, value);
            }
        }
        min
    }
}

pub fn get_all_board_move(
    result: &mut HashMap<Point, i32>,
    local_point: Vec<Point>,
    matrix: &[[u8; MAX]; MAX],
) {
    // let mut result = Vec::new();
    for point in local_point {
        //Horizon
        for c in 1..DISTANCE {
            if point.y + c == MAX {
                break;
            }
            if matrix[point.x][point.y + c] == 0 {
                result.entry(Point::new(point.x, point.y + c)).or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x][point.y - c] == 0 {
                result.entry(Point::new(point.x, point.y - c)).or_insert(0);
            } else {
                break;
            }
        }
        //Vertical
        for c in 1..DISTANCE {
            if point.x + c == MAX {
                break;
            }
            if matrix[point.x + c][point.y] == 0 {
                result.entry(Point::new(point.x + c, point.y)).or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.x as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x - c][point.y] == 0 {
                result.entry(Point::new(point.x - c, point.y)).or_insert(0);
            } else {
                break;
            }
        }
        //Cross down
        for c in 1..DISTANCE {
            if point.x + c == MAX || point.y + c == MAX {
                break;
            }
            if matrix[point.x + c][point.y + c] == 0 {
                result
                    .entry(Point::new(point.x + c, point.y + c))
                    .or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.x as i16 - c as i16 == -1 || point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x - c][point.y - c] == 0 {
                result
                    .entry(Point::new(point.x - c, point.y - c))
                    .or_insert(0);
            } else {
                break;
            }
        }
        //Cross up
        for c in 1..DISTANCE {
            if point.x as i16 - c as i16 == -1 || point.y + c == MAX {
                break;
            }
            if matrix[point.x - c][point.y + c] == 0 {
                result
                    .entry(Point::new(point.x - c, point.y + c))
                    .or_insert(0);
            } else {
                break;
            }
        }
        for c in 1..DISTANCE {
            if point.x + c == MAX || point.y as i16 - c as i16 == -1 {
                break;
            }
            if matrix[point.x + c][point.y - c] == 0 {
                result
                    .entry(Point::new(point.x + c, point.y - c))
                    .or_insert(0);
            } else {
                break;
            }
        }
    }
}

//pub minimax(all_board_move,)
