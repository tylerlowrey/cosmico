mod app;
mod renderer;
mod core;

fn main() {
    pollster::block_on(app::run());
}
