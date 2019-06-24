use std::{
    cmp::Ordering,
    ops::{Add, Sub},
};

/// Iterator returning subsets that sum to a given target.
pub struct SubsetEnumerator<'a, T> {
    set: &'a [T],
    target: T,
    selected: Vec<usize>,
    selected_sum: T,
    index: usize,
}

impl<'a, T> SubsetEnumerator<'a, T>
where
    T: Default,
{
    pub fn new(set: &'a [T], target: T) -> Self {
        SubsetEnumerator {
            set,
            target,
            selected: Vec::new(),
            selected_sum: T::default(),
            index: 0,
        }
    }
}

#[derive(PartialEq)]
enum CombinationResult {
    Found(Vec<usize>),
    Skip,
    Exhausted,
}

impl<'a, T> SubsetEnumerator<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy,
{
    fn next_combination(&mut self) -> CombinationResult {
        if self.index < self.set.len() {
            let s = self.selected_sum + self.set[self.index];
            match s.partial_cmp(&self.target) {
                Some(Ordering::Equal) => {
                    let mut subset = self.selected.clone();
                    subset.push(self.index);
                    self.index += 1;
                    CombinationResult::Found(subset)
                }
                Some(Ordering::Less) => {
                    self.selected.push(self.index);
                    self.selected_sum = s;
                    self.index += 1;
                    CombinationResult::Skip
                }
                Some(Ordering::Greater) | None => {
                    self.index += 1;
                    CombinationResult::Skip
                }
            }
        } else {
            match self.selected.pop() {
                None => CombinationResult::Exhausted,
                Some(i) => {
                    self.selected_sum = self.selected_sum - self.set[i];
                    self.index = i + 1;
                    CombinationResult::Skip
                }
            }
        }
    }
}

impl<'a, T> Iterator for SubsetEnumerator<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy,
{
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        let mut x = self.next_combination();
        while x == CombinationResult::Skip {
            x = self.next_combination();
        }
        match x {
            CombinationResult::Exhausted => None,
            CombinationResult::Found(subset) => Some(subset),
            CombinationResult::Skip => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let set = [2, 3, 2, 5, 3];
        let mut enumerator = SubsetEnumerator::new(&set, 8);
        assert_eq!(enumerator.next().unwrap(), [0, 1, 4]);
        assert_eq!(enumerator.next().unwrap(), [1, 2, 4]);
        assert_eq!(enumerator.next().unwrap(), [1, 3]);
        assert_eq!(enumerator.next().unwrap(), [3, 4]);
        assert!(enumerator.next().is_none());
    }
}
