mod cursor;
mod imagesort;
mod util;

use image::{ImageBuffer, RgbaImage, DynamicImage};
use std::{io, time::SystemTime, i64, cmp};
use rand::Rng;
//clap::Parser; // For CLI flags


fn main() {

    // TODO: Change input from prompts to CLI flags ie. --path './' --width 1000
    let board_width: u32;
    let board_height: u32;
    let image_path: &str = "./";
    
    println!("Enter desired WIDTH (pixels):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    board_width = input.trim().parse().expect("Please type a number!");

    println!("Enter desired HEIGHT (pixels):");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    board_height = input.trim().parse().expect("Please type a number!");


    // set image size from board size input
    let max_height = cmp::min(board_width, board_height) / 2;
    let max_width = cmp::min(board_width, board_height) / 2;     

    // TODO: create flag or prompt for boarder width
    let border: i64 = 10;
    
    // create blank artboard to build on
    let mut artboard: RgbaImage = ImageBuffer::new(board_width, board_height);
    
    // read in all files in directory
    let mut files: Vec<String> = Vec::new();

    // vec to store resized image buffers
    let mut images: Vec<DynamicImage> = Vec::new();

    match util::get_dir_item_names(&image_path) {
        Ok(val) => files = val,
        Err(e) => println!("[Error Item Retrieval: {}]", e),
    }
    
    // narrows list to inlude only valid image types
    let valid = util::remove_incompatible_filetypes(files.clone());

    // random generator
    let mut rng = rand::thread_rng();


    // randomize, re-size & add to buffer
    for image in valid {
        util::get_dims(&image_path, &image);
        let rand_size = rng.gen_range((max_width / 3)..max_width);
        // adding resized images to buffer
        images.push(util::resize_img(&image_path, &image, rand_size, rand_size));
    }

    // sort images in place so larger images lay first as background
    imagesort::lg_to_sm(&mut images);

    // main cursor used for insertion point reference
    let mut cursor = cursor::create_cursor();
    
    // overlay image & move cursor
    for img in &images {

        // overlay img ontop of artboard
        image::imageops::overlay(&mut artboard, img, cursor.x, cursor.y);

        // move cursor
        cursor.x += img.width() as i64 + border + rng.gen_range(0..(max_width / 6) as i64);
        cursor.y += (img.height() as f32 * 0.66) as i64 + border + rng.gen_range(0..(max_width / 6) as i64);

        // bumping cursor back into frame y
        if cursor.y > (board_height - img.height()) as i64{
            cursor.y = rng.gen_range(0..max_height - img.height()) as i64;
        }

        // bumping cursor back into frame x
        if cursor.x > (board_width - img.height()) as i64{
            cursor.x = rng.gen_range(0..max_width - img.width()) as i64;
        }

       }
    

    // gen unique filename to avoid overwrite
    let artboard_name: String;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => artboard_name = format!("artboard_{}.jpg", n.as_secs()),
        Err(_) => artboard_name = format!("artboard.jpg")
    }

    // save out
    match artboard.save(artboard_name.clone()){
        Err(e) => panic!("Write Error: {}", e),
        _ => (),
    }

    // success msg
    println!("Artboard Complete! >>> {}", artboard_name);
}

/////////////////// TODO Implement CLI tooling ///////////////////
//    let args = Args::parse();

//    let image_path = args.path;      
//    let board_height = args.width;
//    let board_width = args.height;    


// TODO: Struct for CLI implementation (future)
//#[derive(Parser, Debug)]
//#[command(version, about, long_about = None)]
//struct Args {
//    /// filepath to image(s) folder
//    #[arg(short, long, default_value = "./")]
//    path: String,
//
//    /// width    
//    #[arg(short, long, default_value_t = 2048)]
//    width: u32,
//
//    /// height    
//    #[arg(short, long, default_value_t = 2048)]
//    height: u32,
//}
//
