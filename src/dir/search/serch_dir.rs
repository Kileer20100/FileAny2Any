use std::fs::read_dir;
use std::path::PathBuf;

pub fn serch_dir(path_dir_fn: String) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let mut stack = vec![PathBuf::from(path_dir_fn)]; // стек папок для обхода

    let mut dir_list: Vec<String> = Vec::new();
    let mut file_list: Vec<String> = Vec::new();

    while let Some(path) = stack.pop() {
        for entry in read_dir(&path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                //println!("Папка: {}", entry_path.display());
                stack.push(entry_path.clone()); // добавляем папку в стек для дальнейшего обхода
                dir_list.push(entry_path.display().to_string());
            } else if entry_path.is_file() {
                //println!("Файл: {}", entry_path.display());
                file_list.push(entry_path.display().to_string());
            }
        }
    }

    Ok((file_list, dir_list))

}