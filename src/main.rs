use std::collections::VecDeque;

use tokio::time::{sleep, Duration};
use getch::Getch;
use tokio::sync::mpsc;

mod utils;
use utils::utils::*;


#[tokio::main]
async fn main(){
    let stdin = Getch::new();
    let (tx, mut rx) = mpsc::channel::<u8>(1);
    

    tokio::spawn(async move {
        loop {
            tx.send(
                match stdin.getch() {
                    Ok(val) => val,
                    Err(e) => panic!("{e}")
                    
                }
            ).await.expect("Deu ruim no send");
        }
    });

   

    let mut matrix = [[0;WIDTH];HEIGHT];
    let mut snake = Snake{
        x: 10,
        y: 10,
        tail: VecDeque::from([Coord{y: 11, x:10}, Coord{y: 12, x: 10}]),
        direction: Direction::Up
    };

    let mut candy: Vec<Coord> = vec![];

    loop {
        std::process::Command::new("clear").status().unwrap();
        if let Ok(val) = rx.try_recv(){
            match val{
                97 => snake.direction = Direction::Left,
                100 => snake.direction = Direction::Right,
                115 => snake.direction = Direction::Down,
                119 => snake.direction = Direction::Up,
                _ => {}
            };
        }

        match check_round(&mut matrix, &mut snake, &mut candy) {
            Ok(_) => {},
            Err(_) => {println!("you lost"); break;}
        }
        
        snake.run();
        
        update_matrix_snake(&mut matrix, &snake, &mut candy);
        
        print_matrix(&matrix);

        
        let sleep_time = 250 - (200f32 * ((snake.tail.len()-1) as f32/snake.tail.len() as f32)) as u64;

        
        println!("{sleep_time}");
        sleep(Duration::from_millis(sleep_time)).await;


    }
}