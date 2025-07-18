// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use core::{iter::{IntoIterator, Iterator}, option::Option::None};
use std::f64::consts::E;

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
  match chars.next() {
        Some(first) =>  first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new()
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    let mut vect_string:Vec<String> = Vec::new();

   for &word in words {
       let word = capitalize_first(word);
       vect_string.push(word);
   }

   vect_string
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    let mut emp_str = String::new();
    for word in words {
        let c = capitalize_first(&word);
        emp_str.push_str(&c);
    }

    emp_str
}

fn main() {
    // You can optionally experiment here.

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
