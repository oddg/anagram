/// https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub struct SwapIterator {
    n: usize,
    c: Vec<usize>,
    i: usize,
}

impl SwapIterator {
    pub fn new(n: usize) -> Self {
        SwapIterator {
            n,
            c: vec![0; n],
            i: 0,
        }
    }
}

impl Iterator for SwapIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.n {
            if self.c[self.i] < self.i {
                let item = if (self.i % 2) == 0 {
                    (0, self.i)
                } else {
                    (self.c[self.i], self.i)
                };
                self.c[self.i] += 1;
                self.i = 0;
                Some(item)
            } else {
                self.c[self.i] = 0;
                self.i += 1;
                self.next()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn factorial(n: usize) -> usize {
        let mut f = n;
        for i in 2..n {
            f *= i;
        }
        f
    }

    // Check the following properties:
    // - n! - 1 swaps are generated
    // - applying those swaps yields a sequence of uniq permutations
    fn test_helper(n: usize) {
        let mut v = (0..n).collect::<Vec<usize>>();
        let mut permutations = Vec::new();
        permutations.push(v.clone());
        for (i, j) in SwapIterator::new(n) {
            let t = v[i];
            v[i] = v[j];
            v[j] = t;
            permutations.push(v.clone());
        }

        assert_eq!(permutations.len(), factorial(n));
        permutations.sort_by(|x, y| {
            for (a, b) in x.iter().zip(y.iter()) {
                if a < b {
                    return Ordering::Less;
                } else if a > b {
                    return Ordering::Greater;
                }
            }
            assert!(false, "two equal permutations");
            panic!()
        });
    }

    #[test]
    fn check_length_and_uniqueness() {
        test_helper(4);
        test_helper(5);
        test_helper(6);
        test_helper(7);
        test_helper(8);
    }

    #[test]
    fn permutation_4() {
        let expected = vec![
            (0, 1),
            (0, 2),
            (0, 1),
            (0, 2),
            (0, 1),
            (0, 3),
            (0, 1),
            (0, 2),
            (0, 1),
            (0, 2),
            (0, 1),
            (1, 3),
            (0, 1),
            (0, 2),
            (0, 1),
            (0, 2),
            (0, 1),
            (2, 3),
            (0, 1),
            (0, 2),
            (0, 1),
            (0, 2),
            (0, 1),
        ];
        let generated = SwapIterator::new(4).collect::<Vec<(usize, usize)>>();
        assert_eq!(expected, generated);
    }
}
