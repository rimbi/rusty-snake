use std::{
    cmp::min,
    time::{Duration, Instant},
};

use bracket_terminal::prelude::*;
use rand::Rng;

const TERMINAL_WIDTH: u8 = 80;
const TERMINAL_HEIGHT: u8 = 50;

/// A position of (x, y) on the terminal
#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    /// Creates a random position
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..TERMINAL_WIDTH),
            y: rng.gen_range(0..TERMINAL_HEIGHT),
        }
    }
}

/// Part of a snake.
///
/// Snake consists of multiple parts each of which has a position
/// on the screen and a direction to move.
#[derive(Debug)]
struct Part {
    pos: Position,
    direction: Option<VirtualKeyCode>,
}

/// State of the game
#[derive(Debug)]
struct State {
    pos: Vec<Part>,
    food_pos: Position,
    last_move_time: Instant,
    game_over: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_terminal::prelude::BTerm) {
        // If the game is over print the message and handle the reset/quit keys
        if self.game_over {
            ctx.print_centered(TERMINAL_HEIGHT / 2 - 1, "GAME OVER");
            ctx.print_centered(TERMINAL_HEIGHT / 2 + 1, "Press Space to restart");
            match ctx.key {
                Some(VirtualKeyCode::Space) => self.reset(),
                Some(VirtualKeyCode::Q) => ctx.quit(),
                _ => {}
            }
            return;
        }
        self.handle_keys(ctx);
        // Make the snake move each 100ms, no less than that
        let now = Instant::now();
        if now.duration_since(self.last_move_time) < Duration::from_millis(100) {
            return;
        }
        self.last_move_time = now;

        // Print the score
        ctx.print_centered(0, format!("Score: {}", self.pos.len()));

        // Print the food
        ctx.print(self.food_pos.x, self.food_pos.y, "*");

        // Draw the snake
        self.draw_snake(ctx);

        // Check if the snake eats the food
        if self.food_pos == self.pos.first().unwrap().pos {
            self.food_pos = Position::random();
            let last = self.pos.last().unwrap();
            let mut pos = last.pos;
            match last.direction {
                Some(VirtualKeyCode::Up) => pos.y = min(pos.y + 1, TERMINAL_HEIGHT - 1),
                Some(VirtualKeyCode::Down) => pos.y = pos.y.saturating_sub(1),
                Some(VirtualKeyCode::Left) => pos.x = min(pos.x + 1, TERMINAL_WIDTH - 1),
                Some(VirtualKeyCode::Right) => pos.x = pos.x.saturating_sub(1),
                _ => {}
            }
            let part = Part {
                pos,
                direction: last.direction,
            };
            self.pos.push(part);
        }
    }
}

impl State {
    /// Handles and reacts to the key strokes by the player.
    fn handle_keys(&mut self, ctx: &mut BTerm) {
        let part_count = self.pos.len();
        let direction = &mut self.pos.first_mut().unwrap().direction;
        match ctx.key {
            d @ Some(
                VirtualKeyCode::Up
                | VirtualKeyCode::Down
                | VirtualKeyCode::Left
                | VirtualKeyCode::Right,
            ) if part_count == 1 => *direction = d,
            d @ Some(VirtualKeyCode::Up) => {
                if !matches!(direction, Some(VirtualKeyCode::Down)) {
                    *direction = d;
                }
            }
            d @ Some(VirtualKeyCode::Down) => {
                if !matches!(direction, Some(VirtualKeyCode::Up)) {
                    *direction = d;
                }
            }
            d @ Some(VirtualKeyCode::Left) => {
                if !matches!(direction, Some(VirtualKeyCode::Right)) {
                    *direction = d;
                }
            }
            d @ Some(VirtualKeyCode::Right) => {
                if !matches!(direction, Some(VirtualKeyCode::Left)) {
                    *direction = d;
                }
            }
            Some(VirtualKeyCode::Q) => ctx.quit(),
            _ => {}
        }
    }

    /// Draws the snake on the screen
    fn draw_snake(&mut self, ctx: &mut BTerm) {
        for Part { pos, direction } in &mut self.pos {
            ctx.print(pos.x, pos.y, format!(" "));
            match direction {
                Some(VirtualKeyCode::Up) => pos.y = pos.y.saturating_sub(1),
                Some(VirtualKeyCode::Down) => pos.y = min(pos.y + 1, TERMINAL_HEIGHT - 1),
                Some(VirtualKeyCode::Left) => pos.x = pos.x.saturating_sub(1),
                Some(VirtualKeyCode::Right) => pos.x = min(pos.x + 1, TERMINAL_WIDTH - 1),
                _ => {}
            }
            ctx.print(pos.x, pos.y, "#");
        }
        // If the head touches to any of his parts, then the game is over
        let first_pos = self.pos.first().unwrap().pos;
        if self.pos.iter().skip(1).any(|part| first_pos == part.pos) {
            self.game_over = true;
            return;
        }
        // Update the directions
        let mut i = self.pos.len() - 1;
        while i > 0 {
            self.pos[i].direction = self.pos[i - 1].direction;
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    pub fn new() -> Self {
        Self {
            pos: vec![Part {
                pos: Position { x: 30, y: 25 },
                direction: None,
            }],
            food_pos: Position::random(),
            last_move_time: Instant::now(),
            game_over: false,
        }
    }

    /// Resets the game state
    ///
    /// This can be used to re/start the game
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Snake")
        .build()?;
    let gs = State::new();
    main_loop(context, gs)
}
