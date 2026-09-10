use rand::prelude::*;

fn main() {
    let mut bomb_map = [0; 36];
    let mut bomb: u8 = 6;

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

    fn random(bomb: &mut u8) -> u8 {
        if rand::rng().random_range(0..4) == 0 {
            *bomb -= 1;
            1
        } else {
            0
        }
    }

    println!("{:?}", bomb_map);


    #[derive(Debug)]
    enum Inside {
        Bomb,
        Number(u8),
        Nule,
    }

    #[derive(Debug)]
    struct Cell {
        open: bool,
        inside: Inside,
    }

    let mut cell_map = Vec::new();

    for i in 0..bomb_map.len() {
        let new_cell = Cell {
            open: false,
            inside: if bomb_map[i] == 1 {Inside::Bomb} else {Inside::Nule},
        };
        cell_map.push(new_cell);
    }

    println!("{:#?}", cell_map);

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
