#![allow(dead_code, unused_imports, unused_variables)]
mod algorithms;
mod binary_search;
mod hashmap;
mod heaps;
mod intervals;
mod linked_list;
mod prefix_sums;
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
// use sliding_window::MinimumSizeSubArray;

// use binary_search::SearchIntersectionPosition;
// use binary_search::FindFirstAndLastPositionOfElementInSortedArray;
// use binary_search::SearchInRotatedSortedArray;
// use binary_search::Sqrt;

// use stacks::NextGreaterElement;
// use stacks::NextGreaterElement1;
// use stacks::NextGreaterElement2;
// use stacks::BasicCalculator2;

// use linked_list::{ListNode, RemoveNthNodeFromEndOfList};

// use algorithms::LinkedList;
// use algorithms::MinHeap;
// use algorithms::MaxHeap;
// use algorithms::DoublyLinkedList;
use algorithms::Tree;

// use heaps::K_MOST_FREQ_ELEMENTS;
// use heaps::TOP_K_FREQ_ELEMENTS;

// use intervals::MergeIntervals;
// use intervals::{MeetingRooms, MeetingRoomsInterval};
// use intervals::IntervalListIntersection;
// use intervals::LargestOverlapOfIntervals;
// use intervals::InsertInterval;
// use intervals::NonOverlappingIntervals;

// use prefix_sums::RangeSumQueryImmutable;
// use prefix_sums::SubArraySumEqualsK;

fn main() {
    let mut t = Tree::new();
    for v in [5, 3, 7, 1, 4] {
        t.insert(v);
    }

    println!("In Order");
    t.in_order();
    println!("Pre Order");
    t.pre_order();
    println!("Post Order");
    t.post_order();

    println!("Inorder after invert tree");
    t.invert_using_stack();
    t.in_order();
}
