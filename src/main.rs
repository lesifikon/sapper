
fn main() {

    #[derive(Debug)]
    struct Cell {
        open: bool,
        bomb: bool,
    }

    impl Cell {
        fn open(&self) -> bool {
            self.open == false
        }
    }

    let cell1 = Cell {
        open: false,
        bomb: true,
    };

    let mut cell2 = Cell {
        open: false,
        bomb: false,
    };

    println!("{:#?}", cell1);
    println!("{:#?}", cell2);

    if cell2.open() {
        cell2.open = true
    }

    println!("{:#?}", cell2);


    let map = [
        ['B', '1', ' '],
        ['1', '1', ' '],
        [' ', ' ', ' '],
    ];

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
