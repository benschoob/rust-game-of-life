pub struct Life {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<bool>,
}

impl Life {
    pub fn new(w: usize, h: usize) -> Self {
        Life {
            width: w,
            height: h,
            grid: vec![false; w * h],
        }
    }

    pub fn cell_at(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false; // Dead Edge
        }
        let i = x + (y * self.width);
        return self.grid[i];
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<bool> {
        if x == 0 && y == 0 {
            return vec![
                false                     , false                     , false                     , 
                false                     ,     /* Current Cell */      self.cell_at(x + 1, y    ), 
                false                     , self.cell_at(x    , y + 1), self.cell_at(x + 1, y + 1), 
            ]
        }
        else if x == 0 {
            return vec![
                false                     , self.cell_at(x    , y - 1), self.cell_at(x + 1, y - 1), 
                false                     ,     /* Current Cell */      self.cell_at(x + 1, y    ), 
                false                     , self.cell_at(x    , y + 1), self.cell_at(x + 1, y + 1), 
            ]
        }
        else if y == 0 {
            return vec![
                false                     , false                     , false                     , 
                self.cell_at(x - 1, y    ),     /* Current Cell */      self.cell_at(x + 1, y    ), 
                self.cell_at(x - 1, y + 1), self.cell_at(x    , y + 1), self.cell_at(x + 1, y + 1), 
            ]
        }
        else {
            return vec![
                self.cell_at(x - 1, y - 1), self.cell_at(x    , y - 1), self.cell_at(x + 1, y - 1), 
                self.cell_at(x - 1, y    ),     /* Current Cell */      self.cell_at(x + 1, y    ), 
                self.cell_at(x - 1, y + 1), self.cell_at(x    , y + 1), self.cell_at(x + 1, y + 1), 
            ]
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, val: bool) {
        let i: usize = x + (y * self.width);
        self.grid[i] = val;
    }

    pub fn clear(&mut self) {
        for i in 0..(self.width * self.height) {
            self.grid[i] = false;
        }
    }

    pub fn update(&mut self) {
        let mut new_grid: Vec<bool> = vec![false; self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let mut true_cells: usize = 0;
                for b in self.neighbors(x, y) {
                    if b { true_cells += 1; }
                }

                // Set the value of the cell in the new grid according to the rules of coway's game of life
                // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

                // Live Cell
                if self.cell_at(x, y) {
                    // Any live cell with fewer than two live neighbours dies, as if by underpopulation
                    if true_cells < 2 {
                        new_grid[x + (y * self.width)] = false;
                    }
                    // Any live cell with more than three live neighbours dies, as if by overpopulation
                    else if true_cells > 3 {
                        new_grid[x + (y * self.width)] = false;
                    }
                    // Any live cell with two or three live neighbours lives on to the next generation
                    else {
                        new_grid[x + (y * self.width)] = true;
                    }
                }
                //Dead Cell
                else {
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
                    if true_cells == 3 {
                        new_grid[x + (y * self.width)] = true;
                    }
                }
            }
        }
        // Replace the grid with the new, completed grid
        self.grid = new_grid;
    }
}