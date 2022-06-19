use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

pub struct TestFile {
    name: String,
    path: PathBuf,
}

pub type TestFileList = HashMap<String, TestFile>;

pub fn get_test_list() -> TestFileList {
    let mut test_dir = fs::read_dir("./texts").expect("Failed to read directory");
    let mut test_list: HashMap<String, TestFile> = HashMap::new();
    while let Some(dir_contents) = test_dir.next() {
        if let Ok(file) = dir_contents {
            let file_name = file
                .file_name()
                .into_string()
                .expect("Unable to convert string");
            let file_path = file.path();
            let hash_key = file_name.clone();
            let test_file = TestFile {
                name: file_name,
                path: file_path,
            };
            test_list.insert(hash_key, test_file);
        }
    }
    test_list
}

pub fn parse_text() {
    let file_content = fs::read_to_string("texts/sample.txt").expect("Failed to read file");
    println!("{:?}", file_content);
}
