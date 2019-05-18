fn main() {
    let ban = create_ban();
    show_ban(ban);
}

fn create_ban() -> [[Stone; 4]; 4] {
    let num_ban =
        [
            [0, 0, 0, 0],
            [0, 2, 1, 0],
            [0, 1, 2, 0],
            [0, 0, 0, 0],
        ];
    to_stone_array(num_ban)
}

fn to_stone_array(num_ban: [[i8; 4]; 4]) -> [[Stone; 4]; 4] {
    let mut stone_array: [[Stone; 4]; 4] = [[Stone::None; 4]; 4];
    for (i, array) in num_ban.iter().enumerate() {
        for (j, num_stone) in array.iter().enumerate() {
            stone_array[i][j] = Stone::of(*num_stone);
        }
    }
    return stone_array;
}

fn show_ban(ban: [[Stone; 4]; 4]) {
    ban.iter().for_each(|array| {
        array.iter().for_each(|x| {
            print!("|{}", &x.name());
        });
        println!("|");
    });
}

#[derive(Copy, Clone)]
enum Stone {
    None,
    Black,
    White,
}

impl Stone {
    fn value(self: Stone) -> i8 {
        self as i8
    }

    fn of(v: i8) -> Stone {
        match v {
            v if Stone::None.value() == v => Stone::None,
            v if Stone::Black.value() == v => Stone::Black,
            v if Stone::White.value() == v => Stone::White,
            _ => Stone::None,
        }
    }

    fn name(self: Stone) -> &'static str {
        match self {
            Stone::None => " ",
            Stone::Black => "●",
            Stone::White => "○",
        }
    }
}

