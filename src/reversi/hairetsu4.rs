fn main() {
    let ban = create_ban();
    show_ban(ban);

    let xx: isize = 0;
}

fn create_ban() -> [[Option<Stone>; 4]; 4] {
    let num_ban =
        [
            [0, 0, 0, 0],
            [0, -1, 1, 0],
            [0, 1, -1, 0],
            [0, 0, 0, 0],
        ];
    to_stone_array(num_ban)
}

fn to_stone_array(num_ban: [[i8; 4]; 4]) -> [[Option<Stone>; 4]; 4] {
    let mut stone_array: [[Option<Stone>; 4]; 4] = [[Option::None; 4]; 4];
    for (i, array) in num_ban.iter().enumerate() {
        for (j, num_stone) in array.iter().enumerate() {
            stone_array[i][j] = Stone::of(*num_stone);
        }
    }
    return stone_array;
}

fn show_ban(ban: [[Option<Stone>; 4]; 4]) {
    ban.iter().for_each(|array| {
        array.iter().for_each(|stone| {
            let show = stone.map_or(" ", |stone| stone.to_string());
            print!("|{}", show);
        });
        println!("|");
    });
}

#[derive(Copy, Clone)]
enum Stone {
    Black,
    White,
}

impl Stone {
    fn of(v: i8) -> Option<Stone> {
        match v {
            1 => Some(Stone::Black),
            -1 => Some(Stone::White),
            _ => None,
        }
    }

    fn to_string(self: Stone) -> &'static str {
        match self {
            Stone::Black => "●",
            Stone::White => "○",
        }
    }
}

