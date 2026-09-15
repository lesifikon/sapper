use rand::prelude::*;
use std::io;

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
        None,
    }

    #[derive(Debug)]
    struct Cell {
        open: bool,
        inside: Inside,
    }

    impl Cell {
        pub fn check_open(&self) -> bool {
            self.open
        }

        pub fn open_cell(&mut self) {
            self.open = true;
        }

        pub fn check_inside(&self) -> char {
            match &self.inside {
                Inside::Mine => '*',
                Inside::None => ' ',
                Inside::Flag => 'F',
                Inside::Number(n) => (*n as u8 + b'0') as char,
            }
        }
    }

    fn open_neighbour(i: i16, cols: i16, map: &mut [Cell], left: &[i16], right: &[i16]) {
        let list_left = [i - cols - 1, i - 1, i + cols - 1];
        let list_right = [i - cols + 1, i + 1, i + cols + 1];
        for j in ((i - (cols + 1))..=(i - (cols - 1))).chain((i - 1)..=(i + 1)).chain((i + (cols - 1))..=(i + (cols + 1))) {
            if j >= 0 && j <= (map.len() as i16) - 1 {
                if left.contains(&i) && list_left.contains(&j) {
                    continue;
                } else if right.contains(&i) && list_right.contains(&j) {
                    continue;
                } else {
                    // println!("{:#?}", map[j as usize]);
                    &mut map[j as usize].open_cell();
                }
            }
        }
    }


    let mut cell_map = Vec::new();

    for i in 0..map.len() {
        // println!("{}", map[i]);
        let new_cell = Cell {
            open: false,
            inside: { 
                match map[i] {
                    9 => Inside::Mine,
                    0 => Inside::None,
                    _ => Inside::Number(map[i]),
                }
            }
        };
        cell_map.push(new_cell);
    }

    // println!("{:#?}", cell_map);

    let mut screen = vec![vec![ ' ' ; cols as usize]; rows];

    loop {
        for _ in 0..=rows {
            let mut check = 0;
            for i in 0..rows {
                for j in 0..cols{
                    screen[i][j as usize] = {
                        let cell = &cell_map[check];
                        if cell.check_open() {
                            cell.check_inside()
                        } else {
                            '#'
                        }
                    };
                    if screen[i][j as usize] == ' ' {
                        let i = check as i16;
                        open_neighbour(i, cols, &mut cell_map, &left, &right);
                    }
                    // screen[i][j as usize] = map[check];
                    check += 1;
                }
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

        let mut guess = String::new();

        println!("Введите номер ячейки которую хотите открыть!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать строку");

        let guess: usize = guess
            .trim()
            .parse()
            .expect("Пожалуйста, введите корректное число!");

        cell_map[guess - 1].open_cell();
    }
}
