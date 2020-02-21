use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use std::cmp::{max, min};
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor
}

#[derive(Component)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
#[storage(VecStorage)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct LeftMover;

struct LeftWalker;

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>,
                        WriteStorage<'a, Position>);
    
    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderable = self.ecs.read_storage::<Renderable>();
        
        for (pos, rend) in (&positions, &renderable).join() {
            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&players, &mut positions).join() {
        if map[xy_idx(pos.x + delta_x, pos.y + delta_y)] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, context: &mut Rltk) {
    match context.key {
        None => { },
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    map
}

fn draw_map(map: &[TileType], context: &mut Rltk) {
    for i in 0..map.len() {
        let (x, y) = idx_xy(i);

        match map[i] {
            TileType::Floor => context.set(x, y, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), rltk::to_cp437('.')),
            TileType::Wall => context.set(x, y, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), rltk::to_cp437('#')),
            _ => {}
        }
    }
}

fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50().with_title("Hello, Rust").build();

    let mut state = State {
        ecs: World::new()
    };
    state.ecs.register::<Position>();
    state.ecs.register::<Renderable>();
    state.ecs.register::<LeftMover>();
    state.ecs.register::<Player>();

    state.ecs.insert(new_map());

    state.ecs
        .create_entity()
        .with(Position { x: 20, y: 30})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK)
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        state.ecs
            .create_entity()
            .with(Position { x: i * 2, y: i * 2})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK)
            })
            .with(LeftMover {})
            .build();
    }

    rltk::main_loop(context, state);
}
