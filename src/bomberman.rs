use crate::bomberman_errors::BombermanError;
use crate::maze_placeable::bomb::Bomb;
use crate::maze_placeable::enemy::Enemy;
use crate::maze_placeable::obstacle::Obstacle;
use crate::maze_placeable::obstacle_type::ObstacleType;
use crate::maze_placeable::{bomb_type, enemy};
use crate::utils::can_be_hit::CanBeHit;
use crate::utils::maze_display::MazeDisplay;
use crate::utils::point::Point;
use std::fmt::Display;

#[derive(Debug)]
pub struct Bomberman {
    enemies: Vec<Enemy>,
    bombs: Vec<Bomb>,
    obstacles: Vec<Obstacle>,
    size: u32,
}
impl Bomberman {
    // Create a new game from a string
    // The string should be a square matrix of squares separated by spaces
    pub fn new(file_string: String) -> Result<Bomberman, BombermanError> {
        let lines: Vec<&str> = file_string.trim().split('\n').collect();

        let mut game = Bomberman {
            enemies: Vec::new(),
            bombs: Vec::new(),
            obstacles: Vec::new(),
            size: lines.len() as u32,
        };

        for (y, line) in lines.iter().enumerate() {
            let squares: Vec<&str> = line.trim().split(' ').collect();
            if squares.len() != game.size as usize {
                return Err(BombermanError::MazeNotSquare(format!(
                    "Maze has {} lines and {} columns, it should be equal",
                    game.size,
                    squares.len()
                )));
            }
            for (x, square) in squares.iter().enumerate() {
                let point = Point::new(x as u32, y as u32);
                game.add_square(square.to_string(), point)?;
            }
        }
        Ok(game)
    }

    // Add a square to the game
    fn add_square(&mut self, square: String, point: Point) -> Result<(), BombermanError> {
        match square.get(..1) {
            Some(enemy::ENEMY) => {
                let enemy = match Enemy::new(square, point) {
                    Ok(enemy) => enemy,
                    Err(e) => return Err(e),
                };
                self.enemies.push(enemy);
            }
            Some(bomb_type::NORMAL_BOMB | bomb_type::PENETRATING_BOMB) => {
                let bomb = match Bomb::new(square, point) {
                    Ok(bomb) => bomb,
                    Err(e) => return Err(e),
                };
                self.bombs.push(bomb);
            }
            Some(x) if ObstacleType::is_obstacle(x) => {
                let obstacle = match Obstacle::new(square, point) {
                    Ok(obstacle) => obstacle,
                    Err(e) => return Err(e),
                };
                self.obstacles.push(obstacle);
            }
            Some("_") => (),
            _ => {
                return Err(BombermanError::InvalidSquare(format!(
                    "The square {square} at position {point} is invalid"
                )))
            }
        }
        Ok(())
    }

    // Set game for next turn
    //  - Reset enemies state
    fn next_turn(&mut self) {
        self.enemies
            .iter_mut()
            .for_each(|enemy| enemy.reset_state());
    }

    fn get_hittable_in_position(&mut self, position: Point) -> Option<&mut dyn CanBeHit> {
        let mut hittable: Option<&mut dyn CanBeHit> = None;
        if let Some(enemy) = self
            .enemies
            .iter_mut()
            .find(|enemy| enemy.in_position(position))
        {
            hittable = Some(enemy);
        }
        if let Some(bomb) = self
            .bombs
            .iter_mut()
            .find(|bomb| bomb.in_position(position))
        {
            hittable = Some(bomb);
        }
        hittable
    }

    // Plays the game with the given starting bomb
    // Returns the string of the maze after the game or an error
    pub fn play(&mut self, start_bomb: Point) -> Result<String, BombermanError> {
        match self
            .bombs
            .iter_mut()
            .find(|bomb| bomb.in_position(start_bomb))
        {
            Some(bomb) => bomb.hit(),
            None => {
                return Err(BombermanError::NoBombInStartingPosition(format!(
                    "No bomb in starting position: {:?}",
                    start_bomb
                )))
            }
        }

        while let Some(bomb) = self.bombs.iter_mut().find(|bomb| bomb.is_active()) {
            let afected_positions = bomb.explode(self.size, &self.obstacles);
            for position in afected_positions {
                if let Some(hittable) = self.get_hittable_in_position(position) {
                    hittable.hit()
                }
            }
            self.next_turn()
        }

        Ok(self.to_string())
    }

    // Return all the displayable objects
    fn get_all_displayable(&self) -> Vec<&dyn MazeDisplay> {
        let mut displayable: Vec<&dyn MazeDisplay> = Vec::new();
        displayable.extend(self.enemies.iter().map(|enemy| enemy as &dyn MazeDisplay));
        displayable.extend(self.bombs.iter().map(|bomb| bomb as &dyn MazeDisplay));
        displayable.extend(
            self.obstacles
                .iter()
                .map(|obstacle| obstacle as &dyn MazeDisplay),
        );
        displayable
    }

    // Convert game to matrix
    fn to_matrix(&self) -> Vec<Vec<String>> {
        let mut matrix = vec![vec!["_".to_string(); self.size as usize]; self.size as usize];
        let displayable = self.get_all_displayable();
        displayable.iter().for_each(|displayable| {
            let position = displayable.get_position();
            matrix[position.y as usize][position.x as usize] = displayable.display();
        });
        matrix
    }
}

impl Display for Bomberman {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display = String::new();
        let matrix = self.to_matrix();
        for line in matrix {
            display.push_str(&line.join(" "));
            display.push('\n');
        }
        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enemy_is_hit_with_redirection() {
        let input = "_ F2 DL\n_ _ _\n_ _ B8\n".to_string();
        let result = "_ F1 DL\n_ _ _\n_ _ _\n".to_string();
        let mut game = Bomberman::new(input).unwrap();
        let board = game.play(Point::new(2, 2)).unwrap();
        assert_eq!(result, board);
    }

    #[test]
    fn test_enemy_is_not_hit_twice_by_same_bomb() {
        let input = "B5 F2 DL\n _ _ _\n_ _ _\n".to_string();
        let result = "_ F1 DL\n_ _ _\n_ _ _\n".to_string();
        let mut game = Bomberman::new(input).unwrap();
        let board = game.play(Point::new(0, 0)).unwrap();
        assert_eq!(result, board);
    }

    #[test]
    fn test_bomb_explodes_other_bomb() {
        let input = "B5 B2\n_ _\n".to_string();
        let result = "_ _\n_ _\n".to_string();
        let mut game = Bomberman::new(input).unwrap();
        let board = game.play(Point::new(0, 0)).unwrap();
        assert_eq!(result, board);
    }

    #[test]
    fn board_not_square_returns_error() {
        let input = "B5 B2\n_ _ _\n".to_string();
        let result = BombermanError::MazeNotSquare(
            "Maze has 2 lines and 3 columns, it should be equal".to_string(),
        );
        let game = Bomberman::new(input);
        assert_eq!(result, game.unwrap_err());
    }

    #[test]
    fn invalid_square_returns_error() {
        let input = "X B2\n_ _ \n".to_string();
        let result =
            BombermanError::InvalidSquare("The square X at position (0, 0) is invalid".to_string());
        let game = Bomberman::new(input);
        assert_eq!(result, game.unwrap_err());
    }
}
