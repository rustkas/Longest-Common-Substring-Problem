#![feature(type_ascription)]

use std::borrow::Cow;
use std::collections::HashMap;
use std::str::Chars;

/// Constructs longest string as a substring of input string consisting of
/// no more than two different characters.
///
/// * `string` input string,
///
///
/// # Examples
///
/// ```
///   let input_string = "ab";
//    let longest_string = longest_substring(input_string);
//    println!("The input string is '{}'", input_string);
//    println!("The longest string is '{}'. Length = {}", longest_string, longest_string.len());
/// ```
///
///

pub fn longest_substring(input: &str) -> Cow<str> {
    let length: usize = input.len();

    if length <= 2 {
        return input.into();
    }

    find_max_string(input)
}

///Searching max continued sequence of characters based on two characters
///
///Return max string
///
pub fn find_max_string(input: &str) -> Cow<str> {
    let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);
    let chars_info_len = chars_info.len();
    if chars_info_len < 2 {
        return input.into();
    }
    // println!("{:?}",chars_info);
    let mut keys_vector: Vec<char> = vec![];
    for char_item in chars_info.keys() {
        keys_vector.push(*char_item);
    }
    keys_vector.sort();

    let mut keys1 = keys_vector.iter();
    let mut key1: Option<&char> = keys1.next();

    let mut key_index = 0;

    let mut max_string: Cow<str> = "".into();
    let mut current_string: Cow<str>;
    while let Some(char_key_value1) = key1 {
        //  let key1_char: char = *char_key_value1;
        //   println!("{:?} ", key1_char);

        if chars_info_len == key_index + 1 {
            break;
        }
        let mut iterator2 = keys_vector.iter().skip(key_index + 1);
        let mut iterator2_key = iterator2.next();

        while let Some(char_key_value2) = iterator2_key {
            //     println!("{:?} || {:?}", key1_char, value2);
            current_string = create_max_string(*char_key_value1, *char_key_value2, &chars_info);

            if current_string.len() > max_string.len() {
                max_string = current_string;
            }

            iterator2_key = iterator2.next();
        }
        key1 = keys1.next();
        key_index += 1;
    }

    max_string
}

/// Makes result of analysis of the input string.
///
/// * `input` input string,
///
/// 'char' is a one character in the string.
/// 'vector' is a sequence of indexes of that character in the string.
///

pub fn make_string_info(input: &str) -> HashMap<char, Vec<usize>> {
    let chars: Chars = input.chars();
    let mut string_info: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, item) in chars.enumerate() {
        if string_info.contains_key(&item) {
            if let Some(inner_vector) = string_info.get_mut(&item) {
                inner_vector.push(i);
            }
        } else {
            let mut vec: Vec<usize> = Vec::new();
            vec.push(i);
            string_info.insert(item, vec);

            // println!("create new vector vector => {}", item)
        }
    }

    // optimize data containers
    string_info.shrink_to_fit();
    for (_, vector) in string_info.iter_mut() {
        vector.shrink_to_fit();
    }
    string_info
}

///Creates max string base on input data: keys of HashMap and a content
///
/// * `fist` the first combination element
/// * `second` the second combination element
/// * `chars_info` information about input string HashMap (key - char item, value - vector of indexes)
///
/// # Return
///
/// Returns the longest string of all possible choices.
///
///
pub fn create_max_string<'a>(
    fist: char,
    second: char,
    chars_info: &HashMap<char, Vec<usize>>,
) -> Cow<'a, str> {
    let indexes1: Option<&Vec<usize>> = chars_info.get(&fist);
    let indexes2: Option<&Vec<usize>> = chars_info.get(&second);

    let longest_sequence: Vec<usize> = merge_vectors(indexes1, indexes2);

    let mut longest_string: String = String::with_capacity(longest_sequence.len());

    let vector_indexes1: Vec<usize>;
    let vector_indexes2: Vec<usize>;
    if let Some(vector1) = indexes1 {
        vector_indexes1: Vec<usize> = vector1.clone();
    } else {
        //panic!("first vector. no data indexes");
        return longest_string.into();
    }

    if let Some(vector2) = indexes2 {
        vector_indexes2: Vec<usize> = vector2.clone();
    } else {
        //panic!("second vector. no data indexes");
        return longest_string.into();
    }

    for item in &longest_sequence {
        if let Some(_i) = vector_indexes1.iter().find(|&&x| {
            // println!("{} {}", x, *item);
            x == *item
        }) {
            // println!("{}", i);
            longest_string.push(fist);
        } else if let Some(_i) = vector_indexes2.iter().find(|&&x| x == *item) {
            //  println!("{}", i);
            longest_string.push(second);
        }
    }

    longest_string.into()
}

/// Util function.
/// Merging two vectors to one.
/// * `indexes1` the first vector
/// * `indexes2` the second vector
///
///  # Returns
///  Longest sequence of indexes of the merged vectors
///
pub fn merge_vectors(indexes1: Option<&Vec<usize>>, indexes2: Option<&Vec<usize>>) -> Vec<usize> {
    let mut collection: Vec<usize> = Vec::new();

    if let Some(vector1) = indexes1 {
        let mut vector_indexes1: Vec<usize> = vector1.to_vec().clone();

        collection.append(&mut vector_indexes1);
    } else {
        //panic!("first vector. no data indexes");
        return collection;
    }

    if let Some(vector2) = indexes2 {
        let mut vector_indexes2: Vec<usize> = vector2.to_vec().clone();
        collection.append(&mut vector_indexes2);
    } else {
        //panic!("second vector. no data indexes");
        return collection;
    }

    let longest_sequence: Vec<usize> = find_longest_sequence(&mut collection);
    longest_sequence
}

/// Find longest continued sequence of numbers.
/// IMPORTANT: This method sorts input vector.
///
/// # Examples
///
/// ```
///        let mut vector: Vec<usize> = vec![1, 2, 3, 4, 6, 8];
///        let expected: Vec<usize> = vec![1, 2, 3, 4];
///        let result: Vec<usize> = find_longest_sequence(&mut vector);
///        println!("{:?}", result);
/// ```
pub fn find_longest_sequence(vec: &mut Vec<usize>) -> Vec<usize> {
    vec.sort();

    let mut index = 0;

    let mut iterator1 = vec.iter();
    let mut key1 = iterator1.next();
    let mut vec_consumer: Vec<usize> = Vec::new();
    let mut max_consumer: Vec<usize> = Vec::new();
    let mut do_push: bool = false;
    while let Some(mut value1) = key1 {
        if !do_push {
            vec_consumer.push(*value1);
        }

        let mut iterator2 = vec.iter().skip(index + 1);
        let mut key2 = iterator2.next();
        if None == key2 {
            if max_consumer.len() < vec_consumer.len() {
                max_consumer = vec_consumer.clone();
            }
            break;
        }
        while let Some(value2) = key2 {
            if value2 < value1 {
                println!("input vector data is not sorted. Please, sort it and try again.");
                return vec.clone();
            }
            if 1 == value2 - value1 {
                vec_consumer.push(*value2);
                do_push = true;
                key1 = iterator1.next();

                key2 = iterator2.next();
                value1 = value2;
                index += 1;
            } else {
                do_push = false;
                key1 = iterator1.next();

                break;
            }
        }

        if !do_push {
            if max_consumer.len() < vec_consumer.len() {
                max_consumer = vec_consumer.clone();
            }
            vec_consumer = Vec::new();
        }
        index += 1;
    }

    max_consumer
}
