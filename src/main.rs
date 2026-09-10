use rand::prelude::*;



fn main() {

    // fn minelayer(bomb: i8) -> char {
    //     let chance = rand::rng().random_range(1..=4);
    //     println!("{}", bomb);
    //     if chance == 4 && bomb > 0{
    //         'B'
    //     } else {
    //         ' '
    //     }
    //
    // }

    fn minelayer() -> char {
        let chance = rand::rng().random_range(1..=4);
        // println!("{}", bomb);
        if chance == 4 {
            'B'
        } else {
            ' '
        }

    }

    // let mut bomb = 20;

    let rows = 6;
    let cols = 6;

    let mut map = vec![vec![' '; cols]; rows];

    
    for i in 0..rows {
        for j in 0..cols{
            map[i][j] = minelayer();
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
