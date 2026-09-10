use rand::prelude::*;

fn main() {

    fn random(bomb: &mut u8) -> u8 {
        if rand::rng().random_range(0..4) == 0 {
            *bomb -= 1;
            1
        } else {
            0
        }
    }

    let mut bomb_map = [0; 36];
    let mut bomb = 6;

    'onecycle: loop {

        for i in 0..bomb_map.len() {
            if bomb_map[i] == 1 {
                continue;
            } else {
                if bomb > 0 {
                    bomb_map[i] = random(&mut bomb);
                } else {
                    break 'onecycle;
                }
            }
        }

    }


    println!("{:?}", bomb_map);
    // let mut bomb = 20; 

    let rows = 6;
    let cols = 6;

    let mut map = vec![vec![' '; cols]; rows];

    
    for i in 0..rows {
        for j in 0..cols{
            map[i][j] = '#';
            // bomb -= 1
        }
    }

    // println!("{}", bomb);


    let size = map.len();

    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print!(" {} ", cell);
            if j < size - 1 {
                print!("|");
            }
        }
        println!();

        if i < size - 1 {
            println!("{}", "--------------------------------------")
        }
    }
}
