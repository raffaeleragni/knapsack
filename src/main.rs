use rand::Rng;

mod app;
mod solver;
mod tui;
mod ui;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new(Box::new(crate::solver::Greedy {}));
    app.capacity = 100;
    let mut rng = rand::thread_rng();
    for i in 1..50 {
        app.inventory.items.push(app::Item {
            id: i,
            size: rng.gen_range(1..20),
            weight: round::round(rng.gen_range(1.0..20.0), 1),
            selected: false,
        });
    }
    for i in 1..8 {
        app.inventory.items.push(app::Item {
            id: -i,
            size: 1,
            weight: 0.0,
            selected: false,
        });
    }
    app.setup();
    app.run(&mut terminal)?;
    tui::shutdown()?;
    Ok(())
}
