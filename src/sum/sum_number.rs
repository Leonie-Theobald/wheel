use bevy::prelude::*;

#[derive(Default, Component, Debug)]
pub(crate) struct SumNumber {
    index: usize,
}

impl SumNumber {
    pub(crate) fn init(index: usize) -> Self {
        Self { index }
    }

    pub(crate) fn get_index(&self) -> usize {
        self.index
    }
}
