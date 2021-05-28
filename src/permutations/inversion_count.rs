// To count inversions in O(n log n) time complexity we just piggyback on merge sort
pub fn inversion_count<T>(input: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let length = input.len();

    if length < 2 {
        return 0;
    }

    let (left, right) = input.split_at_mut(length / 2);

    let inversions_left = inversion_count(left);
    let inversions_right = inversion_count(right);
    // We start by adding up inversions coming from recursive call
    let mut inversions = inversions_left + inversions_right;

    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = 0;

    let mut sorted: Vec<T> = Vec::with_capacity(length);

    while left_pointer < left.len() && right_pointer < right.len() {
        if left[left_pointer] < right[right_pointer] {
            // If the left element is inserted first, no inversions are involved
            sorted.push(left[left_pointer]);
            left_pointer += 1;
        } else {
            sorted.push(right[right_pointer]);
            right_pointer += 1;
            // Here, the length of the left half minus the insertion
            // index gives us the number of invertions
            inversions += left.len() - left_pointer;
        }
    }

    for x in left_pointer..left.len() {
        sorted.push(left[x]);
    }
    for y in right_pointer..right.len() {
        sorted.push(right[y]);
    }

    input
        .iter_mut()
        .enumerate()
        .for_each(|(i, val)| *val = sorted[i]);

    inversions
}

#[cfg(test)]
mod counting_inversions {
    use super::*;

    #[test]
    fn works_with_unsorted_vector() {
        let mut vec = vec![1, 3, 5, 2, 4, 6];
        let inversions = inversion_count(&mut vec);
        assert_eq!(inversions, 3);
    }
    #[test]
    fn works_with_inverted_vector() {
        let mut vec = vec![5, 4, 3, 2, 1, 0];
        let inversions = inversion_count(&mut vec);
        assert_eq!(inversions, 15);
    }
    #[test]
    fn works_with_sorted_vector() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let inversions = inversion_count(&mut vec);
        assert_eq!(inversions, 0);
    }
    #[test]
    fn works_with_empty_vector() {
        let mut vec: Vec<i32> = vec![];
        let inversions = inversion_count(&mut vec);
        assert_eq!(inversions, 0);
    }
    #[test]
    fn works_with_array() {
        let mut arr: [u32; 6] = [1, 3, 5, 2, 4, 6];
        let inversions = inversion_count(&mut arr);
        assert_eq!(inversions, 3);
    }
}

#[cfg(test)]
mod benching_inversion_count {
    use rand::{thread_rng, Rng};
    use test::Bencher;
    extern crate test;
    use super::*;

    #[bench]
    fn counting_inversions_in_i32_vec(b: &mut Bencher) {
        let length: usize = 10000;
        let mut rng = thread_rng();
        let mut numbers = Vec::<i32>::with_capacity(length);

        for _ in 0..length {
            numbers.push(rng.gen_range(0..10000));
        }

        b.iter(|| {
            inversion_count(&mut numbers);
        })
    }
}
