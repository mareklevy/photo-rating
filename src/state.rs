use serde_derive::{Deserialize, Serialize};
use crate::item::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub total: f32,
    pub average: f32,
    pub items: Vec<Item>,
    pub buttons: Vec<i32>,
}

impl State {
    pub fn recount(&mut self) {
        let sum: f32 = self.items
            .iter()
            .map(|i| i.value)
            .collect::<Vec<_>>()
            .iter()
            .sum();

        match sum == 0.0 {
            true => {
                self.average = 0.0;
                self.total = 0.0;
            }
            _ => {
                let count = self.items.len() as f32;
                self.average = ((sum / count) * 100.0).round() / 100.0; 
        
                self.total = State::total_rounding(&self.average);
            }
        }        
    }

    pub fn remove(&mut self, idx: usize) {
        let idx = {
            let items = self
                .items
                .iter()
                .enumerate()
                .collect::<Vec<_>>();
            let &(idx, _) = items.get(idx).unwrap();
            idx
        };
        
        self.items.remove(idx);
    }

    pub fn reset(&mut self) {
        self.total = 0.0;
        self.average = 0.0;
        self.items.clear();
    }

    fn total_rounding(value: &f32) -> f32 {
        let round = (((value * 10.0)) / 10.0).round();

        round
    }

}
