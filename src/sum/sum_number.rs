use bevy::prelude::*;

#[derive(Default, Component, Debug)]
pub struct SumNumber {
    index: usize,
}

impl SumNumber {
    pub fn init(index: usize) -> Self {
        Self { index }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}
