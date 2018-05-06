extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{timer, Context, ContextBuilder, GameResult};
use std::{env, path};

/**
 * The board is assumed to be square, with dimensions set in the main function.
 *
 * The number of columns and rows is determined by the width of the window
 * divided by CELL_SIZE
 */

const CELL_SIZE: f32 = 1.0;
const WINDOW_SIZE: u32 = 1000;

struct MainState {
    curr: Vec<Vec<bool>>,
    prev: Vec<Vec<bool>>,
    limit: usize,
    spritebatch: graphics::spritebatch::SpriteBatch,
}

fn wrap(idx: i32, size: i32, amt: i32) -> usize {
    let mut res = idx + amt;
    if res < 0 {
        res = size + res;
    } else {
        res = res % size;
    }
    res as usize
}

fn is_alive(i: usize, j: usize, board: &Vec<Vec<bool>>) -> bool {
    let mut neighbors_alive = 0;
    for n in -1..1 + 1 {
        for m in -1..1 + 1 {
            if (n != 0 || m != 0)
                && board[wrap(i as i32, board.len() as i32, n as i32)]
                    [wrap(j as i32, board.len() as i32, m as i32)]
            {
                neighbors_alive += 1;
            }
        }
    }
    neighbors_alive == 3 || neighbors_alive == 2 && board[i][j]
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // columns/rows count set here
        let limit = ctx.conf.window_mode.width as usize / CELL_SIZE as usize;
        let mut board: Vec<_> = Vec::new();
        for i in 0..limit {
            board.push(vec![]);
            for _ in 0..limit {
                board[i].push(rand::random::<f64>() < 0.25);
            }
        }

        let image = graphics::Image::new(ctx, "/cell.png")?;
        let batch = graphics::spritebatch::SpriteBatch::new(image);

        let s = MainState {
            curr: board.clone(),
            prev: board,
            limit,
            spritebatch: batch,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for i in 0..self.limit {
            for j in 0..self.limit {
                self.prev[i][j] = is_alive(i, j, &self.curr);
            }
        }
        std::mem::swap(&mut self.curr, &mut self.prev);
        if timer::get_ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::get_delta(ctx));
            println!("Average FPS: {}", timer::get_fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        for i in 0..self.limit {
            for j in 0..self.limit {
                if self.curr[i][j] {
                    let p = graphics::DrawParam {
                        dest: graphics::Point2::new(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE),
                        ..Default::default()
                    };
                    self.spritebatch.add(p);
                }
            }
        }
        graphics::draw_ex(ctx, &self.spritebatch, Default::default())?;
        self.spritebatch.clear();
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let cb = ContextBuilder::new("game_of_life", "ggez")
        .window_setup(conf::WindowSetup::default().title("Game of Life"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_SIZE, WINDOW_SIZE));
    let ctx = &mut cb.build().unwrap();
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
