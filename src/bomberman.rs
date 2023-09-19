use crate::bomb::Bomb;
use crate::enemy::Enemy;
use crate::obstacle::Obstacle;
use crate::point::Point;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct Bomberman {
    enemies: Vec<Enemy>,
    bombs: Vec<Bomb>,
    obstacles: Vec<Obstacle>,
    size: u32,
}

#[derive(Debug, PartialEq)]
pub(crate) enum BombermanError {
    MazeNotSquare(String),
    InvalidSquare(String),
    NoBombInStartingPosition(String),
}

impl Display for BombermanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BombermanError::MazeNotSquare(e) => write!(f, "MazeNotSquare: {}", e),
            BombermanError::InvalidSquare(e) => write!(f, "InvalidSquare: {}", e),
            BombermanError::NoBombInStartingPosition(e) => {
                write!(f, "NoBombInStartingPosition: {}", e)
            }
        }
    }
}

pub(crate) trait CanBeHit {
    // Hit the object so it changes its state if needed
    fn hit(&mut self);
    // Return the position of the object
    fn in_position(&self, position: Point) -> bool;
}

pub(crate) trait MazeDisplay {
    // Return the string to display
    fn display(&self) -> String;
    // Return the position of the object
    fn get_position(&self) -> Point;
}

impl Bomberman {
    pub(crate) fn new(file_string: String) -> Result<Bomberman, BombermanError> {
        let lines: Vec<&str> = file_string.split('\n').collect();

        let mut game = Bomberman {
            enemies: Vec::new(),
            bombs: Vec::new(),
            obstacles: Vec::new(),
            size: lines.len() as u32,
        };

        for (y, line) in lines.iter().enumerate() {
            let squares: Vec<&str> = line.split(' ').collect();
            if squares.len() != game.size as usize {
                return Err(BombermanError::MazeNotSquare(format!(
                    "Maze has {} lines and {} columns, it should be equal",
                    game.size,
                    squares.len()
                )));
            }
            for (x, square) in squares.iter().enumerate() {
                let point = Point::new(x as u32, y as u32);
                if let Some(e) = game.add_square(square.to_string(), point) {
                    return Err(e);
                }
            }
        }
        Ok(game)
    }

    fn add_square(&mut self, square: String, point: Point) -> Option<BombermanError> {
        let first_char = square.chars().next().unwrap_or('_');

        match first_char {
            'F' => {
                let enemy = match Enemy::new(square, point) {
                    Ok(enemy) => enemy,
                    Err(e) => return Some(e),
                };
                self.enemies.push(enemy);
            }
            'B' | 'S' => {
                let bomb = match Bomb::new(square, point) {
                    Ok(bomb) => bomb,
                    Err(e) => return Some(e),
                };
                self.bombs.push(bomb);
            }
            'R' | 'W' | 'D' => {
                let obstacle = match Obstacle::new(square, point) {
                    Ok(obstacle) => obstacle,
                    Err(e) => return Some(e),
                };
                self.obstacles.push(obstacle);
            }
            '_' => (),
            _ => {
                return Some(BombermanError::InvalidSquare(format!(
                    "The square {} at position ({}, {}) is invalid",
                    square, point.x, point.y
                )))
            }
        }
        None
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
    pub(crate) fn play(&mut self, start_bomb: Point) -> Result<String, BombermanError> {
        match self
            .bombs
            .iter_mut()
            .find(|bomb| bomb.in_position(start_bomb))
        {
            Some(bomb) => bomb.hit(),
            None => {
                return Err(BombermanError::NoBombInStartingPosition(format!(
                    "No bomb in starting position: {}",
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

        Ok(self.display_lines())
    }

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

    // Convert game maze to string
    pub(crate) fn display_lines(&self) -> String {
        let mut display = String::new();
        let matrix = self.to_matrix();
        for line in matrix {
            display.push_str(&line.join(" "));
            display.push('\n');
        }
        display
    }
}
