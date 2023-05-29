pub mod utils {
    
use std::collections::VecDeque;
use rand::prelude::*;

pub const HEIGHT: usize = 40;
pub const WIDTH: usize = 40;
const SQUARE: &str = "⬜";
const BLANK: &str = "  ";
const CANDY: &str = "🍎";

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right, 
    Up, 
    Down
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub y: usize,
    pub x: usize
}


#[derive(Debug)]
pub struct Snake{
    pub y: usize,
    pub x: usize,
    pub tail: VecDeque<Coord>,
    pub direction: Direction
}

impl Snake {

    pub fn run(&mut self){
        self.tail.push_front(Coord { y: self.y, x: self.x });
        self.tail.pop_back();

        match self.direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1
        };
    }

    pub fn add_size(&mut self){
        self.tail.push_back(
            match self.direction {
                Direction::Left => Coord{y: self.y, x: self.x + 1},
                Direction::Right => Coord{y: self.y, x: self.x - 1},
                Direction::Up => Coord{y: self.y + 1, x: self.x},
                Direction::Down => Coord{y: self.y - 1, x: self.x}
        });
    }

    
}



pub fn print_matrix(matrix: &[[i32;WIDTH];HEIGHT]) {
    for i in matrix.iter(){
        for j in i.iter(){
            print!("{}", if *j > 0 && *j < 4 {SQUARE} else {if *j >= 4 {CANDY} else {BLANK}})
        }
        println!();
    }
}


pub fn update_matrix_snake(matrix: &mut [[i32;WIDTH];HEIGHT], snake: &Snake, candy: &mut Vec<Coord>){
    
    for i in 0..HEIGHT{
        for j in 0..WIDTH{
            matrix[i][j] = 0;
        }
    }
    
    for Coord { y, x } in candy.iter(){
        matrix[*y][*x] = 4;
    }

    matrix[snake.y][snake.x] = 1;
    for Coord{y, x} in  snake.tail.iter(){
        matrix[*y][*x] += 1;
    }

    for i in 1..HEIGHT-1{
        matrix[i][0] += 1;
        matrix[i][WIDTH-1] += 1;
    }

    for i in 0..WIDTH{
        matrix[0][i] += 1;
        matrix[HEIGHT-1][i] += 1;
        
    }


    while candy.len() < 5{
        candy.push(Coord {y: rand::thread_rng().gen_range(1..HEIGHT-1), x: rand::thread_rng().gen_range(1..WIDTH-1)})
    }


}



pub fn check_round(matrix: &mut [[i32;WIDTH];HEIGHT], snake: &mut Snake, candy: &mut Vec<Coord>) -> Result<(), ()>{

    for (i, Coord { y, x }) in candy.iter().enumerate(){
        if *y == snake.y && *x == snake.x{
            snake.add_size();
            candy.remove(i);
            break;
        }
    }


    for i in matrix.iter(){
        for j in i.iter(){
            if *j == 2{
                return Err(());
            }
        }
    }

    return Ok(())
}



}