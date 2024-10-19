use atlas_language::utils::file_handling;

const FILE_NAME: &str = "index.atlas";
const FILE_PATH: &str = "./tests/index.atlas";
const FILE_SIZE: u64 = 41;

#[test]
fn test_file_data() {
    let file = file_handling::File::open(FILE_PATH).unwrap();
    assert_eq!(file.get_name(), FILE_NAME);
    assert_eq!(file.get_size(), FILE_SIZE);
    assert_eq!(file.get_extension(), FILE_NAME.split('.').last().unwrap());
}

#[test]

fn test_file_content() {
    let file = file_handling::File::open(FILE_PATH).unwrap();
    assert_eq!(file.get_content(), "1 + 1 # primer comentario\n\n# ultima linea");
}
