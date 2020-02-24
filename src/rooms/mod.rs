mod geometry;

use rltk::{Console, Rltk, RGB};
use std::cmp::{max, min};

use geometry::Rect;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::utils::{idx_xy, xy_idx};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..= room.x2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], x: i32, y1: i32, y2: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];

    for x in 0..SCREEN_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..SCREEN_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
    }

    map
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    apply_room_to_map(&Rect::new(20, 15, 10, 15), &mut map);
    apply_horizontal_tunnel(&mut map, 30, 40, 15);

    map
}

pub fn draw_map(map: &[TileType], context: &mut Rltk) {
    for i in 0..map.len() {
        let (x, y) = idx_xy(i);

        match map[i] {
            TileType::Floor => context.set(x, y, RGB::named(rltk::GRAY20), RGB::named(rltk::BLACK), rltk::to_cp437('.')),
            TileType::Wall => context.set(x, y, RGB::named(rltk::GRAY50), RGB::named(rltk::BLACK), rltk::to_cp437('#')),
            _ => {}
        }
    }
}