#![feature(type_ascription)]
#![feature(core_intrinsics)]
#![feature(exact_size_is_empty)]

extern crate find_longest_substring;
/*
cargo test --test test1
cargo test --test test1 iter1 -- --nocapture
cargo test --test test1 make_string_info -- --nocapture
cargo test --test test1 create_max_string -- --nocapture
*/

#[cfg(test)]
mod tests {
    use find_longest_substring::*;
    use std::borrow::Cow;
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[allow(dead_code)]
    /// Util method. Prit information about type of a variable
    fn print_type_of<T>(_: &T) {
        println!("{}", unsafe { std::intrinsics::type_name::<T>() });
    }

    // TESTS:

    #[test]
    // cargo test --test test1 find_longest_sequence_1 -- --nocapture
    fn find_longest_sequence_1() {
        let mut vector: Vec<usize> = vec![1, 2, 3, 4, 6, 8];
        let expected: Vec<usize> = vec![1, 2, 3, 4];
        let result: Vec<usize> = find_longest_sequence(&mut vector);

        let result = expected.cmp(&result);
        assert_eq!(Ordering::Equal, result);
    }

    #[test]
    // cargo test --test test1 find_longest_sequence_2 -- --nocapture
    fn find_longest_sequence_2() {
        let mut vector: Vec<usize> = vec![1, 2, 3, 4, 6, 8];
        vector.reverse();
        //println!("{:?}", &vector);

        let expected: Vec<usize> = vec![1, 2, 3, 4];
        let result: Vec<usize> = find_longest_sequence(&mut vector);
        // println!("{:?}", &result);
        let result = expected.cmp(&result);
        assert_eq!(Ordering::Equal, result);
    }

    #[test]
    // cargo test --test test1 find_longest_sequence_3 -- --nocapture
    fn find_longest_sequence_3() {
        let mut vector: Vec<usize> = vec![1, 3, 6, 8, 10, 11, 12, 13, 14];
        vector.reverse();
        //println!("{:?}", &vector);

        let expected: Vec<usize> = vec![10, 11, 12, 13, 14];
        let result: Vec<usize> = find_longest_sequence(&mut vector);
        // println!("{:?}", &result);
        let result = expected.cmp(&result);
        assert_eq!(Ordering::Equal, result);
    }

    #[test]
    // cargo test --test test1 find_longest_sequence_4 -- --nocapture
    fn find_longest_sequence_4() {
        let mut vector: Vec<usize> = vec![];
        vector.reverse();
        //println!("{:?}", &vector);

        let expected: Vec<usize> = vec![];
        let result: Vec<usize> = find_longest_sequence(&mut vector);
        // println!("{:?}", &result);
        let result = expected.cmp(&result);
        assert_eq!(Ordering::Equal, result);
    }

    #[test]
    // cargo test --test test1 make_string_info_1 -- --nocapture
    fn make_string_info_1() {
        let input = "";

        let chars_info: HashMap<char, Vec<usize>> = make_string_info(input);
        //println!("{:?}", chars_info);

        let hash_map_size: usize = chars_info.len();
        assert_eq!(0, hash_map_size);
    }

    #[test]
    // cargo test --test test1 make_string_info_2 -- --nocapture
    fn make_string_info_2() {
        let input = "aaaa";
        const A: char = 'a';
        let chars_info: HashMap<char, Vec<usize>> = make_string_info(input);

        assert_eq!(false, chars_info.keys().next().is_none());

        let hash_map_size: usize = chars_info.len();
        assert_eq!(1, hash_map_size);

        if let Some(key) = chars_info.keys().next() {
            assert_eq!(A, *key);
        }

        if let Some(content) = chars_info.get(&A) {
            let expected: Vec<usize> = vec![0, 1, 2, 3];

            let result = expected.cmp(&content);
            assert_eq!(Ordering::Equal, result);
        } else {
            panic!("no content was found");
        }
    }

