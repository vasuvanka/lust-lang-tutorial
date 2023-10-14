use std::{fs, io::ErrorKind};

fn main() {
    let s = read_dir("src".to_owned());
    for path in s {
        let file_data = read_file(path);
        println!("{:?}", file_data);
    }
}

fn read_dir(file_path: String) -> Vec<String> {
    let paths = fs::read_dir(file_path).unwrap();
    let mut my_vec: Vec<String> = Vec::new();
    for path in paths {
        my_vec.push(path.unwrap().path().display().to_string());
    }
    return my_vec;
}

fn read_file(file_path: String) -> Vec<String> {
    let metadata = fs::metadata(&file_path).unwrap();
    println!("{:?}", metadata.file_type());
    let files = fs::read_to_string(file_path);
    let mut my_vec: Vec<String> = Vec::new();
    let result = match files {
        Ok(files) => files,
        Err(e) if e.kind() == ErrorKind::NotFound => panic!("file not found : {}", e),
        Err(e) if e.kind() == ErrorKind::PermissionDenied => panic!("permission denied : {}", e),
        Err(e) => panic!("unknown error : {}", e),
    };

    result
        .lines()
        .for_each(|line| my_vec.push(line.to_string()));
    return my_vec;
}
