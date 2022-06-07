mod app;
mod renderer;
mod core;
mod game;

fn main() {
    pollster::block_on(app::run());
}
