pub fn merge_sort<T>(input: &mut [T])
where
    T: PartialOrd + Copy,
{
    let length = input.len();
    // Our base case
    if length < 2 {
        return;
    }
    // We split the input into two more slices
    let (left, right) = input.split_at_mut(length / 2);
    // We recursively sort
    merge_sort(left);
    merge_sort(right);

    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    // We create a vector to temporarily hold the sorted values
    let mut sorted: Vec<T> = Vec::with_capacity(length);
    // We push values in order
    while let (Some(&x), Some(&y)) = (left_iter.peek(), right_iter.peek()) {
        if *x < *y {
            sorted.push(*left_iter.next().unwrap())
        } else {
            sorted.push(*right_iter.next().unwrap())
        }
    }
    // We push whatever is left
    for x in left_iter {
        sorted.push(*x)
    }
    for y in right_iter {
        sorted.push(*y)
    }
    // We rewrite the vector in place
    input
        .iter_mut()
        .enumerate()
        .for_each(|(i, val)| *val = sorted[i]);
}

#[cfg(test)]
mod sorting {
    use super::*;

    #[test]
    fn works_with_unsorted_vector() {
        let mut vec = vec![3, 4, 5, 6, 1, 9, 2, 7, 8];
        merge_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn works_with_inverted_vector() {
        let mut vec = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn works_with_sorted_vector() {
        let mut vec = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        merge_sort(&mut vec);
        assert_eq!(vec, [1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }
    #[test]
    fn works_with_empty_vector() {
        let mut vec: Vec<i32> = vec![];
        merge_sort(&mut vec);
        assert_eq!(vec, [])
    }
    #[test]
    fn works_with_array() {
        let mut arr: [u32; 9] = [3, 4, 5, 6, 1, 9, 2, 7, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[cfg(test)]
mod benching {
    use rand::{thread_rng, Rng};
    use test::Bencher;
    extern crate test;
    use super::*;

    #[bench]
    fn sorting_i32_vec(b: &mut Bencher) {
        let length: usize = 10000;
        let mut rng = thread_rng();
        let mut numbers = Vec::<i32>::with_capacity(length);

        for _ in 0..length {
            numbers.push(rng.gen_range(0..100));
        }

        b.iter(|| {
            merge_sort(&mut numbers);
        })
    }
}
