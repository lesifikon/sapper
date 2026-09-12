use rand::prelude::*;

fn main() {

    let rows = 18;
    let cols = 10;

    let mut map = [0u8; 180];
    let mut mine: u8 = 50;

    'onecycle: loop {
        for i in 0..map.len() {
            if map[i] == 9 {
                continue;
            } else {
                if mine > 0 {
                    map[i] = random(&mut mine);
                } else {
                    break 'onecycle;
                }
            }
        }
    }
    // настроил lazygit чтобы не просил каждый раз

    fn random(mine: &mut u8) -> u8 {
        if rand::rng().random_range(0..20) == 0 {
            *mine -= 1;
            9
        } else {
            0
        }
    }

    // println!("{:?}", map);
    let mut left = vec![0];
    let mut right = vec![cols - 1];

    let mut bad_numbers = cols;

    for _j in 2..=rows {
        left.push(bad_numbers);
        right.push(bad_numbers + cols - 1);
        bad_numbers += cols;
    }

    for k in 0..map.len() {
        let i = k as i16;
        if map[i as usize] == 9 {
            for j in ((i - (cols + 1))..=(i - (cols - 1))).chain((i - 1)..=(i + 1)).chain((i + (cols - 1))..=(i + (cols + 1))) {
                if j >= 0 && j <= (map.len() as i16) - 1 && map[j as usize] != 9 {
                    map[j as usize] += 1;
                }
            }

            if left.contains(&i) {
                let y = [i - cols - 1, i - 1, i + cols - 1];

                second_check(&y, &mut map);
            } else if right.contains(&i) {
                let y = [i - cols + 1, i + 1, i + cols + 1];

                second_check(&y, &mut map);
            }
        }
    }

    fn second_check(y: &[i16], map: &mut [u8]) {
        for j in 0..y.len() {
            let z = y[j];

            if z > 0 && z <= (map.len() as i16) - 1 && map[z as usize] != 9 {
                map[z as usize] -= 1;
            }
        }
    }


    // println!("{:?}", map);


    #[derive(Debug)]
    enum Inside {
        Mine,
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

    for i in 0..map.len() {
        let new_cell = Cell {
            open: false,
            inside: if map[i] == 1 {Inside::Mine} else {Inside::Nule},
        };
        cell_map.push(new_cell);
    }

    // println!("{:#?}", cell_map);


    let mut screen = vec![vec![ 0 ; cols as usize]; rows];

    let mut check = 0;

    for i in 0..rows {
        for j in 0..cols{
            screen[i][j as usize] = map[check];
            check += 1;
        }
    }

    // println!("{}", mine);


    let size = screen.len();

    for (i, row) in screen.iter().enumerate() {
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
