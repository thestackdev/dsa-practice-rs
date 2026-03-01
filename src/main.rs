#![allow(dead_code, unused_imports, unused_variables)]
mod hashmap;
mod two_pointers;

// use two_pointers::Solution;
// use two_pointers::RemoveDuplicatesFromSortedArray;
// use two_pointers::NextPermutation;
// use two_pointers::RemoveElement;
// use two_pointers::ReverseString;
// use two_pointers::TwoSumTwo;
// use two_pointers::ThreeSum;
use two_pointers::NextPermutation;

// use hashmap::TwoSum;
// use hashmap::FourSum;
// use hashmap::FourSumTwo;
// use hashmap::GroupAnagrams;
// use hashmap::FindAllDuplicatesInAnArray;

fn main() {
    let mut nums = vec![1, 2, 4, 3];
    NextPermutation::next_permutation(&mut nums);

    println!("{nums:?}");
}
