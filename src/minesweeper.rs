use crate::gen_range;

pub struct Minesweeper {
    pub width : usize,
    pub height : usize,
    pub mines : usize,
    pub grid : Vec<Vec<(i32, bool, bool)>>
}

impl Minesweeper {
    pub fn new(width : usize, height : usize, mines : usize) -> Self {
        Self {
            width,
            height,
            mines,
            grid : Vec::new()
        }
    }

    pub fn init_grid(&mut self) {
        self.grid = vec![vec![(0, false, false); self.width]; self.height];
    }
    
    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].1 == true {
                    if self.grid[y][x].0 < 0 {
                        print!("[-] ");
                    } else {
                        print!("[{}] ", self.grid[y][x].0);
                    }
                } else if self.grid[y][x].2 == true {
                    print!("[f] ");
                } else {
                    print!("[ ] ");
                }
            }
            println!();
        }
        println!();
    }
    
    pub fn generate_grid(&mut self) {
        self.init_grid();
        for _ in 0..self.mines {
            let mut mine_x = gen_range(0, (self.width - 1) as u32).expect("rng failed") as usize;
            let mut mine_y = gen_range(0, (self.height -1) as u32).expect("rng failed") as usize;

            while self.grid[mine_y][mine_x].0 == -1 {
                mine_x = gen_range(0, (self.width - 1) as u32).expect("rng failed") as usize;
                mine_y = gen_range(0, (self.height -1) as u32).expect("rng failed") as usize;
            }

            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    let nx = mine_x as i32 + dx;
                    let ny = mine_y as i32 + dy;

                    if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                        continue; // skip out-of-bounds
                    }

                    let (nx, ny) = (nx as usize, ny as usize);

                    if dx == 0 && dy == 0 {
                        self.grid[ny][nx].0 = -1; // mine
                    } else if self.grid[ny][nx].0 != -1 {
                        self.grid[ny][nx].0 += 1; // neighbor count
                    }
                }
            }
        }
    }
    
    pub fn check_win(&self) -> bool {
        let mut correct : usize = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].0 < 0 && self.grid[y][x].2 == true {
                    correct += 1;
                }
            }
        }
        correct == self.mines
    }
}