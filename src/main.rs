mod bomberman;
mod input_errors;

use crate::bomberman::game::Bomberman;
use crate::bomberman::utils::point::Point;
use crate::input_errors::InputError;

// Format the output path to remove leading and trailing slashes if they exist
// Return the formatted path like ./{path}/
fn format_out_path(path: String) -> String {
    let mut path = path.trim_end_matches('/');
    path = path.trim_start_matches('/');
    format!("./{}/", path)
}

// Validate the arguments provided to the program
// Creates the output directory if it doesn't exist
// Return the input file path, output file path and starting point with correct format
fn validate_args(args: Vec<String>) -> Result<(String, String, Point), InputError> {
    if args.len() != 4 {
        return Err(InputError::InvalidInput(format!(
            "incorrect number of arguments provided, need 4 got {}",
            args.len()
        )));
    }
    let dir = format_out_path(args[1].clone());
    match create_dir(dir.clone()) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    let input_path = format!("./{}", args[0].trim_start_matches('/'));
    let output_path = format!(
        "{}{}",
        dir,
        args[0].split('/').last().unwrap_or(args[0].as_str())
    );

    let x = args[2].parse::<u32>();
    let y = args[3].parse::<u32>();

    match (x, y) {
        (Ok(x), Ok(y)) => Ok((input_path, output_path, Point::new(x, y))),
        _ => Err(InputError::InvalidInput(
            "invalid starting point, x and y should be positive numbers".to_string(),
        )),
    }
}

// Create a directory if it doesn't exist
fn create_dir(path: String) -> Result<(), InputError> {
    if std::path::Path::new(&path).exists() {
        return Ok(());
    }

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => Ok(()),
        Err(e) => Err(InputError::FileError(format!(
            "error creating directory {}, context {}",
            path, e
        ))),
    }
}

// Read file contents to string
fn read_file(path: String) -> Result<String, InputError> {
    match std::fs::read_to_string(path.clone()) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(InputError::FileError(format!(
            "error reading file {}, context {}",
            path, e
        ))),
    }
}

// Write string contents to file
fn write_out_file(path: String, contents: String) {
    match std::fs::write(path.clone(), contents) {
        Ok(_) => (),
        Err(e) => println!("Error writing file {}: {}", path, e),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (input_file, output_path, start_point) = match validate_args(args) {
        Ok((input_file, output_path, point)) => (input_file, output_path, point),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let contents = match read_file(input_file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut game = match Bomberman::new(contents) {
        Ok(game) => game,
        Err(e) => {
            write_out_file(output_path, e.to_string());
            return;
        }
    };

    let result = match game.play(start_point) {
        Ok(result) => result,
        Err(e) => e.to_string(),
    };

    write_out_file(output_path, result);
}
