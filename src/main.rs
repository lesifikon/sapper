use rand::prelude::*;

fn main() {

    let rows = 9;
    let cols = 9;

    let mut map = [0u8; 81];
    let mut mine: u8 = 8;

    let mut left = vec![0];
    let mut right = vec![cols - 1];

    let mut bad_numbers = cols;

    for _j in 2..=rows {
        left.push(bad_numbers);
        right.push(bad_numbers + cols - 1);
        bad_numbers += cols;
    }

    // генерируються мины и цифры вокруг мин
    'onecycle: loop {
        for k in 0..map.len() {
            if map[k] == 9 {
                continue;
            } else {
                if mine > 0 {
                    map[k] = {
                        if rand::rng().random_range(0..20) == 0 {
                            mine -= 1;
                            9
                        } else {
                            continue;
                        }
                    };
                    let i = k as i16;
                    let list_left = [i - cols - 1, i - 1, i + cols - 1];
                    let list_right = [i - cols + 1, i + 1, i + cols + 1];
                    for j in ((i - (cols + 1))..=(i - (cols - 1))).chain((i - 1)..=(i + 1)).chain((i + (cols - 1))..=(i + (cols + 1))) {
                        if j >= 0 && j <= (map.len() as i16) - 1 && map[j as usize] != 9 {
                            if left.contains(&i) && list_left.contains(&j) {
                                continue;
                            } else if right.contains(&i) && list_right.contains(&j) {
                                continue;
                            } else {
                                map[j as usize] += 1;
                            }
                        }
                    }
                } else {
                    break 'onecycle;
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
