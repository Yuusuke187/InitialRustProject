use std::collections::HashMap;
use parameterized::parameterized;

// use core::num;
// use std::{string, ptr::null};
use itertools::Itertools;

fn get_sum_of_multiples(num: i32) -> i32 {
    if num < 3 {
        return 0;
    }

    let mut final_num: i32 = 0;
    for i in (3..num).rev() {
        if i % 5 == 0 || i % 3 == 0 {
            final_num +=i;
        }
    }

    return final_num;
}

fn alphabet_position(text: &str) -> String {
    // Code here...
    let mut num_string: String = String::new();

    // create an alphabet string, no, array
    let alphabet: &str = "_abcdefghijklmnopqrstuvwxyz";
    let _alphabet_array: Vec<char> = alphabet.chars().collect();

    // break the string text into an array
    let char_array: Vec<char> = text.chars().collect();

    // create an index position number
    let _index_position: i32;

    // iterate through each letter and retrieve the index positions 
    // in the alphabet
    for _letter in char_array.iter() {
        num_string.push_str(" ");
        // num_string.push_str(_index_position);
    }
    
    return num_string;

}

pub fn fizz_buzz(n:i32) -> Vec<String> {
    // define a string array
    let mut string_array: Vec<String> = Vec::new();

    for i in 1..n+1 {
        if i % 5 == 0 && i % 3 == 0 {
            string_array.push(String::from("FizzBuzz"));
        }
        else if i % 3 == 0  {
            string_array.push(String::from("Fizz"));
        }
        else if i % 5 == 0 {
            string_array.push(String::from("Buzz"));
        }
        else {
            string_array.push(String::from(i.to_string()));
        }
    }

    return string_array;
}

pub fn steps_to_reduce_num_to_zero(mut num:i32) -> i32 {
    // create a while loop for when num is greater than 0
    let mut _count: i32 = 0;
    while num > 0 {
        if num % 2 == 0 {
            num /= 2;
        }
        else {
            num -= 1;
        }   

        _count += 1;
    }
    
    return _count;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    // fn next(val: i32)
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Return the second middle node
    let mut last_node: Option<Box<ListNode>> = head;
    let mut second_middle_node: Option<Box<ListNode>> = None;
    // let mut optional_second_middle_node: Option<Box<ListNode>> = None;

    let mut optional_last_node: Box<ListNode> = last_node.take().unwrap();
    // let mut optional_second_middle_node: Box<ListNode> = second_middle_node.take().unwrap();
    
    // while last_node != None && last_node.next != None {    
    while last_node != None && optional_last_node.next != None {    
        let mut optional_last_node_next: Box<ListNode> = optional_last_node.next.take().unwrap();
        let optional_last_node_next_next:Box<ListNode> = optional_last_node_next.next.take().unwrap();
        
        optional_last_node = optional_last_node_next_next;
        second_middle_node = Some(optional_last_node_next);
    }

    return second_middle_node;
}

// pub fn find_middle_node_new(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut last_node: Option<Box<ListNode>> = head;
//     let mut second_middle_node: Option<Box<ListNode>> = head;

//     while let (Some(node), Some(mid_node)) = (&last_node, &second_middle_node) {
//         second_middle_node = &node.next;
//         last_node = &mid_node.as_ref()?.next;
//     }

//     second_middle_node.as_ref().map(|node: &Box<ListNode>| &node.val)
// }

fn sorted_sentence_example() {
    let wordy = "I am a hello world example";

    let s = wordy.chars().sorted().rev().collect::<String>();

    println!("{}", s);
}

pub fn can_construct_iter(ransom_note: String, magazine: String) -> bool {
    // check if ransom_note and magazine sorted are the same
    let sorted_ransom = ransom_note.chars().sorted().rev().collect::<String>();
    let sorted_mag = magazine.chars().sorted().rev().collect::<String>();

    // if magazine.contains(&ransom_note) {
    if sorted_ransom == sorted_mag {
        return true;
    }

    return false;
}

