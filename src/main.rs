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
    let temp_x = args[2].parse::<u32>();
    let temp_y = args[3].parse::<u32>();
    match (temp_x, temp_y) {
        (Ok(x), Ok(y)) => {
            if x < 0 || y < 0 {
                return Err(format!("x and y must be greater than 0"));
            }
        }
        (Err(_), Ok(_)) => return Err(format!("x must be a number")),
        (Ok(_), Err(_)) => return Err(format!("y must be a number")),
        (Err(_), Err(_)) => return Err(format!("x and y must be numbers")),
    }
    Ok((
        input_path,
        output_path,
        Point {
            x: temp_x.unwrap_or(0),
            y: temp_y.unwrap_or(0),
        },
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        println!(
            "Incorrect number of arguments provided, need 4 got {}",
            args.len()
        );
        println!("Usage: cargo run -- <input.txt> <output_path> <x> <y>");
        return;
    }
}
