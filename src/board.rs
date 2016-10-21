use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub matrix: Vec<Vec<bool>>
}

impl Board {
    /// Creates a new struct `Board`.
    ///
    /// # Examples
    /// ```
    /// let board = Board::new(40, 20);
    /// ```
    pub fn new(width: usize, height: usize) -> Board {
        let matrix = vec![vec![false; width]; height];
        Board { width: width, height: height, matrix: matrix }
    }

    pub fn randomize(&mut self) -> &mut Board {
        let mut rng = rand::thread_rng();
        for i in 0..self.height {
            for j in 0..self.width {
                self.matrix[i][j] = rng.gen::<bool>();
            }
        }
        self
    }

    pub fn next(&self) -> Board {
        let mut result = Board::new(self.width, self.height);

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let alive_count = self.count_alive_around(x as i32, y as i32);
                let is_alive = match (*cell, alive_count) {
                    (_   , 3) => true,
                    (true, 2) => true,
                    (_   , _) => false,
                };
                result.matrix[y][x] = is_alive;
            }
        }
        result
    }

    fn count_alive_around(&self, x: i32, y : i32) -> i32 {
        let mut count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 { continue; }
                let ny = mod_add(self.height as i32, y, i) as usize;
                let nx = mod_add(self.width as i32, x, j) as usize;
                if self.matrix[ny][nx] { count += 1; }
            }
        }
        count
    }
}

fn mod_add(base: i32, a: i32, b: i32) -> i32 {
    let mut res = (a + b) % base;
    if res < 0 { res += base; }
    res
}

#[test]
fn test_mod_add() {
    assert_eq!(mod_add(10, 9, 1), 0);
    assert_eq!(mod_add(10, 0, -1), 9);
    assert_eq!(mod_add(5, 2, 4), 1);
}
