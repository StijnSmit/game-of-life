use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct CellState {
    pub is_alive: bool,
}

impl CellState {
    pub fn toggle(&mut self) {
        self.is_alive = !self.is_alive;
    }
}
