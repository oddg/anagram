use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, Sub},
};

/// Representation of an equivalence class of the anagram relationship
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct EquivalenceClass {
    /// count of the letters
    occurences: [u8; 12],
}

impl EquivalenceClass {
    pub fn new(s: &str) -> Option<Self> {
        let mut a = [0; 12];
        for c in s.chars() {
            match EquivalenceClass::index(c) {
                None => return None,
                Some(i) => a[i] += 1,
            }
        }

        Some(EquivalenceClass { occurences: a })
    }

    fn index(c: char) -> Option<usize> {
        match c {
            'a' => Some(0),
            'i' => Some(1),
            'l' => Some(2),
            'n' => Some(3),
            'o' => Some(4),
            'p' => Some(5),
            'r' => Some(6),
            's' => Some(7),
            't' => Some(8),
            'u' => Some(9),
            'w' => Some(10),
            'y' => Some(11),
            _ => None,
        }
    }
}

impl PartialOrd for EquivalenceClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self
            .occurences
            .iter()
            .zip(other.occurences.iter())
            .all(|(x, y)| x <= y)
        {
            return Some(Ordering::Less);
        }
        if self
            .occurences
            .iter()
            .zip(other.occurences.iter())
            .all(|(x, y)| x >= y)
        {
            return Some(Ordering::Greater);
        }

        return None;
    }
}

impl Add for EquivalenceClass {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut occurences = [0; 12];
        for (i, c) in occurences.iter_mut().enumerate() {
            *c = self.occurences[i] + other.occurences[i];
        }
        EquivalenceClass { occurences }
    }
}

impl Sub for EquivalenceClass {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut occurences = [0; 12];
        for (i, c) in occurences.iter_mut().enumerate() {
            *c = self.occurences[i] - other.occurences[i];
        }
        EquivalenceClass { occurences }
    }
}

impl Default for EquivalenceClass {
    fn default() -> Self {
        EquivalenceClass {
            occurences: [0; 12],
        }
    }
}

impl Sum for EquivalenceClass {
    fn sum<I: Iterator<Item = EquivalenceClass>>(iter: I) -> Self {
        iter.fold(EquivalenceClass::default(), |acc, c| acc + c)
    }
}
