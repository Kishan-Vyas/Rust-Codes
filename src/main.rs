fn main() {
    let str = String::from("nayan");
    println!("String is {}", str);
    // question 1
    if check_palindrome_string(&str) {
        println!("{} is palindrome", str)
    } else {
        println!("{} is not palindrome", str)
    }

    // question 2
    let arr = [1, 2, 3, 4, 4, 4, 5, 6, 7, 8];
    let target = 4;
    match find_index_of_num_in_sorted_array(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} is not found in the array", target),
    }

    // question 3
    let long_string = String::from("the string value is a");
    match shortest_word(&long_string) {
        Some(word) => println!("{} is shortest word", word),
        None => println!("No words found in the string"),
    }

    // question 4
    let num: u64 = 7;
    if is_prime(num) {
        println!("{} is prime", num)
    } else {
        println!("{} is prime", num)
    }

    // question 5
    let arr = [1, 2, 3, 4, 5, 6];
    println!("Median is {}", find_median(&arr));

    // question 6
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));

    // question 7
    // 7 Implement a function that returns the kth smallest element in a given array.
    let mut arr = [5, 3, 4, 1, 2];
    let k = 2;
    arr.sort();
    println!("{} th smallest num is {}", k, arr[k-1] );

    // question 8
    let mut root = TreeNode::new(3);
    let left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let right_left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let right_right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.left = left;
    root.right = right;
    root.right.as_mut().unwrap().borrow_mut().left = right_left;
    root.right.as_mut().unwrap().borrow_mut().right = right_right;

    let depth = max_depth(Some(Rc::new(RefCell::new(root))));
    println!("Max depth of the binary tree: {}", depth);


    // question 9 
    // Reverse a string in Rust
    let s = String::from("this is string");
    let reversed_s = s.chars().rev().collect::<String>();
    println!("Original: {}", s);
    println!("Reversed: {}", reversed_s);

    // question 10 
    // Check if a number is prime in Rust
    let num: u64 = 7;
    if is_prime(num) {
        println!("{} is prime", num)
    } else {
        println!("{} is prime", num)
    }

    //question 11
    let arr1 = vec![1, 3, 5, 7, 9];
    let arr2 = vec![2, 4, 6, 8, 10];
    
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);

    // question 12
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);

}

// 1. Implement a function that checks whether a given string is a palindrome or not.
fn check_palindrome_string(str: &String) -> bool {
    let reverse_str: String = str.chars().rev().collect();
    return *str == reverse_str;
}

// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn find_index_of_num_in_sorted_array(arr: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut result = None;

    while start <= end {
        let mid = start + (end - start) / 2;
        if arr[mid] == target {
            result = Some(mid);
            end = mid - 1;
        } else if arr[mid] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    result
}

// 3 Given a string of words, implement a function that returns the shortest word in the string.
fn shortest_word(s: &String) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

// 4 Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 2 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// 5 Given a sorted array of integers, implement a function that returns the median of the array.
fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 1 {
        arr[n / 2] as f64
    } else {
        (arr[n / 2] as f64 + arr[(n / 2) - 1] as f64) / 2.0
    }
}

// 6 Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();

    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}

// 8 Given a binary tree, implement a function that returns the maximum depth of the tree.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            let left_depth = max_depth(node.left.clone());
            let right_depth = max_depth(node.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// 11 Merge two sorted arrays in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

// 12 Find the maximum subarray sum in Rust
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];

    for &num in arr.iter().skip(1) {
        max_ending_here = max_ending_here.max(num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}
