use ej_individual::bomberman::Bomberman;
use ej_individual::input_errors::InputError;
use ej_individual::utils::point::Point;

// Format the output path to remove leading and trailing slashes if they exist
// Return the formatted path like ./{path}/
fn format_out_path(path: &str) -> String {
    let mut path = path.trim_end_matches('/');
    path = path.trim_start_matches('/');
    format!("./{path}/")
}

// Validate the arguments provided to the program
// Creates the output directory if it doesn't exist
// Return the input file path, output file path and starting point with correct format
fn validate_args(args: &[String]) -> Result<(String, String, Point), InputError> {
    if args.len() != 4 {
        return Err(InputError::InvalidInput(format!(
            "incorrect number of arguments provided, need 4 got {}",
            args.len()
        )));
    }
    let dir = format_out_path(&args[1]);
    match create_dir(&dir) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    let input_path = format!("./{}", args[0].trim_start_matches('/'));
    let output_path = format!(
        "{dir}{}",

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
fn create_dir(path: &str) -> Result<(), InputError> {
    if std::path::Path::new(path).exists() {
        return Ok(());
    }

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => Ok(()),
        Err(e) => Err(InputError::FileError(format!(
            "error reading file {path}, context {e}"
        ))),
    }
}

// Read file contents to string
fn read_file(path: &str) -> Result<String, InputError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(InputError::FileError(format!(
            "error reading file {path}, context {e}"
        ))),
    }
}

// Write string contents to file
fn write_out_file(path: &str, contents: String) {
    match std::fs::write(path, contents) {
        Ok(_) => (),
        Err(e) => println!("Error writing file {path}: {e}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (input_file, output_path, start_point) = match validate_args(&args) {
        Ok((input_file, output_path, point)) => (input_file, output_path, point),
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let contents = match read_file(&input_file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let mut game = match Bomberman::new(&contents) {
        Ok(game) => game,
        Err(e) => {
            write_out_file(&output_path, e.to_string());
            return;
        }
    };

    let result = match game.play(start_point) {
        Ok(result) => result,
        Err(e) => e.to_string(),
    };

    write_out_file(&output_path, result);
}
