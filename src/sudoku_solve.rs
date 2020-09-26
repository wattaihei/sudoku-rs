use std::collections::HashSet;
use crate::utils::*;


// ブロックごとに見て、数字が一箇所しか埋められないなら埋める
fn fillin_probables_blocks(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    // numごとの決定場所と空欄場所を分ける
    let mut already_defined_locations : Vec<Vec<(i32,i32)>> = vec![vec![]; (UNIT+1) as usize];
    let mut block_blanks : Vec<Vec<HashSet<(i32, i32)>>> = vec![vec![HashSet::new(); BLOCKUNIT as usize]; BLOCKUNIT as usize];
    let mut new_board_probables : Vec<Vec<i32>> = board_probables.clone();
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row * UNIT + col) as usize;
            if board_probables[ind].len() == 1 {
                let num = board_probables[ind][0];
                already_defined_locations[num as usize].push((row, col));
            } else {
                block_blanks[(row/BLOCKUNIT) as usize][(col/BLOCKUNIT) as usize].insert((row, col));
            }
        }
    }
    // 数ごとに見て、埋められるブロックがあれば埋める
    for num in 1..UNIT+1 {
        let mut defined_r = [false; UNIT as usize];
        let mut defined_c = [false; UNIT as usize];
        let mut defined_block = [[false; BLOCKUNIT as usize]; BLOCKUNIT as usize];
        for &(row, col) in &already_defined_locations[num as usize] {
            defined_r[row as usize] = true;
            defined_c[col as usize] = true;
            defined_block[(row/BLOCKUNIT) as usize][(col/BLOCKUNIT) as usize] = true;
        }
        for block_r in 0..BLOCKUNIT {
            for block_c in 0..BLOCKUNIT {
                if defined_block[block_r as usize][block_c as usize] {
                    continue;
                }
                let mut now_blanks = vec![];
                for &(blank_r, blank_c) in &block_blanks[block_r as usize][block_c as usize] {
                    if !defined_r[blank_r as usize] && !defined_c[blank_c as usize] {
                        now_blanks.push(blank_r*UNIT + blank_c);
                    }
                }
                if now_blanks.len() == 1 {
                    let ind = now_blanks[0];
                    new_board_probables[ind as usize] = vec![num];
                    block_blanks[block_r as usize][block_c as usize].remove(&(ind/UNIT, ind%UNIT));
                } else if now_blanks.len() == 0 {
                    return (false, new_board_probables);
                }
            }
        }
    }
    (true, new_board_probables)
}

// 行ごとに見て、数字が一箇所しか埋められないなら埋める
fn fillin_probables_row(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut new_board_probables : Vec<Vec<i32>> = board_probables.clone();
    for row in 0..UNIT {
        let mut remaining = [true; (UNIT+1) as usize];
        let mut blank_col = HashSet::new();
        for col in 0..UNIT {
            let ind = (row*UNIT + col) as usize;
            if board_probables[ind].len() == 1 {
                remaining[board_probables[ind][0] as usize] = false;
            } else {
                blank_col.insert(col);
            }
        }
        for num in 1..UNIT+1 {
            if remaining[num as usize] {
                let mut allowed_blanks = vec![];
                for &col in &blank_col {
                    if follow_rules_cell(&new_board_probables, row, col, num) {
                        allowed_blanks.push(col);
                    }
                }
                if allowed_blanks.len() == 1 {
                    let ind = (row*UNIT + allowed_blanks[0]) as usize;
                    new_board_probables[ind] = vec![num];
                    blank_col.remove(&allowed_blanks[0]);
                } else if allowed_blanks.len() == 0 {
                    return (false, new_board_probables);
                }
            }
        }
    }
    (true, new_board_probables)
}

// 列ごとに見て、数字が一箇所しか埋められないなら埋める
fn fillin_probables_col(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut new_board_probables : Vec<Vec<i32>> = board_probables.clone();
    for col in 0..UNIT {
        let mut remaining = [true; (UNIT+1) as usize];
        let mut blank_row = HashSet::new();
        for row in 0..UNIT {
            let ind = (row*UNIT + col) as usize;
            if board_probables[ind].len() == 1 {
                remaining[board_probables[ind][0] as usize] = false;
            } else {
                blank_row.insert(row);
            }
        }
        for num in 1..UNIT+1 {
            if remaining[num as usize] {
                let mut allowed_blanks = vec![];
                for &row in &blank_row {
                    if follow_rules_cell(&new_board_probables, row, col, num) {
                        allowed_blanks.push(row);
                    }
                }
                if allowed_blanks.len() == 1 {
                    let ind = (allowed_blanks[0]*UNIT + col) as usize;
                    new_board_probables[ind] = vec![num];
                    blank_row.remove(&allowed_blanks[0]);
                } else if allowed_blanks.len() == 0 {
                    return (false, new_board_probables);
                }
            }
        }
    }
    (true, new_board_probables)
}


// １つのマスに注目し、入る数字が１つだけなら決定する
fn fillin_probables_cell(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut new_board_probables = vec![vec![]; BOARDSIZE];
    for row in 0..UNIT {
        for col in 0..UNIT {
            let ind = (row*UNIT + col) as usize;
            if board_probables[ind].len() == 1 {
                new_board_probables[ind] = board_probables[ind].clone();
                continue;
            }
            for &num in &board_probables[ind] {
                if follow_rules_cell(board_probables, row, col, num) {
                    new_board_probables[ind].push(num);
                }
            }
            if new_board_probables[ind].len() == 0 {
                return (false, new_board_probables);
            }
        }
    }
    (true, new_board_probables)
}

// 予約
fn consider_revervation(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let new_board_probables = (*board_probables).clone();

    (true, new_board_probables)
}


