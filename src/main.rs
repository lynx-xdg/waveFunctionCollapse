use rand::seq::SliceRandom;

trait CellValue where Self: Sized {
    fn get_valid_states(grid: &Vec<Vec<Cell<Self>>>, x: usize, y: usize) -> Vec<Self>;
    fn get_all_states() -> Vec<Self>;
}

struct WaveFunctionCollapse2D<T> {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell<T>>>,
}

enum Cell<T> {
    Discovered(T),
    Undiscovered(Vec<T>)
}

impl<T: CellValue + Clone + std::fmt::Debug> WaveFunctionCollapse2D<T> {
    fn new(width: usize, height: usize) -> WaveFunctionCollapse2D<T> {
        let mut grid:  Vec<Vec<Cell<T>>> = Vec::new();
        let all_states = T::get_all_states();
        for _x in 0..width {
            let mut col: Vec<Cell<T>> = Vec::new();
            for _y in 0..height {
                col.push(Cell::Undiscovered(all_states.clone()));
            }
            grid.push(col);
        }
        let mut s = Self {width, height, grid};
        s.update_cells();
        s
    }
    fn get_all_lowest(&self) -> Vec<[usize; 2]>{
        let mut coords: Vec<[usize; 2]> = Vec::new();
        let mut lowest = f32::MAX;
        for x in 0..self.width {
            for y in 0..self.height {
                match &self.grid[x][y] {
                    Cell::Discovered(_) => {}
                    Cell::Undiscovered(v) => {
                        let entropy = (v.len() as f32).log2();
                        if entropy > lowest {
                            continue;
                        } else if entropy < lowest {
                            lowest = entropy;
                            coords.clear();
                            coords.push([x, y]);
                        } else if entropy == lowest {
                            coords.push([x, y]);
                        }
                    }
                }
            }
        }
        coords
    }
    fn get_random_lowest(&self) -> [usize; 2] {
        let coords = self.get_all_lowest();
        *coords.choose(&mut rand::thread_rng()).unwrap()
    }
    fn update_cells(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.grid[x][y] = match &self.grid[x][y] {
                    Cell::Discovered(v) => {Cell::Discovered(v.clone())}
                    Cell::Undiscovered(_) => {
                        Cell::Undiscovered(T::get_valid_states(&self.grid, x, y))
                    }
                }
            }
        }
    }
    fn collapse_one(&mut self) {
        self.update_cells();
        let coords = self.get_random_lowest();
        let cell = &self.grid[coords[0]][coords[1]];
        match cell {
            Cell::Undiscovered(v) => {
                let picked = v.choose(&mut rand::thread_rng()).unwrap();
                self.grid[coords[0]][coords[1]] = Cell::Discovered(picked.clone());
            }
            _ => {}
        }
        self.update_cells();
    }
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.grid[x][y] {
                    Cell::Undiscovered(v) => {print!(" {:>3.1} ", (v.len() as f32).log2())}
                    Cell::Discovered(v)  => {print!("{:?}", v)}
                }
            }
            println!("");
        }
        println!("");
    }
}

#[derive(PartialEq, Clone)]
#[allow(unused)]
enum Test {
    NONE,
    U,    //0001
    D,    //0010
    DU,   //0011
    L,    //0100
    LU,   //0101
    LD,   //0110
    LDU,  //0111
    R,    //1000
    RU,   //1001
    RD,   //1010
    RDU,  //1011
    RL,   //1100
    RLU,  //1101
    RLD,  //1110
    RLDU, //1111
}

impl std::fmt::Debug for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Test::*;
        write!(f, "{}", match self {
            U    => "╵",
            D    => "╷",
            DU   => "│",
            L    => "╴",
            LU   => "┘",
            LD   => "┐",
            LDU  => "┤",
            R    => "╶",
            RU   => "└",
            RD   => "┌",
            RDU  => "├",
            RL   => "─",
            RLU  => "┴",
            RLD  => "┬",
            RLDU => "┼",
            NONE => " ",
        })
    }
}

