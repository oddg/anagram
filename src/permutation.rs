/// https://en.wikipedia.org/wiki/Heap%27s_algorithm
struct Permutation {
    permutation: Vec<usize>,
    // length of the fixed prefix
    fixed: usize,
}

impl Iterator for Permutation {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if fixed == (permutation.len() - 1) {
            self.fixed -= 1;
            return Some(self.permutation.clone());
        }
        unimplemented!()
    }
}
