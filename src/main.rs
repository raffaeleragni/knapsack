use rand::Rng;

mod app;
mod solver;
mod tui;
mod ui;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new(Box::new(crate::solver::Greedy {}));
    app.capacity = 500;
    let mut rng = rand::thread_rng();
    for i in 1..30 {
        app.inventory.items.push(app::Item {
            id: 1,
            size: 10 + i,
            weight: round::round(rng.gen_range(1.0..20.0), 1),
            selected: (i & 1) == 1,
        });
    }
    app.run(&mut terminal)?;
    tui::shutdown()?;
    Ok(())
}
