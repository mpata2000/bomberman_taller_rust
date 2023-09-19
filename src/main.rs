mod bomb;
mod bomberman;
mod enemy;
mod obstacle;
mod point;

use bomberman::Bomberman;

use crate::point::Point;
use std::{self, ops::Add};

fn validate_args(args: Vec<String>) -> Result<(String, String, Point), String> {
    if args.len() != 4 {
        return Err(format!(
            "Incorrect number of arguments provided, need 4 got {}",
            args.len()
        ));
    }
    let input_path = args[0].clone();
    let output_path = args[1].clone();
    let x = args[2].parse::<u32>();
    let y = args[3].parse::<u32>();

    match (x, y) {
        (Ok(x), Ok(y)) => Ok((input_path, output_path, Point::new(x, y))),
        (_, _) => Err("x and y must be numbers and greater equal to 0".to_string()),
    }
}

fn read_file(path: String) -> Result<String, String> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}

fn write_out_file(path: String, file_name: String, contents: String) {
    match std::fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(e) => {
            println!("Error creating directory: {}", e);
            return;
        }
    }
    
    let full_path = format!("{}/{}", path, file_name);
    match std::fs::write(full_path, contents) {
        Ok(_) => (),
        Err(e) => println!("Error writing file: {}", e),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (input_file, output_path, start_point) = match validate_args(args) {
        Ok((input_path, output_path, point)) => (input_path, output_path, point),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let contents = match read_file(format!("./{}", input_file)) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut game = match bomberman::Bomberman::new(contents) {
        Ok(game) => game,
        Err(e) => {
            write_out_file(output_path,input_file, e.to_string());
            return;
        }
    };

    let result = match game.play(start_point){
        Ok(result) => result,
        Err(e) => e.to_string(),
    };

    write_out_file(output_path,input_file, result);
}
