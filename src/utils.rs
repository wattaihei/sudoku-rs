pub const BLOCKUNIT : i32 = 3;
pub const UNIT : i32 = BLOCKUNIT*BLOCKUNIT;
pub const BOARDSIZE : usize = (UNIT*UNIT) as usize;

pub fn print_board_probables(board_probables : &Vec<Vec<i32>>) {
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row*UNIT + col) as usize;
            if board_probables[ind].len() == 1 {
                print!("{}, ", board_probables[ind][0]);
            } else {
                print!("0, ");
            }
        }
        println!("");
    }
}

pub fn print_board(board : [i32; BOARDSIZE]) {
    println!("state:");
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row * UNIT + col) as usize;
            print!("{}, ", board[ind]);
        }
        println!("");
    }
}

pub fn board_to_probables(board : [i32; BOARDSIZE]) -> Vec<Vec<i32>> {
    let mut board_probables = vec![vec![]; BOARDSIZE];
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row*UNIT + col) as usize;
            if board[ind] == 0 {
                for num in 1..UNIT+1 {
                    board_probables[ind].push(num);
                }
            } else {
                board_probables[ind].push(board[ind]);
            }
        }
    }
    board_probables
}

pub fn probables_to_board(board_probables : Vec<Vec<i32>>) -> [i32; BOARDSIZE] {
    let mut board = [0; BOARDSIZE];
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row*UNIT + col) as usize;
            if board_probables[ind].len() == 1 {
                board[ind] = board_probables[ind][0];
            }
        }
    }
    board
}


// if board[row][col] can follow rules
pub fn follow_rules_cell(
    board_probables : &Vec<Vec<i32>>,
    row : i32,
    col : i32,
    num : i32
) -> bool {
    for r in 0..UNIT {
        let ind = (r*UNIT+col) as usize;
        if board_probables[ind].len() == 1 && board_probables[ind][0] == num {
            return false;
        }
    }
    for c in 0..UNIT {
        let ind = (row*UNIT+c) as usize;
        if board_probables[ind].len() == 1 && board_probables[ind][0] == num {
            return false;
        }
    }
    let start_r = (row/BLOCKUNIT)*BLOCKUNIT;
    let start_c = (col/BLOCKUNIT)*BLOCKUNIT;
    for r in start_r..start_r+BLOCKUNIT {
        for c in start_c..start_c+BLOCKUNIT {
            let ind = (r*UNIT+c) as usize;
            if board_probables[ind].len() == 1 && board_probables[ind][0] == num {
                return false;
            }
        }
    }
    true
}

pub fn follow_rules_board(board_probables : &Vec<Vec<i32>>) -> bool {
    for r in 0..UNIT {
        let mut row_nums = [0; (UNIT+1) as usize];
        for c in 0..UNIT {
            let ind = r*UNIT+c;
            if board_probables[ind as usize].len() == 1 {
                row_nums[board_probables[ind as usize][0] as usize] += 1;
            } else if board_probables[ind as usize].len() == 0 {
                return false;
            }
        }
        for num in 1..UNIT+1 {
            if row_nums[num as usize] > 1 {
                return false;
            }
        }
    }

    for c in 0..UNIT {
        let mut col_nums = [0; (UNIT+1) as usize];
        for r in 0..UNIT {
            let ind = r*UNIT+c;
            if board_probables[ind as usize].len() == 1 {
                col_nums[board_probables[ind as usize][0] as usize] += 1;
            }
        }
        for num in 1..UNIT+1 {
            if col_nums[num as usize] > 1 {
                return false;
            } 
        }
    }

    for start_r in 0..BLOCKUNIT {
        for start_c in 0..BLOCKUNIT {
            let mut block_nums = [0; (UNIT+1) as usize];
            for r in start_r*BLOCKUNIT..(start_r+1)*BLOCKUNIT {
                for c in start_c*BLOCKUNIT..(start_c+1)*BLOCKUNIT {
                    let ind = r*UNIT+c;
                    if board_probables[ind as usize].len() == 1 {
                        block_nums[board_probables[ind as usize][0] as usize] += 1;
                    }
                }
            }
            for num in 1..UNIT+1 {
                if block_nums[num as usize] > 1 {
                    return false;
                }
            }
        }
    }

    true
}

pub fn complete_board(board_probables : &Vec<Vec<i32>>) -> bool {
    for ind in 0..BOARDSIZE {
        if board_probables[ind].len() != 1 {
            return false;
        }
    }
    true
}


pub fn match_nums(board : [i32; BOARDSIZE], row : i32, col : i32) -> Vec<i32> {
    let filled_num = board[(row*UNIT + col) as usize];
    if filled_num != 0 {
        return vec![filled_num];
    }
    let mut used_nums = [false; (UNIT+1) as usize];
    for c in 0..UNIT {
        let ind = row*UNIT + c;
        used_nums[board[ind as usize] as usize] = true;
    }
    for r in 0..UNIT {
        let ind = r*UNIT + col;
        used_nums[board[ind as usize] as usize] = true;
    }
    let start_r = BLOCKUNIT*(row/BLOCKUNIT);
    let start_c = BLOCKUNIT*(col/BLOCKUNIT);
    for r in start_r..start_r+BLOCKUNIT {
        for c in start_c..start_c+BLOCKUNIT {
            let ind = r*UNIT + c;
            used_nums[board[ind as usize] as usize] = true;
        }
    }

    let mut unused_vec = vec![];
    for num in 1..UNIT+1 {
        if !used_nums[num as usize] {
            unused_vec.push(num);
        }
    }
    unused_vec
}

pub fn count_blank(board_probables : &Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    for ind in 0..BOARDSIZE {
        if board_probables[ind].len() > 1 {
            cnt += 1;
        }
    }
    cnt
}