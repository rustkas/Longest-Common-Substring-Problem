#![feature(test)]
#![feature(extern_crate_item_prelude)]

//

/*
cargo test --benches
cargo test --bench bench2 -- --nocapture
*/

extern crate find_longest_substring;
extern crate test;

#[cfg(test)]
mod tests {

    use test::Bencher;
    use find_longest_substring::*;
    use std::collections::HashMap;

    fn bench_find_longest_sequence (b: &mut Bencher,capacity:usize){
        let mut vector: Vec<usize> = Vec::with_capacity(capacity);
        for i in 0..vector.capacity() {
            vector.push(i);
        }

        b.iter(|| {
            let _ = find_longest_sequence(&mut vector);
        });
    }

    fn bench_make_string_info(b: &mut Bencher,capacity:usize) {

        let string:String = format!("{}", capacity);
        let input:&str = string.as_str();
        b.iter(|| {
            let _ = make_string_info(input);
        });
    }

    fn bench_merge_vectors(b: &mut Bencher, capacity:usize) {
        let mut vector1: Vec<usize> = Vec::with_capacity(capacity/2);
        let mut vector2: Vec<usize> = Vec::with_capacity(capacity/2);
        let mut i:usize = 0;
        for _ in 0..vector1.capacity() {
            vector1.push(i);
            i+=1;
        }
        for _ in 0..vector2.capacity() {
            vector2.push(i);
            i+=1;
        }
        let indexes1: Option<&Vec<usize>> = Some(&vector1);
        let indexes2: Option<&Vec<usize>> = Some(&vector2);

        b.iter(|| {
            let _ = merge_vectors(indexes1, indexes2);
        });
    }

    fn bench_create_max_string(b: &mut Bencher, capacity:usize) {
        let mut string:String = String::new();
        let mut tmp_string:String;
        for i in 0..capacity {
            tmp_string = format!("{}",i);
            string.push_str(tmp_string.as_str());
        }
        
        let input:&str = string.as_str();
       
        let fist: char = '1';
        let second: char = '2';
        let chars_info: (HashMap<char, Vec<usize>>) = make_string_info(input);

        b.iter(|| {
            let _ = create_max_string(fist, second, &chars_info);
        });
        
        
    }

    fn bench_find_max_string(b: &mut Bencher, capacity:usize) {

        let mut string:String = String::new();
        let mut tmp_string:String;
        for i in 0..capacity {
            tmp_string = format!("{}",i);
            string.push_str(tmp_string.as_str());
        }

        let input:&str = string.as_str();

        b.iter(|| {
            let _ = find_max_string(input);
        });


    }

    #[bench]
    fn find_longest_sequence_100(b: &mut Bencher) {
        let capacity:usize = 100;
        bench_find_longest_sequence (b, capacity)
    }

    #[bench]
    fn find_longest_sequence_1000(b: &mut Bencher) {
        let capacity:usize = 1000;
        bench_find_longest_sequence (b, capacity)
    }

    #[bench]
    fn find_longest_sequence_10000(b: &mut Bencher) {
        let capacity:usize = 10000;
        bench_find_longest_sequence (b, capacity)
    }

    #[bench]
    fn make_string_info_100(b: &mut Bencher) {
        let capacity:usize = 100;
        bench_make_string_info (b, capacity)
    }

    #[bench]
    fn make_string_info_1000(b: &mut Bencher) {
        let capacity:usize = 1000;
        bench_make_string_info (b, capacity)
    }

    #[bench]
    fn make_string_info_10000(b: &mut Bencher) {
        let capacity:usize = 10000;
        bench_make_string_info (b, capacity)
    }

    #[bench]
    fn merge_vectors_100(b: &mut Bencher) {
        let capacity:usize = 100;
        bench_merge_vectors (b, capacity)
    }

    #[bench]
    fn merge_vectors_1000(b: &mut Bencher) {
        let capacity:usize = 1000;
        bench_merge_vectors (b, capacity)
    }

    #[bench]
    fn merge_vectors_10000(b: &mut Bencher) {
        let capacity:usize = 10000;
        bench_merge_vectors (b, capacity)
    }

    #[bench]
    fn create_max_string_100(b: &mut Bencher) {
        let capacity:usize = 100;
        bench_create_max_string (b, capacity)
    }

    #[bench]
    fn create_max_string_1000(b: &mut Bencher) {
        let capacity:usize = 1000;
        bench_create_max_string (b, capacity)
    }

    #[bench]
    fn create_max_string_10000(b: &mut Bencher) {
        let capacity:usize = 10000;
        bench_create_max_string (b, capacity)
    }

    #[bench]
    fn find_max_string_100(b: &mut Bencher) {
        let capacity:usize = 100;
        bench_find_max_string (b, capacity)
    }

    #[bench]
    fn find_max_string_1000(b: &mut Bencher) {
        let capacity:usize = 1000;
        bench_find_max_string (b, capacity)
    }

    #[bench]
    fn find_max_string_10000(b: &mut Bencher) {
        let capacity:usize = 10000;
        bench_find_max_string (b, capacity)
    }

}
