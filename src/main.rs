use bracket_terminal::prelude::{main_loop, BError, BTermBuilder, GameState};

struct State;

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_terminal::prelude::BTerm) {
        ctx.print_centered(25, "Hello, world!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Snake")
        .build()?;
    let gs = State;
    main_loop(context, gs)
}