    #[test]
    // cargo test --test test1 make_string_info_3 -- --nocapture
    fn make_string_info_3() {
        let input = "aaaaBBBBBB";
        const A: char = 'a';
        const B: char = 'B';
        let chars_info: HashMap<char, Vec<usize>> = make_string_info(input);

        assert_eq!(false, chars_info.keys().is_empty());

        let hash_map_size: usize = chars_info.len();
        assert_eq!(2, hash_map_size);

        {
            let expected_key = A;
            let mut key_found: bool = false;
            let keys = chars_info.keys();
            for key in keys {
                if *key == expected_key {
                    key_found = true;
                    break;
                }
            }

            assert_eq!(true, key_found, "key {} is not found", expected_key);
        }

        {
            let expected_key = B;
            let mut key_found: bool = false;
            let keys = chars_info.keys();
            for key in keys {
                if *key == expected_key {
                    key_found = true;
                    break;
                }
            }

            assert_eq!(true, key_found, "key {} is not found", expected_key);
        }

        if let Some(content) = chars_info.get(&A) {
            let expected: Vec<usize> = vec![0, 1, 2, 3];

            let result = expected.cmp(&content);
            assert_eq!(Ordering::Equal, result);
        } else {
            panic!("no content {} was found", &A);
        }

        if let Some(content) = chars_info.get(&B) {
            let expected: Vec<usize> = vec![4, 5, 6, 7, 8, 9];

            let result = expected.cmp(&content);
            assert_eq!(Ordering::Equal, result);
        } else {
            panic!("no content {} was found", &B);
        }
    }

    #[test]
    // cargo test --test test1 make_string_info_4 -- --nocapture
    fn make_string_info_4() {
        let input = "aaBcccD";
        const A: char = 'a';
        const B: char = 'B';
        const C: char = 'c';
        const D: char = 'D';

        let vector: Vec<char> = vec!['a', 'B', 'c', 'D'];

        let chars_info: HashMap<char, Vec<usize>> = make_string_info(input);

        for vector_item in vector {
            let expected_key = vector_item;
            let mut key_found: bool = false;
            let keys = chars_info.keys();
            for key in keys {
                if *key == expected_key {
                    key_found = true;
                    break;
                }
            }

            assert_eq!(true, key_found, "key {} is not found", expected_key);
        }

        let array: [(char, Vec<usize>); 4] = [
            (A, vec![0, 1]),
            (B, vec![2]),
            (C, vec![3, 4, 5]),
            (D, vec![6]),
        ];
        for array_item in array.iter() {
            let char_item = array_item.0;
            let expected = &array_item.1;
            if let Some(content) = chars_info.get(&char_item) {
                let result = expected.cmp(&content);
                assert_eq!(Ordering::Equal, result);
            } else {
                panic!("no content {} was found", &char_item);
            }
        }
    }

    #[test]
    //cargo test --test test1 merge_vectors_1 -- --nocapture
    fn merge_vectors_1() {
        let vector1: Vec<usize> = vec![];
        let vector2: Vec<usize> = vec![];
        let indexes1: Option<&Vec<usize>> = Some(&vector1);
        let indexes2: Option<&Vec<usize>> = Some(&vector2);

        let expected_sequence: Vec<usize> = vec![];
        let result: Vec<usize> = merge_vectors(indexes1, indexes2);

        let result = expected_sequence.cmp(&result);
        assert_eq!(Ordering::Equal, result);
    }

