use crate::constants::SCREEN_WIDTH;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH as usize) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % SCREEN_WIDTH, idx as i32 / SCREEN_WIDTH)
}