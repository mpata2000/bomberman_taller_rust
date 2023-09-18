use crate::bomb::{Bomb, CanBeHit};
use crate::enemy::Enemy;
use crate::obstacle::Obstacle;
use crate::point::Point;

#[derive(Debug)]
pub(crate) struct Bomberman {
    enemies: Vec<Enemy>,
    bombs: Vec<Bomb>,
    obstacles: Vec<Obstacle>,
}

impl Bomberman {
    pub(crate) fn new(file_string: String) -> Result<Bomberman, String> {
        let lines: Vec<&str> = file_string.split("\n").collect();
        let enemies: Vec<Enemy> = Vec::new();
        let bombs: Vec<Bomb> = Vec::new();
        let obstacles: Vec<Obstacle> = Vec::new();

        let mut game = Bomberman {
            enemies,
            bombs,
            obstacles,
        };

        for (y, line) in lines.iter().enumerate() {
            let squares: Vec<&str> = line.split(" ").collect();
            if squares.len() != lines.len() {
                return Err(format!("Incorrect number of squares in line: {}", line));
            }
            for (x, square) in squares.iter().enumerate() {
                let point = Point::new(x as u32, y as u32);
                match game.add_square(square.to_string(), point) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(game)
    }

    fn add_square(&mut self, square: String, point: Point) -> Result<(), String> {
        let first_char = match square.chars().next() {
            Some(first_char) => first_char,
            None => return Err("Empty square string".to_string()),
        };
        match first_char {
            'F' => {
                let enemy = match Enemy::new(square, point) {
                    Ok(enemy) => enemy,
                    Err(e) => return Err(e),
                };
                self.enemies.push(enemy);
            }
            'B' | 'S' => {
                let bomb = match Bomb::new(square, point) {
                    Ok(bomb) => bomb,
                    Err(e) => return Err(e),
                };
                self.bombs.push(bomb);
            }
            'R' | 'W' => {
                let obstacle = match Obstacle::new(square, point) {
                    Ok(obstacle) => obstacle,
                    Err(e) => return Err(e),
                };
                self.obstacles.push(obstacle);
            }
            'D' => todo!("Implement deflection"),
            '_' => (),
            _ => return Err(format!("Invalid square: {}", square)),
        }
        Ok(())
    }

    fn there_are_active_bombs(&mut self) -> bool {
        self.bombs.iter().any(|bomb| bomb.is_active())
    }

    fn get_hittable_in_position(&mut self, position: Point) -> Option<&mut dyn CanBeHit> {
        let enemy = self.enemies.iter_mut().find(|enemy| enemy.is_in_position(position));
        match enemy {
            Some(enemy) => return Some(enemy),
            None => (),
        }
        let bomb = self.bombs.iter_mut().find(|bomb| bomb.is_in_position(position));
        match bomb {
            Some(bomb) => return Some(bomb),
            None => (),
        }
        None
    }

    pub(crate) fn play(&mut self, start_bomb: Point) -> Result<String,String>{
        let first_bomb = self.bombs.iter_mut().find(|bomb| bomb.is_in_position(start_bomb));
        match first_bomb {
            Some(bomb) => bomb.hit(),
            None => return Err("No bomb in starting position".to_string()),
        }

        // TODO: Fix Searching twice?
        while self.there_are_active_bombs(){
            let bomb = self.bombs.iter_mut().find(|bomb| bomb.is_active());
            let afected_positions = match bomb {
                Some(bomb) => bomb.explode(&self.obstacles),
                None => return Err("No active bomb found".to_string()),
            };
            for position in afected_positions {
                let hittable = self.get_hittable_in_position(position);
                match hittable {
                    Some(hittable) => hittable.hit(),
                    None => (),
                }
            }
        }

        Ok("".to_string())
    }
}
