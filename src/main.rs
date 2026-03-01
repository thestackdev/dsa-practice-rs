#![allow(dead_code, unused_imports, unused_variables)]
mod hashmap;
mod sliding_window;
mod two_pointers;

// use two_pointers::Solution;
// use two_pointers::RemoveDuplicatesFromSortedArray;
// use two_pointers::NextPermutation;
// use two_pointers::RemoveElement;
// use two_pointers::ReverseString;
// use two_pointers::TwoSumTwo;
// use two_pointers::ThreeSum;
// use two_pointers::NextPermutation;

// use hashmap::TwoSum;
// use hashmap::FourSum;
// use hashmap::FourSumTwo;
// use hashmap::GroupAnagrams;
// use hashmap::FindAllDuplicatesInAnArray;

// use sliding_window::AnagramsInAString;
// use sliding_window::LongestSubstringWithoutRepeatingCharacters;
// use sliding_window::LongestRepeatingCharacterReplacement;
use sliding_window::MaximumSumOfDistinctSubarraysWithLengthK;

fn main() {
    let nums = vec![1, 5, 4, 2, 9, 9, 9];

    let result = MaximumSumOfDistinctSubarraysWithLengthK::maximum_subarray_sum(nums, 3);

    println!("{result:?}");
}