pub fn can_construct_other(ransom_note: String, magazine: String) -> bool {
     // put ransom_note and magazine in hash tables
     let mut ransom_hash: HashMap<char, i32> = HashMap::new();
     let mut mag_hash: HashMap<char, i32> = HashMap::new();
     // let (random_hash, mag_hash) = HashMap::new();

     for _letter in ransom_note.chars() {
        let counter: &mut i32 = ransom_hash.entry(_letter).or_insert(0);
        *counter += 1;
     }

     for _letter in magazine.chars() {
        let counter: &mut i32 = mag_hash.entry(_letter).or_insert(0);
        *counter += 1;
     }

     if magazine.contains(&ransom_note ) || ransom_hash == mag_hash {
        return true;
     }

     return false;
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut mag_letter_map: HashMap<char, i32> = HashMap::new();

    for i in 0..magazine.len() {
        let _letter: Option<char> = magazine.chars().nth(i);

        // let letter_count = mag_letter_map.get(_letter);
        // mag_letter_map.insert(_letter, letter_count + 1);
    }

    return false;
}

pub fn find_number_of_digits(num: i32) -> i32 {
    let float_value: f64 = num as f64;
    let log_num = f64::log10(float_value);
    let num: i32 = log_num as i32 + 1;
    return num;
}

pub fn find_number_of_digits_again(num: i32) -> usize {
    let num_digits: usize = num.to_string().len();
    return num_digits;
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut squares: Vec<i32> = nums.iter().map(|&x| x * x).collect();
    squares.sort();
    return squares;
}

fn main() {    
    // let num_steps: i32 = steps_to_reduce_num_to_zero(14);
    // println!("The number of steps to zero is {}", num_steps);
    
    // let digits = find_number_of_digits_again(123456);
    // println!("The number of digits in the number are {}", digits);

    let squares = sorted_squares([-4, -1, 0, 3, 10].to_vec());
    println!("{}", squares[0]);
    println!("{}", squares[1]);
    println!("{}", squares[2]);

    // println!("Hello, world!");
    // println!("The value of the final sum is {}", final_sum);

    // You know what, this is messy. Let's write tests.
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;
    // use parameterized::parameterized;
   use test_case::test_case;
    
   
    #[test]
    fn test_get_sum_of_multiples() {
        let final_sum = get_sum_of_multiples(50);
        assert_eq!(final_sum, 543);
    }

    // #[rstest(        // was parameterized
    //     value => 50 // was =
    // )]
    // fn test_get_sum_of_multiples_parameterized(value: i32) {
    //     let final_sum = get_sum_of_multiples(value);
    //     assert_eq!(final_sum, 543);
    // }

    #[parameterized(       
        value = { 50 }
    )]
    fn test_get_sum_of_multiples_parameterized(value: i32) {
        let final_sum = get_sum_of_multiples(value);
        assert_eq!(final_sum, 543);
    }

    #[test_case(50)]
    fn test_get_sum_of_multiples_test_case(value: i32) {
        let final_sum = get_sum_of_multiples(value);
        assert_eq!(final_sum, 54);
    }

    #[test]
    fn test_alphabet_position() {
        let alpha_num_string: String = alphabet_position("Something Something");
        // Come back to this. I can't quite get it to work
        assert_eq!(alpha_num_string, "1234567890");
    }

    #[test]
    fn test_fizz_buzz() {
        // 
        let fizz_buzz_string: Vec<String> = fizz_buzz(20);
        for element in &fizz_buzz_string {
            println!("{}", element);
            
        }
    }

    #[test_case( [5, 4, 3]   )]
    fn test_sorted_squares(nums: Vec<i32>) {
        let mut squares: Vec<i32> = nums.iter().map(|&x| x * x).collect();
        squares.sort();
        // return squares;
        
    }

    #[test]
    fn test_reduce_num_to_zero() {

    }
}