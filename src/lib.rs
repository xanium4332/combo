#[cfg(test)]
use std::collections::BTreeSet;

/// An non-standard iterator yielding combinations of elements from a sequence.
pub struct Combinator<'a, T>
    where T: 'a
{
    seq: &'a [T],
    indices: Vec<usize>,
    inited: bool,
}

impl<'a, 'b, T> Combinator<'a, T> {
    pub fn next(&'b mut self) -> Option<CombinationIter<'a, 'b, T>> {
        let seq_len = self.seq.len();
        let ref mut indices = self.indices;
        let k = indices.len();

        // First permutation is special cased
        if !self.inited {
            self.inited = true;

            return Some(CombinationIter {
                it: indices.iter(),
                seq: self.seq,
            });
        }

        for i in (0..k).rev() {
            // Try and increment this index
            indices[i] += 1;

            if indices[i] == seq_len - k + 1 + i {
                // Index has overflowed, try parent index
                continue;
            }

            // Reset child indices
            for j in i + 1..k {
                indices[j] = indices[j - 1] + 1;
            }

            return Some(CombinationIter {
                it: indices.iter(),
                seq: self.seq,
            });
        }

        return None;
    }
}

/// An `Iterator` yielding references to elements of a particular combination.
pub struct CombinationIter<'a, 'b, T>
    where T: 'a
{
    it: std::slice::Iter<'b, usize>,
    seq: &'a [T],
}

impl<'a, 'b, T> Iterator for CombinationIter<'a, 'b, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.it.next() {
            Some(&self.seq[*i])
        } else {
            None
        }
    }
}

/// Returns a (non-standard) iterator yielding all combinations of length `len` from
/// the sequence `seq`.
///
/// Each combination is itself a (standard) `Iterator` yielding references to each element of the
/// combination.
///
/// # Panics
/// When attempting to iterate over combination lengths longer than the original sequence.
///
/// # Examples
///
/// ```
/// use combo::combinations;
///
/// let sequence: Vec<u32> = (0..5).collect();
/// let mut combinator = combinations(&sequence[..], 3);
///
/// let mut i = 0;
/// while let Some(combo) = combinator.next() {
/// 	let combination: Vec<&u32> = combo.collect();
///     println!("{}: {:?}", i, combination);
///     i += 1;
/// }
///
/// // Prints:
/// // 0: [0, 1, 2]
/// // 1: [0, 1, 3]
/// // 2: [0, 1, 4]
/// // 3: [0, 2, 3]
/// // 4: [0, 2, 4]
/// // 5: [0, 3, 4]
/// // 6: [1, 2, 3]
/// // 7: [1, 2, 4]
/// // 8: [1, 3, 4]
/// // 9: [2, 3, 4]
/// ```
///
pub fn combinations<'a, T>(seq: &'a [T], len: usize) -> Combinator<'a, T> {
    if len > seq.len() {
        panic!("Combination length longer than sequence ({} > {})",
               len,
               seq.len());
    }

    Combinator {
        seq: seq,
        indices: (0..len).collect(),
        inited: false,
    }
}

#[test]
fn all_combinations_generated() {
    // Original sequence
    let sequence: Vec<u32> = (0..4).collect();

    // Set of all combinations
    let combos: BTreeSet<BTreeSet<u32>> = [[0, 1, 2].iter().cloned().collect(),
                                           [0, 1, 3].iter().cloned().collect(),
                                           [0, 2, 3].iter().cloned().collect(),
                                           [1, 2, 3].iter().cloned().collect()]
                                              .iter()
                                              .cloned()
                                              .collect();

    let mut combinator = combinations(&sequence[..], 3);
    while let Some(combo) = combinator.next() {
        let c: BTreeSet<_> = combo.cloned().collect();
        assert!(combos.contains(&c),
                "{:?} does not contain {:?}",
                &combos,
                &c);
    }
}

#[test]
#[should_panic]
fn panics_on_invalid_combination_length() {
    let sequence: Vec<u32> = (0..4).collect();
    combinations(&sequence[..], sequence.len() + 1);
}
