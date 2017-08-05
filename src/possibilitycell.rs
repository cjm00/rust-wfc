use bit_vec::BitVec;
use rand;
use rand::distributions::{Range, IndependentSample};

use patch::Patch2D;

#[derive(Eq, PartialEq, Clone)]
pub struct PossibilityCell {
    pub possible_states: BitVec,
}

impl PossibilityCell {
    pub fn new(num_states: usize) -> Self {
        let possible_states = BitVec::from_elem(num_states, true);
        PossibilityCell { possible_states }
    }

    pub fn entropy(&self, state_counts: &[u32]) -> Option<f64> {
        if self.possible_states.none() {
            return None;
        };
        if self.possible_states.iter().filter(|p| *p).count() == 1 {
            return Some(0.);
        };

        let possible_state_count: u32 = state_counts
            .iter()
            .zip(self.possible_states.iter())
            .filter(|&(_, p)| p)
            .map(|(count, _)| *count)
            .sum();

        let possible_state_count = possible_state_count as f64;
        let entropy: f64 = state_counts
            .iter()
            .zip(self.possible_states.iter())
            .filter(|&(_, p)| p)
            .map(|(count, _)| {
                let x = *count as f64 / possible_state_count;
                x * x.ln()
            })
            .sum();

        Some(-entropy)
    }

    /// Marks all but a single state of the BitVec as forbidden, randomly chosen
    /// from the states still permitted and weighted by their frequency in the original image.
    pub fn collapse(&mut self, state_counts: &[u32]) {
        let chosen_state = PossibilityCell::masked_weighted_choice(state_counts, &self.possible_states).unwrap();
        self.possible_states.clear();
        self.possible_states.set(chosen_state, true);
    }

    /// Returns true if any states are permitted.
    pub fn consistent(&self) -> bool {
        self.possible_states.any()
    }

    pub fn decided(&self) -> bool {
        self.possible_states.iter().filter(|p| *p).count() == 1
    }

    pub fn to_output_state<T: Patch2D>(&self, states: &[T]) -> T::Output {
        if self.decided() {
            let chosen_state_index = self.possible_states.iter().enumerate().filter(|&(_, p)| p).map(|(i, _)| i).next().unwrap();
            states[chosen_state_index].output_state()
        } else {
            unimplemented!();
        }


    }

    /// Returns an index chosen from a slice of [u32] where each entry in the slice
    /// is the relative weight of that index, not considering any entries which are
    /// false in the argument iterator.
    fn masked_weighted_choice<M>(state_counts: &[u32], mask: &M) -> Option<usize>
    where
        for<'a> &'a M: IntoIterator<Item = bool>,
    {
        let masked_total: u32 = state_counts
            .iter()
            .cloned()
            .zip(mask.into_iter())
            .filter(|&(_, m)| m)
            .map(|(u, _)| u)
            .sum();
        let between = Range::new(0, masked_total);
        let mut rng = rand::thread_rng();
        let mut choice: u32 = between.ind_sample(&mut rng);

        for ((index, u), mask) in
            state_counts.iter().cloned().enumerate().zip(
                mask.into_iter(),
            )
        {
            if mask {
                if choice < u {
                    return Some(index as usize);
                }
                choice = choice.saturating_sub(u);
            }
        }

        None
    }
}
