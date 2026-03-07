#![allow(dead_code, unused_imports, unused_variables)]
mod binary_search;
mod hashmap;
mod sliding_window;
mod stacks;
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
// use sliding_window::MaximumSumOfDistinctSubarraysWithLengthK;
// use sliding_window::PermutationInString;
// use sliding_window::FruitIntoBaskets;

// use binary_search::SearchIntersectionPosition;
// use binary_search::FindFirstAndLastPositionOfElementInSortedArray;
// use binary_search::SearchInRotatedSortedArray;

// use stacks::NextGreaterElement;
use stacks::NextGreaterElement1;

fn main() {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    let result = NextGreaterElement1::next_greater_element_optimised(nums1, nums2);

    println!("{:?}", result);
}
