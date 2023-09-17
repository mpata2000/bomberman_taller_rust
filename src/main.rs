use std;

struct Point {
    x: u32,
    y: u32,
}

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
        (Ok(x), Ok(y)) if x > 0 && y > 0 => {
            Ok((
                input_path,
                output_path,
                Point { x, y },
            ))
        }
        (Ok(_), Ok(_)) => Err("x and y must be greater than 0".to_string()),
        (Err(_), Ok(_)) => Err("x must be a number".to_string()),
        (Ok(_), Err(_)) => Err("y must be a number".to_string()),
        (Err(_), Err(_)) => Err("x and y must be numbers".to_string()),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (input_path, output_path, start_point) = match validate_args(args) {
        Ok((input_path, output_path, point)) => (input_path, output_path, point),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

}
