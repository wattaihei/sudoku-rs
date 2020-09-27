use rand::seq::SliceRandom;
use super::utils::*;
use super::sudoku_solve::solve;


fn fullboard() -> [i32; BOARDSIZE] {
    let mut board = [0; BOARDSIZE];

    let mut nums_first_row : Vec<i32> = (1..UNIT+1).collect();
    let mut rng = rand::thread_rng();
    nums_first_row.shuffle(&mut rng);
    // 1行目
    for col in 0..UNIT {
        board[col as usize] = nums_first_row[col as usize];
    }

    // 1ブロック目
    let mut nums_first_block = vec![];
    for i in BLOCKUNIT..UNIT {
        nums_first_block.push(nums_first_row[i as usize]);
    }
    rng = rand::thread_rng();
    nums_first_block.shuffle(&mut rng);
    let mut nums_first_col = vec![];
    for row in 0..BLOCKUNIT {
        for col in 0..BLOCKUNIT {
            if row != 0 {
                board[(row*UNIT+col) as usize] = nums_first_block[((row-1)*BLOCKUNIT+col) as usize]
            }
            if col != 0 {
                nums_first_col.push(board[(row*UNIT+col) as usize]);
            }
        }
    }

    // 1列目
    rng = rand::thread_rng();
    nums_first_col.shuffle(&mut rng);
    for row in BLOCKUNIT..UNIT {
        board[(row*UNIT) as usize] = nums_first_col[(row-BLOCKUNIT) as usize];
    }

    // 解の１つ
    let res = solve(board, 5);

    // 数字の入れ替え
    let mut nums_convert : Vec<i32> = (1..UNIT+1).collect();
    rng = rand::thread_rng();
    nums_convert.shuffle(&mut rng);
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row*UNIT+col) as usize;
            board[ind] = nums_convert[(res.2[ind]-1) as usize];
        }
    }
    board
}

fn make_deleting(level : i32) -> [i32; BOARDSIZE] {
    let mut board = fullboard();
    let mut surviving_indexes : Vec<i32> = (0..BOARDSIZE as i32).collect();
    loop {
        let mut rng = rand::thread_rng();
        surviving_indexes.shuffle(&mut rng);
        let mut removed = false; 
        for i in 0..surviving_indexes.len() {
            let ind = surviving_indexes[i] as usize;
            let pre_num = board[ind];
            board[ind] = 0;
            let res = solve(board, level);
            if res.0 && res.1 {
                surviving_indexes.remove(i);
                removed = true;
                break;
            }
            board[ind] = pre_num;
        }
        if !removed {
            break;
        }
    }

    board
}

#[allow(dead_code)]
fn make_filling(_level : i32) -> [i32; BOARDSIZE] {
    let mut board = [0; BOARDSIZE];

    let mut undefined_indexes = vec![];
    for ind in 0..BOARDSIZE {
        undefined_indexes.push(ind);
    }
    let mut rng = rand::thread_rng();
    undefined_indexes.shuffle(&mut rng);
    
    // 最強レベルで解けるかつ唯一の解を作成
    let mut completed : bool = false;
    for ind in undefined_indexes {
        if board[ind] != 0 { continue; }
        let mut prob_nums = match_nums(board, (ind as i32)/UNIT, (ind as i32)%UNIT);
        rng = rand::thread_rng();
        prob_nums.shuffle(&mut rng);
        for num in prob_nums {
            board[ind] = num;
            let res = solve(board, 5);
            if res.0 {
                if res.1 {
                    completed = true;
                }
                break;
            }
        }
        if completed {
            break;
        }
    }
    board
}


pub fn make(level : i32) -> [i32; BOARDSIZE] {
    make_deleting(level)
}