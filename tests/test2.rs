extern crate find_longest_substring;

#[cfg(test)]
mod tests {
    use find_longest_substring::*;
    use std::cmp::Ordering;
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::io::{BufRead, BufReader, Read};
    use std::path::{Display, Path, PathBuf};

    fn get_tests_data_dir() -> PathBuf {
        let root_dir: &Path = Path::new(".");

        let file_path: PathBuf = root_dir.join("tests").join("data");
        file_path
    }

    #[test]
    //cargo test --test test2 read_data_from_a_file_1 -- --nocapture
    /// Read small peace of data to a string
    fn read_data_from_a_file_1() {
        let file_path: PathBuf = get_tests_data_dir().join("input1.txt");
        let path: &Path = file_path.as_path();
        let display: Display = file_path.display();
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let mut contents = String::new();

        let _ = match file.read_to_string(&mut contents) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            _ => (),
        };

        assert_eq!("CCGCCGGGCGCG", longest_substring(contents.as_str()));
    }

    #[test]
    //cargo test --test test2 read_rosalind_test_data -- --nocapture
    /// Write data for testing Rosalind
    ///
    fn read_rosalind_test_data() {
        let file_data_for_test: File;
        {
            let file_path: PathBuf = get_tests_data_dir().join("rosalind.txt");
            let path: &Path = file_path.as_path();
            let display: Display = file_path.display();

            file_data_for_test = match File::open(path) {
                Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                Ok(file) => file,
            };
        }

        let file_expected_data: File;
        {
            let file_path: PathBuf = get_tests_data_dir().join("rosalind_expected.txt");
            let path: &Path = file_path.as_path();
            let display: Display = file_path.display();

            file_expected_data = match File::open(path) {
                Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                Ok(file) => file,
            };
        }
        let tested_data_buffer_reader = BufReader::new(file_data_for_test).lines();
        let mut expected_data_buffer_reader = BufReader::new(file_expected_data);

        for line in tested_data_buffer_reader {
            if let Ok(string_line) = line {
                if string_line.starts_with(">") {
                    continue;
                } else {
                    let new_line = longest_substring(string_line.as_str());
                    let mut expected_line: String = String::new();
                    if let Err(e) = expected_data_buffer_reader.read_line(&mut expected_line) {
                        eprintln!("reading from cursor won't fail. {}", e);
                    }
                    expected_line = expected_line.trim().to_string();
                    let result = expected_line.cmp(&new_line.to_string());
                    // println!("{} | {}", &expected_line, &new_line.to_string() );
                    assert_eq!(Ordering::Equal, result);
                }
            }
        }
    }

    #[test]
    #[ignore]
    //cargo test --test test2 ignore_read_rosalind_test_data -- --nocapture
    /// Write data for testing Rosalind
    ///
    fn ignore_read_rosalind_test_data() {
        let file_path: PathBuf = get_tests_data_dir().join("rosalind.txt");
        let path: &Path = file_path.as_path();
        let display: Display = file_path.display();

        let file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let output_file_path: PathBuf = get_tests_data_dir().join("rosalind_expected.txt");
        let output_path: &Path = output_file_path.as_path();

        if output_path.exists() {
            if let Err(e) = fs::remove_file(output_path) {
                eprintln!("Can't remove a file: {}", e);
            }
        }

        let mut output_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(output_path)
            .unwrap();

        for line in BufReader::new(file).lines() {
            if let Ok(string_line) = line {
                if string_line.starts_with(">") {
                    // println!("{}",string_line);
                    continue;
                } else {
                    //println!("{}",longest_substring(string_line.as_str()));
                    // output_buffer_writer.write_all(string_line.as_bytes()).expect("Unable to write data");
                    // output_buffer_writer.write_all(string_line.as_bytes()).expect("Unable to write data");
                    let new_line = longest_substring(string_line.as_str());

                    if let Err(e) = writeln!(output_file, "{}", new_line.as_ref()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        }
    }
}
