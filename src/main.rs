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
    // настроил lazygit чтобы не просил каждый раз

    fn random(bomb: &mut u8) -> u8 {
        if rand::rng().random_range(0..4) == 0 {
            *bomb -= 1;
            9
        } else {
            0
        }
    }

    println!("{:?}", bomb_map);

    for i in 0..bomb_map.len() {
        if bomb_map[i] == 9 {
            if i == 0 {
                continue;
            } else {
                bomb_map[i-1] = 1;
            }
        }
        // match bomb_map[i] {
        //     9 => println!("dfsdf")
        // }
    }

    #[derive(Debug)]
    enum Inside {
        Bomb,
        Flag,
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

    // println!("{:#?}", cell_map);

    // for i in 0..cell_map.len() {
    //     match cell_map[i].inside {
    //         Inside::Bomb => miner(&i),
    //         Inside::Nule => (),
    //         _ => println!("тебя пока нет"),
    //     }
    // }
    //
    // fn miner(i: &usize) {
    //     println!("{}", i)
    // }

    let rows = 6;
    let cols = 6;

    let mut map = vec![vec![' '; cols]; rows];

    
    for i in 0..rows {
        for j in 0..cols{
            map[i][j] = '#';
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
