fn main() {
    let ban = create_ban();
    show_ban(ban);
    println!("================");
    let stone_array = to_stone_array([0, 1, 2]);
    println!("{}", stone_array.len());
}

fn create_ban() -> [[Stone; 4]; 4] {
    [
        [Stone::None, Stone::None, Stone::None, Stone::None],
        [Stone::None, Stone::None, Stone::None, Stone::None],
        [Stone::None, Stone::None, Stone::None, Stone::None],
        [Stone::None, Stone::None, Stone::None, Stone::None],
    ]
}

fn show_ban(ban: [[Stone; 4]; 4]) {
    ban.iter().for_each(|array| {
        ban.iter().for_each(|x| print!("{}", x));
        println!();
    });
}

fn to_stone_array(param_array: [i8; 3]) -> [Stone; 3] {
//    let mut stone_array: [Stone; 3] = unsafe { std::mem::uninitialized() };
    let mut stone_array: [Stone; 3] = [Stone::None; 3];

    for i in 0..3 {
        stone_array[i] = Stone::of(param_array[i]);
    }
    return stone_array;
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
}

