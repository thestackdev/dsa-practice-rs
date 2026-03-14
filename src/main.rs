#![allow(dead_code, unused_imports, unused_variables)]
mod algorithms;
mod binary_search;
mod hashmap;
mod heaps;
mod intervals;
mod linked_list;
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
// use stacks::NextGreaterElement1;
// use stacks::NextGreaterElement2;
// use stacks::BasicCalculator2;

// use linked_list::{ListNode, RemoveNthNodeFromEndOfList};

// use algorithms::LinkedList;
// use algorithms::MinHeap;
// use algorithms::MaxHeap;

// use heaps::K_MOST_FREQ_ELEMENTS;
// use heaps::TOP_K_FREQ_ELEMENTS;

// use intervals::MergeIntervals;
// use intervals::{MeetingRooms, MeetingRoomsInterval};
// use intervals::IntervalListIntersection;
// use intervals::LargestOverlapOfIntervals;
use intervals::InsertInterval;

fn main() {
    let intervals = vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
    let new_interval = vec![4, 8];

    println!("{:?}", InsertInterval::insert(intervals, new_interval));
}
