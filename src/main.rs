use rand::prelude::*;

fn main() {

    let rows = 9;
    let cols = 9;

    let mut map = [0u8; 81];
    let mut mine: u8 = 8;

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

    for i in 0..map.len() {
        if map[i] == 9 {
            numbers((i as i8).into(),rows, &mut map);
        }
    }

    fn numbers(place: i16, row: i16, map: &mut [u8]) {
        let mut left = vec![0];
        let mut right = vec![row - 1];


        let x = (map.len() as i16) / row;
        let mut bad_numbers = row;
        for j in 2..=row {
            left.push(bad_numbers);
            right.push(bad_numbers + row - 1);
            bad_numbers += row;
        }

        for i in ((place - (row + 1))..=(place - (row - 1))).chain((place - 1)..=(place + 1)).chain((place + (row - 1))..=(place + (row + 1))) {
            if i >= 0 && i <= (map.len() as i16) - 1 && map[i as usize] != 9 {
                map[i as usize] += 1;
            }
        }

        if left.contains(&place) {
            let y = [place - row - 1, place - 1, place + row - 1];
            for i in 0..y.len() {
                let z = y[i];
                if z > 0 && map[z as usize] != 9 {
                    map[z as usize] -= 1;
                }
            }
        } else if right.contains(&place) {
            let y = [place - row + 1, place + 1, place + row + 1];
            for i in 0..y.len() {
                let z = y[i];
                if z > 0 && map[z as usize] != 9 {
                    map[z as usize] -= 1;
                }
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


    let mut screen = vec![vec![ 0 ; cols]; rows as usize];

    let mut check = 0;

    for i in 0..rows {
        for j in 0..cols{
            screen[i as usize][j] = map[check];
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
