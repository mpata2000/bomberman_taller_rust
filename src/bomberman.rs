use crate::bomb::Bomb;
use crate::enemy::Enemy;
use crate::obstacle::Obstacle;
use crate::point::Point;

#[derive(Debug)]
pub(crate) struct Bomberman {
    enemies: Vec<Enemy>,
    bombs: Vec<Bomb>,
    obstacles: Vec<Obstacle>,
    size: u32,
}

pub(crate) trait CanBeHit {
    fn hit(&mut self);
    fn is_in_position(&self, position: Point) -> bool;
}

pub(crate) trait MazeDisplay {
    fn display(&self) -> String;
    fn get_position(&self) -> Point;
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
            size: lines.len() as u32,
        };

        for (y, line) in lines.iter().enumerate() {
            let squares: Vec<&str> = line.split(" ").collect();
            if squares.len() != game.size as usize {
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
            'R' | 'W' | 'D' => {
                let obstacle = match Obstacle::new(square, point) {
                    Ok(obstacle) => obstacle,
                    Err(e) => return Err(e),
                };
                self.obstacles.push(obstacle);
            }
            '_' => (),
            _ => return Err(format!("Invalid square: {}", square)),
        }
        Ok(())
    }

    fn there_are_active_bombs(&mut self) -> bool {
        self.bombs.iter().any(|bomb| bomb.is_active())
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
        if let Some(enemy) = self.enemies.iter_mut().find(|enemy| enemy.is_in_position(position)) {
            hittable = Some(enemy);
        }
        if let Some(bomb) = self.bombs.iter_mut().find(|bomb| bomb.is_in_position(position)) {
            hittable = Some(bomb);
        }
        hittable
    }

    pub(crate) fn play(&mut self, start_bomb: Point) -> Result<Vec<Vec<String>>, String> {
        let first_bomb = self
            .bombs
            .iter_mut()
            .find(|bomb| bomb.is_in_position(start_bomb));
        match first_bomb {
            Some(bomb) => bomb.hit(),
            None => return Err("No bomb in starting position x: {}, y: {}".to_string()),
        }

        while let Some(bomb) = self.bombs.iter_mut().find(|bomb| bomb.is_active()) {
            let afected_positions = bomb.explode(&self.obstacles);
            for position in afected_positions {
                match self.get_hittable_in_position(position) {
                    Some(hittable) => (*hittable).hit(),
                    None => (),
                }
            }
            self.next_turn()
        }

        Ok(self.to_matrix())
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

    pub(crate) fn to_matrix(&self) -> Vec<Vec<String>> {
        let mut matrix = vec![vec!["_".to_string(); self.size as usize]; self.size as usize];
        let displayable = self.get_all_displayable();
        displayable.iter().for_each(|displayable| {
            let position = displayable.get_position();
            matrix[position.y as usize][position.x as usize] = displayable.display();
        });
        matrix
    }
}