#[derive(PartialEq)]
enum TriBool {True, False, Undef}

impl CellValue for Test {
    fn get_valid_states(grid: &Vec<Vec<Cell<Self>>>, x: usize, y: usize) -> Vec<Self> {
        use Test::*;
        use TriBool::*;
        let mut valid_states = Self::get_all_states();
        let left = if x == 0 {
            False
        } else {
            match &grid[x - 1][y] {
                Cell::Discovered(v) => {
                    if vec![R, RU, RD, RDU, RL, RLU, RLD, RLDU].contains(v) {
                        True
                    } else if vec![NONE, U, D, DU, L, LU, LD, LDU].contains(v) {
                        False
                    } else {
                        Undef
                    }
                }
                Cell::Undiscovered(_) => {
                    Undef
                }
            }
        };
        let right = if x == grid.len() - 1 {
            False
        } else {
            match &grid[x + 1][y] {
                Cell::Discovered(v) => {
                    if vec![L, LU, LD, LDU, RL, RLU, RLD, RLDU].contains(v) {
                        True
                    } else if vec![NONE, U, D, DU, R, RU, RD, RDU].contains(v) {
                        False
                    } else {
                        Undef
                    }
                }
                Cell::Undiscovered(_) => {
                    Undef
                }
            }
        };
        let up = if y == 0 {
            False
        } else {
            match &grid[x][y-1] {
                Cell::Discovered(v) => {
                    if vec![D, DU, LD, LDU, RD, RDU, RLD, RLDU].contains(v) {
                        True
                    } else if vec![NONE, U, L, LU, R, RU, RL, RLU].contains(v) {
                        False
                    } else {
                        Undef
                    }
                }
                Cell::Undiscovered(_) => {
                    Undef
                }
            }
        };
        let down = if y == grid[0].len() - 1 {
            False
        } else {
            match &grid[x][y+1] {
                Cell::Discovered(v) => {
                    if vec![U, DU, LU, LDU, RU, RDU, RLU, RLDU].contains(v) {
                        True
                    } else if vec![NONE, D, L, LD, R, RD, RL, RLD].contains(v) {
                        False
                    } else {
                        Undef
                    }
                }
                Cell::Undiscovered(_) => {
                    Undef
                }
            }
        };
        valid_states.retain(|v| {
            if left == True {
                if vec![NONE, U, D, DU, R, RU, RD, RDU].contains(v) {
                    return false;
                }
            } else if left == False {
                if vec![L, LU, LD, LDU, RL, RLU, RLD, RLDU].contains(v) {
                    return false;
                }
            }
            if right == True {
                if vec![NONE, U, D, DU, L, LU, LD, LDU].contains(v) {
                    return false;
                }
            } else if right == False {
                if vec![R, RU, RD, RDU, RL, RLU, RLD, RLDU].contains(v) {
                    return false;
                }
            }
            if up == True {
                if vec![NONE, D, L, LD, R, RD, RL, RLD].contains(v) {
                    return false;
                }
            } else if up == False {
                if vec![U, DU, LU, LDU, RU, RDU, RLU, RLDU].contains(v) {
                    return false;
                }
            }
            if down == True {
                if vec![NONE, U, L, LU, R, RU, RL, RLU].contains(v) {
                    return false;
                }
            } else if down == False {
                if vec![D, DU, LD, LDU, RD, RDU, RLD, RLDU].contains(v) {
                    return false;
                }
            }
            true
        });
        valid_states
    }
    fn get_all_states() -> Vec<Self> {
        use Test::*;
        vec![NONE, U, D, DU, L, LU, LD, LDU, R, RU, RD, RDU, RL, RLU, RLD, RLDU]
    }
}

fn main() {
    let mut wfc = WaveFunctionCollapse2D::<Test>::new(8,8);
    wfc.display();
    for _ in 0..(8 * 8) {
        wfc.collapse_one();
        wfc.display();
    }
}
