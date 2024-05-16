use app::App;

mod app;
mod entity;
fn main() {
    let mut app = App::new("shoot").unwrap();
    app.run().ok();
}