    #[test]
    //cargo test --test test1 merge_vectors_2 -- --nocapture
    fn merge_vectors_2() {
        let vector1: Vec<usize> = vec![0, 1, 2];
        let vector2: Vec<usize> = vec![3, 4, 5];
        let indexes1: Option<&Vec<usize>> = Some(&vector1);
        let indexes2: Option<&Vec<usize>> = Some(&vector2);

        let expected_sequence: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let result: Vec<usize> = merge_vectors(indexes1, indexes2);

        let result = expected_sequence.cmp(&result);
        assert_eq!(Ordering::Equal, result);
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "first vector. no data indexes")]
    //cargo test --test test1 merge_vectors_3 -- --nocapture
    fn merge_vectors_3() {
        let indexes1: Option<&Vec<usize>> = None;
        let indexes2: Option<&Vec<usize>> = None;

        let _result: Vec<usize> = merge_vectors(indexes1, indexes2);
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "second vector. no data indexes")]
    //cargo test --test test1 merge_vectors_4 -- --nocapture
    fn merge_vectors_4() {
        let vector1: Vec<usize> = vec![0, 1, 2];
        let indexes1: Option<&Vec<usize>> = Some(&vector1);
        let indexes2: Option<&Vec<usize>> = None;

        let _result: Vec<usize> = merge_vectors(indexes1, indexes2);
    }

    #[test]
    //cargo test --test test1 create_max_string_1 -- --nocapture
    fn create_max_string_1() {
        let input = "";

        let fist: char = 'a';
        let second: char = 'c';
        let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);

        let max_string: Cow<str> = create_max_string(fist, second, &chars_info);
        // println!("{}", max_string.as_ref());
        assert_eq!("", max_string.as_ref());
    }

    #[test]
    //cargo test --test test1 create_max_string_2 -- --nocapture
    fn create_max_string_2() {
        let input = "ac";

        let fist: char = 'a';
        let second: char = 'c';
        let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);

        let max_string: Cow<str> = create_max_string(fist, second, &chars_info);
        // println!("{}", max_string.as_ref());
        assert_eq!("ac", max_string.as_ref());
    }

    #[test]
    //cargo test --test test1 create_max_string_3 -- --nocapture
    fn create_max_string_3() {
        let input = "Baacc";

        let fist: char = 'a';
        let second: char = 'c';
        let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);

        let max_string: Cow<str> = create_max_string(fist, second, &chars_info);
        // println!("{}", max_string.as_ref());
        assert_eq!("aacc", max_string.as_ref());
    }

    #[test]
    //cargo test --test test1 create_max_string_4 -- --nocapture
    fn create_max_string_4() {
        let input = "BBBaaccDDADADAFS";

        let fist: char = 'D';
        let second: char = 'A';
        let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);

        let max_string: Cow<str> = create_max_string(fist, second, &chars_info);
        // println!("{}", max_string.as_ref());
        assert_eq!("DDADADA", max_string.as_ref());
    }

    #[test]
    //cargo test --test test1 create_max_string_5 -- --nocapture
    fn create_max_string_5() {
        let input = "ABBA";

        let fist: char = 'A';
        let second: char = 'B';
        let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);

        let max_string: Cow<str> = create_max_string(fist, second, &chars_info);
        // println!("{}", max_string.as_ref());
        assert_eq!("ABBA", max_string.as_ref());
    }

    #[test]
    //cargo test --test test1 find_max_string_1 -- --nocapture
    fn find_max_string_1() {
        let input = "";
        let v: Cow<str> = find_max_string(input);

        assert_eq!("", v.as_ref());
    }

    #[test]
    //cargo test --test test1 find_max_string_2 -- --nocapture
    fn find_max_string_2() {
        let input = "a";
        let v: Cow<str> = find_max_string(input);

        assert_eq!("a", v.as_ref());
    }

    #[test]
    //cargo test --test test1 find_max_string_3 -- --nocapture
    fn find_max_string_3() {
        let input = "ab";
        let v: Cow<str> = find_max_string(input);

        assert_eq!("ab", v.as_ref());
    }
    #[test]
    //cargo test --test test1 find_max_string_4 -- --nocapture
    fn find_max_string_4() {
        let input = "abababababAC";
        let v: Cow<str> = find_max_string(input);

        assert_eq!("ababababab", v.as_ref());
    }

    #[test]
    //cargo test --test test1 find_max_string_5 -- --nocapture
    fn find_max_string_5() {
        let input = "ACabababababACDC";
        let v: Cow<str> = find_max_string(input);

        assert_eq!("ababababab", v.as_ref());
    }

    #[test]
    //cargo test --test test1 longest_substring_1 -- --nocapture
    fn longest_substring_1() {
        let value = longest_substring("");
        assert_eq!("", value);
    }

    #[test]
    //cargo test --test test1 longest_substring_2 -- --nocapture
    fn longest_substring_2() {
        let value = longest_substring("1");
        assert_eq!("1", value);
    }

    #[test]
    //cargo test --test test1 longest_substring_3 -- --nocapture
    fn longest_substring_3() {
        assert_eq!(
            "ADDAADDAADDAADDAADDA",
            longest_substring("ADDAADDASWADDAADDAADDAADDAADDA")
        );
    }

    #[test]
    //cargo test --test test1 longest_substring_4 -- --nocapture
    fn longest_substring_4() {
        assert_eq!("acaa", longest_substring("abcabacaa"));
    }

    #[test]
    //cargo test --test test1 longest_substring_5 -- --nocapture
    fn longest_substring_5() {
        assert_eq!(
            "CACCA",
            longest_substring("ATGAGTGTCTAATACCGAAGACGTCACCAGGCTCAACATATCTCTAGTTTAGCACCCTTA")
        );
    }



}
