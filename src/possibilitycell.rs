use bit_vec::BitVec;

#[derive(Eq, PartialEq, Clone)]
pub struct PossibilityCell {
    possible_states: BitVec,
}

impl PossibilityCell {
    pub fn new(num_states: usize) -> Self {
        let possible_states = BitVec::from_elem(num_states, true);
        PossibilityCell { possible_states }
    }

    pub fn entropy<T>(&self, states: &[(T, usize)]) -> Option<f64> {
        unimplemented!()
    }
}
