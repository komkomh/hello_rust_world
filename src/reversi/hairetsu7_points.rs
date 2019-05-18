const BAN_SIZE: usize = 4;

fn main() {
    let ban = create_ban();
    show_ban(ban);

    let points = get_points(ban, -1);
    points.iter().for_each(|point| println!("{},{}", point.0, point.1));

//    let count = get_reverse_count(ban, -1, (0, 1));
//    println!("{}", count);
//
//    let score = get_score(ban, -1);
//    println!("score = {}", score);
//
//    let score = get_points(ban, -1);
//    println!("score = {}", score);
}

fn create_ban() -> [[i8; 4]; 4] {
    [
        [0, 0, 0, 0],
        [0, 1, -1, 0],
        [0, -1, 1, 0],
        [0, 0, 0, 0],
    ]
}

fn show_ban(ban: [[i8; BAN_SIZE]; BAN_SIZE]) {
    ban.iter().for_each(|array| {
        array.iter().for_each(|stone| {
            let stone_display = match stone {
                -1 => "●",
                1 => "○",
                _ => " "
            };
            print!("|{}", stone_display);
        });
        println!("|")
    });
}

fn get_reverse_count(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone: i8, point: (i8, i8)) -> i8 {
    let point_stone = ban[point.0 as usize][point.1 as usize];
    if point_stone != 0 {
        return 0;
    }
    let reverse_count = (-1..2)
        .map(|y| {
            let line_reverse_count: i8 = (-1..2)
                .map(|x| get_reverse_count_line(ban, stone, point, (y, x), 0))
                .sum();
            return line_reverse_count;
        })
        .sum();
    return reverse_count;
}

fn get_reverse_count_line(ban: [[i8; BAN_SIZE]; BAN_SIZE],
                          stone: i8,
                          point: (i8, i8),
                          direction: (i8, i8),
                          reverse_count: i8) -> i8 {
    // 調査点を生成する
    let y = point.0 + direction.0;
    let x = point.1 + direction.1;

    // 盤の端まで来ていたら
    if y < 0 || y >= BAN_SIZE as i8 || x < 0 || x >= BAN_SIZE as i8 {
        // 反転数0を返却する
        return 0;
    }

    // この場所の石が
    return match ban[y as usize][x as usize] {
        // 無ければ反転数0を返す
        point_stone if (point_stone == 0) => 0,
        // 自石なら反転数を返す
        point_stone if (point_stone == stone) => reverse_count as i8,
        // 他石なら次の場所を読む
        _ => get_reverse_count_line(
            ban, stone, (y, x), direction, reverse_count + 1)
    };
}

// 評価関数
fn get_score(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone_color: i8) -> usize {

    // 自石が無しなら0点とする
    let has_my_color = ban.iter()
        .map(|array| array.iter().any(|&x| x == stone_color))
        .any(|has_my_color_line| has_my_color_line);
    if !has_my_color {
        return 0;
    }

    // 置ける場所の数をscoreとする
    let mut score = (0..BAN_SIZE)
        .map(|y| {
            (0..BAN_SIZE)
                .filter(|&x| {
                    let reverse_count = get_reverse_count(ban, stone_color, (y as i8, x as i8));
                    return reverse_count > 0;
                })
                .count();
        })
        .count();

    // 角を取っていれば100点追加する
    // 左上
    if ban[0][0] == stone_color {
        score += 100;
    }

    // 右上
    if ban[0][BAN_SIZE - 1] == stone_color {
        score += 100;
    }

    // 左下
    if ban[BAN_SIZE - 1][0] == stone_color {
        score += 100;
    }

    // 右下
    if ban[BAN_SIZE - 1][BAN_SIZE - 1] == stone_color {
        score += 100;
    }

    return score;
}

// 置ける場所のリストを取得する
//fn get_points(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone_color: i8) -> Vec<(i8, i8)> {
//    let mut points: Vec<(i8, i8)> = vec![];
//    (0..BAN_SIZE).for_each(|y| {
//        (0..BAN_SIZE).for_each(|x| {
//            let point = (y as i8, x as i8);
//            let count = get_reverse_count(ban, stone_color, point);
//            if count > 0 {
//                points.push(point);
//            }
//        })
//    });
//    return points;
//}

// 置ける場所のリストを取得する
fn get_points(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone_color: i8) -> Vec<(i8, i8)> {
    let points: Vec<(i8, i8)> = (0..BAN_SIZE)
        .flat_map(|y| (0..BAN_SIZE)
            .filter(move |&x| {
                let point = (y as i8, x as i8);
                let count = get_reverse_count(ban, stone_color, point);
                return count > 0;
            })
            .map(move |x| (y as i8, x as i8)))
        .collect();
    return points;
}


// 場所毎の点数を取得する
// -> 置けるか判定する -> ダメならEmpty
// -> x階層潜る
// -> 場所の最大点数を取得する
// -> x階層帰る
// 最大点数を返す