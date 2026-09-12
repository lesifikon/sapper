use rand::prelude::*;

fn main() {
    let mut bomb_map = [0u8; 81];
    let mut bomb: u8 = 8;


    'onecycle: loop {
        for i in 0..bomb_map.len() {
            if bomb_map[i] == 9 {
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
        if rand::rng().random_range(0..20) == 0 {
            *bomb -= 1;
            9
        } else {
            0
        }
    }

    // println!("{:?}", bomb_map);

    for i in 0..bomb_map.len() {
        let j = i as i16;
        match bomb_map[i] {
            0 => (),
            9 => { numbers(j, &mut bomb_map); },
            _ => (),
        };
    }

    fn numbers(place: i16, map: &mut [u8; 81]) {
        for i in ((place - 10)..=(place - 8)).chain((place - 1)..=(place + 1)).chain((place + 8)..=(place + 10)) {
            if i >= 0 && i <= 80 && map[i as usize] != 9{
                map[i as usize] += 1;
            }
        }
    }

    fn ap_bomb(palce: u8, map: &mut [u8; 81]) {
        for i in 1..=3 {
            let new_palce = (palce - (11 - i)) as usize;
            if map[new_palce] == 9 {
                continue;
            } else {
                map[new_palce] = 1;
            }

        }
    }

    // println!("{:?}", bomb_map);


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

    let rows = 9;
    let cols = 9;

    let mut map = vec![vec![ 0 ; cols]; rows];

    let mut check = 0;

    for i in 0..rows {
        for j in 0..cols{
            map[i][j] = bomb_map[check];
            check += 1;
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
