use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub matrix: Vec<Vec<bool>>
}

impl Board {
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

    //fn count_alive_around(matrix : i32, y : i32) -> i32 {
    //}

    #[test]
    fn test_count_alive_around() {
    }
}

