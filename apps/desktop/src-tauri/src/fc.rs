use anyhow::Result as AnyResult;
use mf_utils::is_md_file_name;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// #[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    name: String,
    kind: String,
    path: String,
    children: Option<Vec<FileInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    created: String,
    link: String,
    description: String,
    content: String,
    author: String,
}

pub fn read_directory(dir_path: &str) -> Vec<FileInfo> {
    let new_path = Path::new(dir_path);
    let paths = fs::read_dir(new_path).unwrap();

    let mut files: Vec<FileInfo> = Vec::new();

    for path in paths {
        let path_unwrap = path.unwrap();
        let meta = path_unwrap.metadata();
        let meta_unwrap = meta.unwrap();

        let mut kind = String::from("file");

        let mut children: Option<Vec<FileInfo>> = None;

        let filename = match path_unwrap.file_name().into_string() {
            Ok(str) => str,
            Err(_error) => String::from("ERROR"),
        };

        let file_path = new_path.join(filename.clone());

        if meta_unwrap.is_dir() {
            kind = String::from("dir");
            children = Some(read_directory(file_path.to_str().unwrap()));
        }

        let new_file_info = FileInfo {
            name: filename,
            kind,
            path: file_path.to_str().unwrap().to_string(),
            children,
        };

        if is_md_file_name(&new_file_info.name) || meta_unwrap.is_dir() {
            files.push(new_file_info);
        }
    }

    sort_files_by_kind_and_name(&mut files);

    files
}

pub fn sort_files_by_kind_and_name(files: &mut Vec<FileInfo>) {
    files.sort_by(|a, b| {
        if a.kind == b.kind {
            a.name.cmp(&b.name)
        } else {
            if a.kind == "dir" {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    });
}

pub fn files_to_json(files: Vec<FileInfo>) -> String {
    let files_str = match serde_json::to_string(&files) {
        Ok(str) => str,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    files_str
}

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("ERROR");
    contents
}

// update file and create new file
pub fn write_file(path: &str, content: &str) -> String {
    let file_path = Path::new(path);
    let result = match fs::write(file_path, content) {
        Ok(()) => String::from("OK"),
        Err(_err) => String::from("ERROR"),
    };

    result
}

pub fn exists(path: &Path) -> bool {
    Path::new(path).exists()
}

pub fn create_file<P: AsRef<Path>>(filename: P) -> AnyResult<()> {
    let filename = filename.as_ref();
    if let Some(parent) = filename.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::File::create(filename)?;
    Ok(())
}

// pub fn create_directory(path: &str) -> SerdeResult<()> {
//     let dir_path = Path::new(path);
//     fs::create_dir(dir_path);
//     Ok(())
// }

pub fn remove_file(path: &str) -> AnyResult<()> {
    let file_path = Path::new(path);
    fs::remove_file(file_path)?;
    Ok(())
}

pub fn remove_folder(path: &str) -> AnyResult<()> {
    let folder_path = Path::new(path);
    fs::remove_dir_all(folder_path)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveFileInfo {
    old_path: String,
    new_path: String,
    is_folder: bool,
    children: Option<Vec<FileInfo>>,
}

pub fn move_files_to_target_folder(
    files: Vec<String>,
    target_folder: &str,
) -> AnyResult<Vec<MoveFileInfo>> {
    let mut path_map_old_to_new = vec![];

    for file in files {
        let file_path = Path::new(&file);
        let file_name = file_path.file_name().unwrap();
        let target_path = Path::new(target_folder).join(file_name);

        if target_path.exists() {
            continue;
        }

        fs::rename(file_path, target_path.clone())?;

        let is_folder = target_path.clone().is_dir();

        if is_folder {
            let files = read_directory(target_path.to_str().unwrap());

            path_map_old_to_new.push(MoveFileInfo {
                old_path: file.to_string(),
                new_path: target_path.to_str().unwrap().to_string(),
                is_folder: is_folder,
                children: Some(files),
            });
        } else {
            path_map_old_to_new.push(MoveFileInfo {
                old_path: file.to_string(),
                new_path: target_path.to_str().unwrap().to_string(),
                is_folder: is_folder,
                children: None,
            });
        }
 
    }

    Ok(path_map_old_to_new)
}

pub mod cmd {
    use std::path::Path;

    use crate::fc;

    use super::MoveFileInfo;

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    #[tauri::command]
    pub fn open_folder(folder_path: &str) -> String {
        let files = fc::files_to_json(fc::read_directory(folder_path));
        files
    }

    #[tauri::command]
    pub fn get_file_content(file_path: &str) -> String {
        let content = fc::read_file(file_path);
        content
    }

    #[tauri::command]
    pub fn write_file(file_path: &str, content: &str) -> String {
        fc::write_file(file_path, content);
        String::from("OK")
    }

    #[tauri::command]
    pub fn delete_file(file_path: &str) -> String {
        fc::remove_file(file_path);
        String::from("OK")
    }

    #[tauri::command]
    pub fn delete_folder(file_path: &str) -> String {
        let _ = fc::remove_folder(file_path);
        String::from("OK")
    }

    #[tauri::command]
    pub fn file_exists(file_path: &str) -> bool {
        fc::exists(Path::new(file_path))
    }

    #[tauri::command]
    pub fn move_files_to_target_folder(
        files: Vec<String>,
        target_folder: &str,
    ) -> Option<Vec<MoveFileInfo>> {
        let res = fc::move_files_to_target_folder(files, target_folder);
        
        match res {
            Ok(path_map_old_to_new) => Some(path_map_old_to_new),
            Err(_) => None,
        }
    }

    #[tauri::command]
    pub fn path_join(path1: &str, path2: &str) -> String {
        let path = Path::new(path1).join(path2);
        path.to_str().unwrap().to_string()
    }
}
