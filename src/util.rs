use image::{GenericImageView, DynamicImage};
use std::{fs, io};

pub fn get_dir_item_names(path: &str) -> io::Result<Vec<String>> {
    // Get a list of folder items
    let entries = fs::read_dir(path)?;

    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}


pub fn remove_incompatible_filetypes(filenames: Vec<String>) -> Vec<String> {
    let mut valid: Vec<String> = Vec::new();

    for file in filenames {
        if file.contains(".jpg") {
            valid.push(file.to_string());
        } else if file.contains(".png") {
            valid.push(file.to_string())
        } else {
            println!("Excluded invalid file: {}", file);
        }   
    }
    valid
}


pub fn resize_img(image_path: &str, image_name: &str, max_width: u32, max_height: u32)-> DynamicImage {
    let full_path = format!("{}{}", image_path, image_name);
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open(full_path).unwrap();

    // original dims
    println!("Start dimensions {:?}", img.dimensions());

    // resizing
    img = img.resize(max_width, max_height, image::imageops::FilterType::Nearest);
    println!("Finish dimensions {:?}", img.dimensions());

    img
}


pub fn get_dims(image_path: &str, image_name: &str) {
    let full_path = format!("{}{}", image_path, image_name);
    match imagesize::size(full_path) {
        Ok(size) => println!("Image dimensions: {}x{}", size.width, size.height),
        Err(why) => println!("Error getting dimensions: {:?}", why)
    }
}


