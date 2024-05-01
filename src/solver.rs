use crate::app::Storage;

pub trait Solver {
    fn setup(&self, inventory: &mut Storage);
    fn step(&self, capacity: usize, inventory: &mut Storage, sack: &mut Storage);
}

pub struct Greedy;

impl Solver for Greedy {
    fn setup(&self, inventory: &mut Storage) {
        inventory.items.sort_by(|a, b| {
            let arate = a.weight / a.size as f64;
            let brate = b.weight / b.size as f64;
            brate.partial_cmp(&arate).unwrap()
        });
    }

    fn step(&self, capacity: usize, inventory: &mut Storage, sack: &mut Storage) {
        if let Some(item) = inventory.items.pop() {
            if sack.total() + item.size >= capacity {
                inventory.items.insert(0, item);
                return;
            }
            sack.items.push(item);
        }
    }
}
