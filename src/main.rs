use rand::Rng;

mod app;
mod solver;
mod tui;
mod ui;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new(Box::new(crate::solver::Transfer {}));
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
    for i in 1..4 {
        app.sack.items.push(app::Item {
            id: 1,
            size: 5 + i,
            weight: 3.0,
            selected: false,
        });
    }
    app.run(&mut terminal)?;
    tui::shutdown()?;
    Ok(())
}
