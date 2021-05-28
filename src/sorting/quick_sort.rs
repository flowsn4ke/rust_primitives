use rand::{thread_rng, Rng};

pub fn quick_sort<T>(input: &mut [T])
where
    T: PartialOrd,
{
    let length = input.len();
    if length < 2 {
        return;
    }
    let mut rng = thread_rng();

    let pivot = rng.gen_range(0..length);

    input.swap(0, pivot);

    let mut pivot = 1;

    for i in 1..input.len() {
        if input[i] < input[0] {
            input.swap(i, pivot);
            pivot += 1;
        }
    }

    input.swap(0, pivot - 1);

    quick_sort(&mut input[0..(pivot - 1)]);
    quick_sort(&mut input[pivot..length]);
}

#[cfg(test)]
mod sorting {
    use super::*;
    #[test]
    fn random_works_with_unsorted_vector() {
        let mut vec = vec![3, 4, 5, 6, 1, 9, 2, 7, 8];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn random_works_with_inverted_vector() {
        let mut vec = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn random_works_with_sorted_vector() {
        let mut vec = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }
    #[test]
    fn random_works_with_empty_vector() {
        let mut vec: Vec<i32> = vec![];
        quick_sort(&mut vec);
        assert_eq!(vec, [])
    }
    #[test]
    fn random_works_with_array() {
        let mut arr: [u32; 9] = [3, 4, 5, 6, 1, 9, 2, 7, 8];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[cfg(test)]
mod benching_quick_sort {
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
            numbers.push(rng.gen_range(0..10000));
        }

        b.iter(|| {
            quick_sort(&mut numbers);
        })
    }
}
