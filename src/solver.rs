use crate::app::Storage;

pub trait Solver {
    fn setup(&self, inventory: &mut Storage);
    fn step(&self, capacity: usize, inventory: &mut Storage, sack: &mut Storage);
}

pub struct Transfer;

impl Solver for Transfer {
    fn setup(&self, _inventory: &mut Storage) {}

    fn step(
        &self,
        capacity: usize,
        inventory: &mut crate::app::Storage,
        sack: &mut crate::app::Storage,
    ) {
        if let Some(item) = inventory.items.pop() {
            if sack.total() + item.size >= capacity {
                inventory.items.push(item);
                return;
            }
            sack.items.push(item);
        }
    }
}
