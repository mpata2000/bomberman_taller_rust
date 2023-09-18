use crate::bomb::Bomb;
use crate::enemy::Enemy;
use crate::obstacle::Obstacle;
use crate::point::Point;

pub(crate) struct Bomberman{
    enemies: Vec<Enemy>,
    bombs: Vec<Bomb>,
    obstacles: Vec<Obstacle>,
}

impl Bomberman {
    pub(crate) fn new(file_string:String) -> Result<Bomberman, String> {
        let lines: Vec<&str> = file_string.split("\n").collect();
        let mut enemies: Vec<Enemy> = Vec::new();
        let mut bombs: Vec<Bomb> = Vec::new();
        let mut obstacles: Vec<Obstacle> = Vec::new();

        let mut game = Bomberman {
            enemies,
            bombs,
            obstacles,
        };

        for (x,line) in lines.iter().enumerate(){
            let squares: Vec<&str> = line.split(" ").collect();
            if squares.len() != lines.len() {
                return Err(format!("Incorrect number of squares in line: {}", line));
            }
            for (y,square) in squares.iter().enumerate(){
                let point = Point::new(x as u32, y as u32);
                match game.add_square(square.to_string(), point){
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(game)
    }

    fn add_square(&mut self, square: String, point: Point) -> Result<(), String>{
        match square[0] {
            "F" => {
                let enemy = match Enemy::new(square, point) {
                    Ok(enemy) => enemy,
                    Err(e) => return Err(e),
                };
                self.enemies.push(enemy);
            }
            "B" | "S" => {
                let bomb = match Bomb::new(square, point) {
                    Ok(bomb) => bomb,
                    Err(e) => return Err(e),
                };
                self.bombs.push(bomb);
            }
            "R" | "W" => {
                let obstacle = match Obstacle::new(square, point) {
                    Ok(obstacle) => obstacle,
                    Err(e) => return Err(e),
                };
                self.obstacles.push(obstacle);
            }
            "D" => todo!("Implement deflection"),
            "_" => (),
            _ => return Err(format!("Invalid square: {}", square)),
        }
        Ok(())
    }
}