mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt)* ) => {
        console::log_1(&format!( $( $t ) *).into());
    };
}
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
    Dead = 0,
    Alive = 1,
}

impl fmt::Display for Cell{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let symbol = if self == &Cell::Dead{
            '◻'
        } else{
            '◼'
        };
        write!(f, "{}", symbol)?;
        Ok(())
    }
}

impl Cell{
    fn toggle(&mut self){
        *self = match *self{
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    old_cells: Vec<Cell>,
    cells: Vec<Cell>,
    delta: Vec<u32>
}

impl fmt::Display for Universe{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for line in self.cells.as_slice().chunks(self.width as usize){
            for &cell in line{
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Universe{
    fn get_index(&self, row: u32, column: u32) -> usize{
        (row * self.width + column) as usize
    }

    fn index(width: u32, row: u32, column: u32) -> u32{
        row * width + column
       }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8{
        let mut count = 0;

        let north = if row == 0{
            self.height - 1
        }else{
            row - 1
        };

        let south = if row == self.height - 1{
            0
        }else{
            row + 1
        };

        let west = if column == 0{
            self.width - 1
        }else{
            column - 1
        };

        let east = if column == self.width - 1{
            0
        }else{
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.old_cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.old_cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.old_cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.old_cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.old_cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.old_cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.old_cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.old_cells[se] as u8;

        count
    }

    pub fn get_cells(&self) -> &[Cell]{
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]){
        for (row, col) in cells.iter().cloned(){
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe{
    pub fn tick(&mut self){
        let _timer = Timer::new("tick");
        self.old_cells = self.cells.clone();
        self.delta.clear();

        for row in 0..self.height{
            for col in 0..self.width{
                let idx = self.get_index(row, col);
                let cell = self.old_cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors){
                    (Cell::Alive, x) if x < 2 => {
                        log!("Cell[{row}, {col}] died by loneliness.");
                        self.delta.push(idx as u32);
                        Cell::Dead
                    },
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => {
                        log!("Cell[{row}, {col}] died by overcrowding.");
                        self.delta.push(idx as u32);
                        Cell::Dead
                    },
                    (Cell::Dead, 3) => {
                        log!("Cell[{row}, {col}] was born.");
                        self.delta.push(idx as u32);
                        Cell::Alive
                    },
                    (otherwise, _) => otherwise, 
                };
                

                self.cells[idx] = next_cell;
            }
        }
    }

    pub fn new() -> Universe{
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|i|{
            if i == Universe::index(width, 2, 3)
            || i == Universe::index(width, 3, 3)
            || i == Universe::index(width, 4, 3)
            || i == Universe::index(width, 4, 2)
            || i == Universe::index(width, 3, 1){
                Cell::Alive
            }else{
                Cell::Dead
            }
        }).collect();

        let old_cells = (0..width * height).map(|_|Cell::Dead).collect();
        let delta = (0..width * height).collect();
            
        Universe {width, height, cells, old_cells, delta}
        
    }

    pub fn render(&self) -> String{
        self.to_string()
    }

    pub fn width(&self) -> u32{
        self.width
    }

    pub fn height(&self) -> u32{
        self.height
    }

    pub fn cells(&self) -> *const Cell{
        self.cells.as_ptr()
    }

    pub fn delta(&self) -> Vec<u32>{
        self.delta.clone()
    }
    
    pub fn clear_delta(&mut self){
        self.delta.clear();
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
        self.delta = (0..width * self.height).collect();
    }

    pub fn set_height(&mut self, height: u32){
        self.height = height;
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
        self.delta = (0..self.width * self.height).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
        self.delta.push(idx as u32);
    }

    pub fn clear(&mut self){
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
        self.delta = (0..self.width * self.height).collect();
    }

    pub fn random(&mut self){

        self.cells = (0..self.width * self.height).map(|_| {
            if js_sys::Math::random() < 0.2{
                Cell::Alive
            }else{
                Cell::Dead
            }
        }).collect();
        self.delta = (0..self.width * self.height).collect();
    }

    pub fn check_cell(&self, idx:usize) -> Cell{
        self.cells[idx]
    }
}

pub struct Timer<'a>{
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a>{
        console::time_with_label(name);
        Timer{name}
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self){
        console::time_end_with_label(self.name);
    }
}
