use rand::prelude::*;



fn main() {

    fn minelayer() -> char {
        let chance = rand::rng().random_range(1..=4);
        if chance == 4 {
            'B'
        } else {
            ' '
        }
    //     'B'

    }
    // let mut bomb = 30;

    let rows = 6;
    let cols = 6;

    let mut map = vec![vec![' '; cols]; rows];

    for i in 0..rows {
        for j in 0..cols{
            map[i][j] = minelayer();
        }
    }

    


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
