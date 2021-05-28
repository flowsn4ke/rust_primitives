#![feature(test)]
#![feature(drain_filter)]
#![feature(destructuring_assignment)]

pub mod graphs;
pub mod permutations;
pub mod sorting;
pub use permutations::inversion_count;
pub use sorting::{merge_sort, quick_sort};
