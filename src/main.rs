use rand::prelude::*;
use std::io::{self, Write};

fn main() {

    println!("Спасибо что отрыли мою игру!!! Управление здесь странное )). Чтобы выбрать ячейку нужно ввести номер верхнего списка от 1 до 9 и номер левого списка от 1 до 9. После этого вы должны выбрать 0 или 1. 0 открывает ячейку, 1 ставит флажок. Вот пример: (3 4 0). Обязательно пробел после кадой цыфры.");
    println!();
    println!("Хорошей игры!");

    let rows = 9;
    let cols = 9;

    let mut map = [0u8; 81];
    let mut mine: u8 = 10;

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

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum Inside {
        Mine,
        Number(u8),
        None,
    }

    #[derive(Debug)]
    struct Cell {
        open: bool,
        flag: bool,
        inside: Inside,
    }

    impl Cell {
        pub fn check_open(&self) -> bool {
            self.open
        }

        pub fn check_flag(&self) -> bool {
            self.flag
        }

        pub fn mine(&self) -> Inside {
            self.inside
        }

        pub fn open_cell(&mut self) {
            self.open = true;
        }

        pub fn put_flag(&mut self) {
            self.flag = true;
        }

        pub fn put_away_flag(&mut self) {
            self.flag = false;
        }

        pub fn check_inside(&self) -> char {
            match &self.inside {
                Inside::Mine => '*',
                Inside::None => ' ',
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
                    let _ = &mut map[j as usize].open_cell();
                }
            }
        }
    }


    let mut cell_map = Vec::new();

    for i in 0..map.len() {
        // println!("{}", map[i]);
        let new_cell = Cell {
            open: false,
            flag: false,
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

    let mut screen = vec![vec![ ' ' ; cols as usize]; rows];

    let mut over = 0;

    'game_over: loop {
        for _ in 0..=rows {
            let mut check = 0;
            for i in 0..rows {
                for j in 0..cols{
                    screen[i][j as usize] = {
                        let cell = &cell_map[check];
                        if cell.check_flag() {
                            'F'
                        } else if cell.check_open() {
                            cell.check_inside()
                        } else {
                            '#'
                        }
                    };
                    if screen[i][j as usize] == ' ' {
                        let i = check as i16;
                        open_neighbour(i, cols, &mut cell_map, &left, &right);
                    }
                    // let cell = &cell_map[check];
                    // if cell.check_open() && cell.mine() == Inside::Mine {
                    //     break 'game_over;
                    // }
                    check += 1;
                }
            }
        }

        let mut number_cols: Vec<u8> = Vec::new();
        let mut number_rows: Vec<u8> = Vec::new();

        list(&mut number_cols, cols);
        list(&mut number_rows, rows as i16);

        fn list(list: &mut Vec<u8>, length: i16) {
            for i in 0..=length {
                list.push(i as u8)
            }
        }


        let size = screen.len();

        for i in 0..number_cols.len() {
            print!(" {} ", i);
        }
        println!();

        for (i, row) in screen.iter().enumerate() {
            print!(" {} ", i + 1);
            for (_j, cell) in row.iter().enumerate() {
                print!(" {} ", cell);
            }
            println!();

            if i < size - 1 {
            }
        }

        // проверка на победу или проигрыш
        let mut min_found = 0;
        for i in 0..map.len() {
            let cell = &cell_map[i];
            if cell.check_open() && cell.mine() == Inside::Mine {
                over += 1;
            } else if cell.check_flag() && cell.mine() == Inside::Mine {
                min_found += 1;
                if min_found == 10 {
                    break 'game_over
                }
            } else if cell.check_flag() && cell.mine() != Inside::Mine {
                min_found += 11;
            }
        }

        if over == 1 {
            for i in 0..map.len() {
                let cell = &mut cell_map[i];
                if cell.mine() == Inside::Mine {
                    cell.open_cell()
                }
            }
            continue;
        } else if over > 1{
            break 'game_over
        }

        loop {
            let mut guess = String::new();

            println!("Введите номер ячейки которую хотите открыть!");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut guess)
                .expect("Не удалось прочитать строку");

            let parts: Vec<&str> = guess.trim().split_whitespace().collect();

            if parts.len() != 3 {
                println!("ты неправильно ввел");
                continue;
            }

            let number_cols: usize = match parts[0].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("неправильно, попробуй занова");
                    continue;
                }
            };

            let number_rows: usize = match parts[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("неправильно, попробуй занова");
                    continue;
                }
            };

            let number_action: usize = match parts[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("неправильно, попробуй занова");
                    continue;
                }
            };

            let number = (number_cols - 1) + (left[(number_rows - 1) as usize] as usize);

            if number_action == 0 {
                cell_map[number].open_cell()
            } else if cell_map[number].check_open() == false {
                if cell_map[number].check_flag() {
                    cell_map[number].put_away_flag()
                } else {
                    cell_map[number].put_flag()
                }
            }
            break;
        }
    }
    println!("Игра окончена!")
}
