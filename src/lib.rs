mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    None = 0,
    Black = 1,
    White = 2,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Horizontal = 0,
    Vertical = 1,
    Diagonal = 2,
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {

    pub fn toggle(&mut self, row: u32, col: u32, color: Cell) {
        self.set_cell(row, col, color.clone());
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn direction_count(&self, color: Cell, direction: Direction ,row: u32, column: u32) -> u32 {
        
        if row <=14 && column <= 14 && color != Cell::None{
            let mut count = 1;

            let cells = self.get_cells();

            if cells[self.get_index(row, column)] == color {
                match direction {
                    Direction::Horizontal => count += self.direction_count(color,direction,row,column+1),
                    Direction::Vertical => count += self.direction_count(color,direction,row+1,column),
                    Direction::Diagonal => count += self.direction_count(color,direction,row+1,column+1),
                }
                count
            }else{
                0
            }
        }else{
            0
        }
    }

    pub fn is_win(&mut self) -> Cell{
        
        for row in 0..self.height{
            for col in 0..self.width {
                
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                if self.direction_count(cell, Direction::Horizontal, row, col) == 5 || 
                   self.direction_count(cell, Direction::Vertical, row, col) == 5 || 
                   self.direction_count(cell, Direction::Diagonal, row, col) == 5 
                {
                    return cell;
                }
            }
        }

        Cell::None
    }

    pub fn new() -> Universe {
        let width = 15;
        let height = 15;

        let cells = (0..width * height)
            .map(|i| {
                Cell::None
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cell(&mut self, row: u32, column: u32, cell: Cell) {
        
        if row <=14 && column <= 14 {
            let index = self.get_index(row, column);
            self.cells[index] = cell;
        }
        
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn universe_get_index() {
        
        let sut = Universe::new();
        
        assert_eq!(0,sut.get_index(0,0));
        assert_eq!(35,sut.get_index(2,5));
        assert_eq!(209,sut.get_index(13,14));

    }

    #[test]
    fn universe_get_cell(){

        let sut = Universe::new();

        let cells = sut.get_cells();

        let index = sut.get_index(10,10);

        assert_eq!(Cell::None, cells[index]);

    }

    #[test]
    fn univsese_set_cell(){

        let mut sut = Universe::new();

        sut.set_cell(10,10,Cell::White);

        let cells = sut.get_cells();

        assert_eq!(Cell::White, cells[sut.get_index(10,10)]);

    }

    #[test]
    fn universe_direction_count_horizontal(){

        let mut sut = Universe::new();

        sut.set_cell(10,5,Cell::Black);
        sut.set_cell(10,6,Cell::Black);
        sut.set_cell(10,7,Cell::Black);

        assert_eq!(0,sut.direction_count(Cell::White, Direction::Horizontal , 10, 5));
        assert_eq!(3,sut.direction_count(Cell::Black, Direction::Horizontal , 10, 5));
        
    }

    #[test]
    fn universe_direction_count_out_of_bound(){

        let mut sut = Universe::new();

        sut.set_cell(14,14,Cell::Black);
        
        assert_eq!(1,sut.direction_count(Cell::Black, Direction::Horizontal , 14, 14));
    }


    #[test]
    fn universe_direction_count_vertical(){

        let mut sut = Universe::new();

        sut.set_cell(9,5,Cell::Black);
        sut.set_cell(10,5,Cell::Black);
        sut.set_cell(11,5,Cell::Black);
        sut.set_cell(12,5,Cell::Black);
        sut.set_cell(13,5,Cell::Black);
        

        assert_eq!(5,sut.direction_count(Cell::Black, Direction::Vertical , 9, 5));

    }

    #[test]
    fn universe_direction_count_diagonal(){

        let mut sut = Universe::new();

        sut.set_cell(5,5,Cell::Black);
        sut.set_cell(6,6,Cell::Black);
        sut.set_cell(7,7,Cell::Black);
        sut.set_cell(8,8,Cell::Black);
        sut.set_cell(9,9,Cell::Black);

        assert_eq!(5,sut.direction_count(Cell::Black, Direction::Diagonal , 5, 5));

    }

    #[test]
    fn universe_direction_count_not_none(){

        let sut = Universe::new();

        assert_eq!(0,sut.direction_count(Cell::None, Direction::Diagonal , 5, 5));

    }

    #[test]
    fn universe_not_win(){

        let mut sut = Universe::new();

        assert_eq!(Cell::None, sut.is_win());
    }

    #[test]
    fn universe_black_win(){

        let mut sut = Universe::new();

        sut.set_cell(5,5,Cell::Black);
        sut.set_cell(6,6,Cell::Black);
        sut.set_cell(7,7,Cell::Black);
        sut.set_cell(8,8,Cell::Black);
        sut.set_cell(9,9,Cell::Black);
        
        assert_eq!(Cell::Black, sut.is_win());
        
    }

    #[test]
    fn universe_white_win(){

        let mut sut = Universe::new();

        sut.set_cell(5,5,Cell::White);
        sut.set_cell(5,6,Cell::White);
        sut.set_cell(5,7,Cell::White);
        sut.set_cell(5,8,Cell::White);
        sut.set_cell(5,9,Cell::White);
        
        assert_eq!(Cell::White, sut.is_win());

    }    

    #[test]
    fn toggle_cell(){

        let mut sut = Universe::new();

        sut.toggle(4, 4, Cell::Black);
        
        let cells = sut.get_cells();

        assert_eq!(Cell::Black, cells[sut.get_index(4, 4)]);
    }

}