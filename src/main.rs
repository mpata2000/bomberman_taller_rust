mod bomb;
mod bomberman;
mod enemy;
mod obstacle;
mod point;

use crate::point::Point;

fn validate_args(args: Vec<String>) -> Result<(String, String, Point), String> {
    if args.len() != 4 {
        return Err(format!(
            "Incorrect number of arguments provided, need 4 got {}",
            args.len()
        ));
    }
    let input_path = format!("./{}", args[0].clone());
    let output_path = format!(
        "./{}{}",
        args[1],
        args[0].split('/').last().unwrap_or(args[0].as_str())
    );
    let x = args[2].parse::<u32>();
    let y = args[3].parse::<u32>();

    match (x, y) {
        (Ok(x), Ok(y)) => Ok((input_path, output_path, Point::new(x, y))),
        (_, _) => Err("x and y must be numbers and greater equal to 0".to_string()),
    }
}

fn read_file(path: String) -> Result<String, String> {
    match std::fs::read_to_string(path.clone()) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(format!("Error reading file from {}: {}", path, e)),
    }
}

fn write_out_file(path: String, contents: String) {
    if std::path::Path::new(&path).exists() == false {
        match std::fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(e) => {
                println!("Error creating directory {}: {}", path, e);
                return;
            }
        }
    }

    match std::fs::write(path, contents) {
        Ok(_) => (),
        Err(e) => println!("Error writing file: {}", e),
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
    let mut game = match bomberman::Bomberman::new(contents) {
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
