use app::App;

mod app;
mod bullet;
mod entity;
mod player;
mod texture;
mod enemy;
fn main() {
    let mut app = App::new("shoot").unwrap();
    app.run().ok();
}
