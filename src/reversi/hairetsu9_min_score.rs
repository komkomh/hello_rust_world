const BAN_SIZE: usize = 6;
const SEARCH_DEPTH: i8 = 5;

fn main() {
    let ban = create_ban();
    show_ban(ban);
//
//    let points = get_points(ban, -1);
//    points.iter().for_each(|point| println!("{},{}", point.0, point.1));

    let new_ban = put_stone(ban, -1, (1, 2));
    show_ban(new_ban);

    let next_point = get_next_point(new_ban, 1);
    next_point.map(|next_point| {
        println!("{},{}", next_point.0, next_point.1);
    });
}

fn create_ban() -> [[i8; 6]; 6] {
    [
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 1, -1, 0, 0],
        [0, 0, -1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
    ]
}

fn show_ban(ban: [[i8; BAN_SIZE]; BAN_SIZE]) {
    println!("---------------");
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

// 石を置いた後の盤を取得する
fn put_stone(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone: i8, point: (i8, i8)) -> [[i8; BAN_SIZE]; BAN_SIZE] {
    let point_stone = ban[point.0 as usize][point.1 as usize];
    if point_stone != 0 {
        return ban;
    }

    let points =
        (-1..2).flat_map(|y| {
            (-1..2)
                .map(move |x| {
                    get_reverse_points(ban, stone, point, (y, x))
                })
                .filter(|p| p.is_some())
                .flat_map(|p| p.unwrap())
        });

    // 新しい盤を生成する
    let mut new_ban: [[i8; BAN_SIZE]; BAN_SIZE] = ban.clone();
    // 盤に石を置く
    new_ban[point.0 as usize][point.1 as usize] = stone;
    // 盤上の石を裏返す
    points.for_each(|reversed_point| {
        new_ban[reversed_point.0 as usize][reversed_point.1 as usize] = stone;
    });
    return new_ban;
}

// 一行反転する
fn get_reverse_points(ban: [[i8; BAN_SIZE]; BAN_SIZE],
                      stone: i8,
                      point: (i8, i8),
                      direction: (i8, i8)) -> Option<Vec<(i8, i8)>> {
    // 調査点を生成する
    let y = point.0 + direction.0;
    let x = point.1 + direction.1;

    // 盤の端まで来ていたら
    if y < 0 || y >= BAN_SIZE as i8 || x < 0 || x >= BAN_SIZE as i8 {
        // 反転数0を返却する
        return Option::None;
    }

    // この場所の石が
    return match ban[y as usize][x as usize] {
        // 無ければ反転数0を返す
        point_stone if (point_stone == 0) => Option::None,
        // 自石なら反転数を返す
        point_stone if (point_stone == stone) => Option::Some(Vec::new()),
        // 他石なら次の場所を読む
        _ => {
            // ひっくり返せるかを取得する
            get_reverse_points(ban, stone, (y, x), direction)
                .map(|points| {
                    let mut new_points = points;
                    let point = (y, x);
                    new_points.push(point);
                    return new_points;
                })
        }
    };
}

// x手先を読む
fn get_next_point(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone_color: i8) -> Option<(i8, i8)> {
    let points: Vec<(i8, i8)> = get_points(ban, stone_color);
    let point_score = points.iter()
        .map(|&point| {
            let new_ban = put_stone(ban, stone_color, point);
            let min_score = get_min_score(new_ban, stone_color * -1, SEARCH_DEPTH);
            return (point, min_score);
        })
        .min_by_key(|point_score| point_score.1);

    return point_score.map(|ps| ps.0);
}

// x手先を読む
fn get_min_score(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone_color: i8, index: i8) -> usize {
    let points: Vec<(i8, i8)> = get_points(ban, stone_color);
    let score = points.iter()
        .map(|&point| {
            let new_ban = put_stone(ban, stone_color, point);
            let next_color = stone_color * -1;
            if index > 0 {
                get_min_score(new_ban, next_color, index - 1)
            } else {
//                show_ban(new_ban);
                get_score(new_ban, next_color)
            }
        })
        .min();
    return score.unwrap_or(get_score(ban, stone_color * -1));
}


//// x手先を読む
//fn get_next_point(ban: [[i8; BAN_SIZE]; BAN_SIZE], stone_color: i8, index: i8) -> Option<((i8, i8), usize)> {
//    let points: Vec<(i8, i8)> = get_points(ban, stone_color);
//    let point_score = points.iter()
//        .map(|&point| {
//            let new_ban = put_stone(ban, stone_color, point);
//            if index > 0 {
//                let result = get_next_point(new_ban, stone_color * -1, index - 1);
//                if result.is_some() {
//                    return result.unwrap();
//                }
//            }
//            return (point, get_score(new_ban, -stone_color * -1));
//        })
//        .min_by_key(|point_score| point_score.1);
//
//    return point_score;
//}