fn solve_stage2(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut new_board_probables : Vec<Vec<i32>> = (*board_probables).clone();
    let mut res = fillin_probables_blocks(&new_board_probables);
    while res.1 != new_board_probables {
        if !res.0 { return res };
        new_board_probables = res.1;
        res = fillin_probables_blocks(&new_board_probables);
    }
    (res.0, new_board_probables)
}

fn solve_stage3(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut new_board_probables : Vec<Vec<i32>> = (*board_probables).clone();
    let mut res1 = fillin_probables_row(&new_board_probables);
    let mut res2 = fillin_probables_col(&res1.1);
    let mut res3 = fillin_probables_blocks(&res2.1);
    if !res1.0 || !res2.0 || !res3.0 { return (false, new_board_probables); }
    while res3.1 != new_board_probables {
        new_board_probables = res3.1;
        res1 = fillin_probables_row(&new_board_probables);
        res2 = fillin_probables_col(&res1.1);
        res3 = fillin_probables_blocks(&res2.1);
        if !res1.0 || !res2.0 || !res3.0 { return (false, new_board_probables); }
    }
    (true, new_board_probables)
}

fn solve_stage4(board_probables : &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut new_board_probables : Vec<Vec<i32>> = (*board_probables).clone();
    let mut res1 = solve_stage3(&new_board_probables);
    let mut res2 = fillin_probables_cell(&res1.1);
    let mut res3 = consider_revervation(&res2.1);
    if !res1.0 || !res2.0 || !res3.0 { return (false, new_board_probables); }
    while res3.1 != new_board_probables {
        new_board_probables = res3.1;
        res1 = fillin_probables_row(&new_board_probables);
        res2 = fillin_probables_cell(&res1.1);
        res3 = consider_revervation(&res2.1);
        if !res1.0 || !res2.0 || !res3.0 { return (false, new_board_probables); }
    }
    (true, new_board_probables)
}


fn solve_forcefully_dfs_countup(board_probables : &Vec<Vec<i32>>, mut startind : i32) -> (bool, Vec<Vec<i32>>) {
    let res = solve_stage4(&board_probables);
    if !res.0 { return res; }

    while startind < BOARDSIZE as i32 && res.1[startind as usize].len() <= 1 { startind += 1; }

    if startind == BOARDSIZE as i32 {
        return (complete_board(&res.1) && follow_rules_board(&res.1), res.1);
    }

    for num in &res.1[startind as usize] {
        if !follow_rules_cell(&res.1, startind/UNIT, startind%UNIT, *num) {
            continue;
        }
        let mut new_board_probables = res.1.clone();
        new_board_probables[startind as usize] = vec![*num];
        let recursive_res = solve_forcefully_dfs_countup(&new_board_probables, startind+1);
        if recursive_res.0 {
            return recursive_res;
        }
    }
    (false, res.1)
}

fn solve_forcefully_dfs_countdown(board_probables : &Vec<Vec<i32>>, mut startind : i32) -> (bool, Vec<Vec<i32>>) {
    let mut res = solve_stage4(&board_probables);
    if !res.0 { return res; }

    while startind >= 0 && res.1[startind as usize].len() <= 1 { startind -= 1; }

    if startind == -1 {
        return (complete_board(&res.1) && follow_rules_board(&res.1), res.1);
    }

    res.1[startind as usize].reverse();
    for num in &res.1[startind as usize] {
        if !follow_rules_cell(&res.1, startind/UNIT, startind%UNIT, *num) {
            continue;
        }
        let mut new_board_probables = res.1.clone();
        new_board_probables[startind as usize] = vec![*num];
        let recursive_res = solve_forcefully_dfs_countdown(&new_board_probables, startind-1);
        if recursive_res.0 {
            return recursive_res;
        }
    }
    (false, res.1)
}

// (can solve, unique solve, solution)
fn solve_forcefully_dfs(board_probables : &Vec<Vec<i32>>) -> (bool, bool, Vec<Vec<i32>>) {
    let res_up = solve_forcefully_dfs_countup(board_probables, 0);
    let res_down = solve_forcefully_dfs_countdown(board_probables, (BOARDSIZE-1) as i32);
    (res_up.0 && res_down.0, res_up.1 == res_down.1, res_up.1)
}


// level
// 1 : 枠が多い
// 2 : ブロック埋めのみで解ける
// 3 : ブロック埋め+行列埋めの組み合わせで解ける
// 4 : 予約,セル埋めが必要
// 5 : シミュレーションが必要
//
// return
// (解ける, 唯一解である, 解)
pub fn solve(board : [i32; BOARDSIZE], level : i32) -> (bool, bool, [i32; BOARDSIZE]) {
    let board_probables = board_to_probables(board);
    match level {
        1 => {
            let blank_limit = 27;
            let res = solve_stage2(&board_probables);
            return (complete_board(&res.1) && follow_rules_board(&res.1) && (count_blank(&board_probables) < blank_limit), true, probables_to_board(res.1));
        }
        2 => {
            let res = solve_stage2(&board_probables);
            return (complete_board(&res.1) && follow_rules_board(&res.1), true, probables_to_board(res.1));
        }
        3 => {
            let res = solve_stage3(&board_probables);
            return (complete_board(&res.1) && follow_rules_board(&res.1), true, probables_to_board(res.1));
        }
        4 => {
            let res = solve_stage4(&board_probables);
            return (complete_board(&res.1) && follow_rules_board(&res.1), true, probables_to_board(res.1));
        }
        _ => {
            let res = solve_forcefully_dfs(&board_probables);
            return (res.0, res.1, probables_to_board(res.2));
        }
    }
}
