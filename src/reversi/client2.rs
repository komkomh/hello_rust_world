use std::io;

const BAN_SIZE: usize = 8;
const SEARCH_DEPTH: usize = 5;

fn main() {

    let mut ban = create_ban();
    show_ban(&ban);

    while ban.is_game_end() == false {

        let input_position: Option<(usize, usize)> = input_position(&ban);
        if input_position.is_some() {
            ban = ban.put_stone(input_position.unwrap());
            show_ban(&ban);
        } else {
            println!("user pass");
        }

        // 次石を検索する
        println!("searching...");
        let position_score = search_position_score(&ban, SEARCH_DEPTH);
        let searched_position: Option<(usize, usize)> = position_score.0;
        if input_position.is_some() {
            ban = ban.put_stone(searched_position.unwrap());
            show_ban(&ban);
        } else {
            println!("cpu pass");
        }

    }
    println!("game end");
    show_ban(&ban);
}

fn create_ban() -> Ban {
    let banmen: [[Option<Stone>; 8]; 8] =
        [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, Some(Stone::White), Some(Stone::Black), None, None, None],
            [None, None, None, Some(Stone::Black), Some(Stone::White), None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
    let teban: Stone = Stone::Black;
    return Ban { banmen: banmen, teban: teban };
}

fn show_ban(ban: &Ban) {
    println!("---------------");
    ban.banmen.iter().for_each(|array| {
        array.iter().for_each(|stone| {
            let str_stone = stone
                .map(|stone| stone.to_string())
                .unwrap_or(" ");
            print!("|{}", str_stone);
        });
        println!("|")
    });
}

fn input_position(ban: &Ban) -> Option<(usize, usize)> {
    // 位置選択を表示する
    let positions = ban.get_positions();
    let mut i = 0;
    for (y, x) in positions {
        println!("{}: {}-{}", i, y, x);
        i = i + 1;
    }

    // 入力値を取得する
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();

    let cs: Vec<usize> = buf.split_whitespace().filter_map(|k| k.parse().ok()).collect::<Vec<usize>>();
    let input = cs[0];
    Some(ban.get_positions()[input])
}

fn search_position_score(ban: &Ban, index: usize) -> (Option<(usize, usize)>, usize) {
    let positions: Vec<(usize, usize)> = ban.get_positions();
    let position_scores = positions.iter()
        .map(|&position| {
            if index <= 0 {
                (Some(position), ban.get_score())
            } else {
                let next_ban = ban.put_stone(position);
                let position_score = search_position_score(&next_ban, index - 1);
                (Some(position), position_score.1)
            }
        });
    let score = if index % 2 == 0 {
        position_scores.max_by_key(|point_score| point_score.1)
    } else {
        position_scores.min_by_key(|point_score| point_score.1)
    };
    score.unwrap_or((None, ban.get_score()))
}

// 石を表現する
#[derive(Copy, Clone, Eq, PartialEq)]
enum Stone {
    Black,
    White,
}

impl Stone {
    fn value_of(v: i8) -> Option<Stone> {
        match v {
            1 => Some(Stone::Black),
            -1 => Some(Stone::White),
            _ => None,
        }
    }

    fn reverse<'a>(&'a self) -> Stone {
        match self {
            Stone::White => Stone::Black,
            Stone::Black => Stone::White,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            Stone::Black => "●",
            Stone::White => "○",
        }
    }
}

// 盤を表現する
#[derive(Copy, Clone)]
struct Ban {
    banmen: [[Option<Stone>; BAN_SIZE]; BAN_SIZE],
    teban: Stone,
}

impl Ban {
    // 指定されたマスの石を取得する
    fn get_stone(&self, point: (usize, usize)) -> Option<Stone> {
        if point.0 < 0 as usize || point.0 >= BAN_SIZE || point.1 < 0 as usize || point.1 >= BAN_SIZE {
            Option::None
        } else {
            self.banmen[point.0 as usize][point.1 as usize]
        }
    }

    // 置ける場所のリストを取得する
    fn get_positions(&self) -> Vec<(usize, usize)> {
        let points: Vec<(usize, usize)> = (0..BAN_SIZE)
            .flat_map(|y| (0..BAN_SIZE)
                .filter(move |&x| {
                    let point = (y, x);
                    let count = &self.count_reverse_stones(point);
                    return *count > 0;
                })
                .map(move |x| (y, x)))
            .collect();
        return points;
    }

    // 裏返す石を数える
    fn count_reverse_stones(&self, point: (usize, usize)) -> usize {
        let stone = self.get_stone(point);
        if stone.is_some() {
            return 0;
        }

        let reverse_count = (-1..2)
            .map(|y| {
                let line_reverse_count: usize = (-1..2)
                    .map(|x| self.count_reverse_line_stones(point, (y, x), 0))
                    .sum();
                return line_reverse_count;
            })
            .sum();
        return reverse_count;
    }

    // 裏返す石を数える(1行)
    fn count_reverse_line_stones(&self,
                                 point: (usize, usize),
                                 direction: (i8, i8),
                                 count: usize) -> usize {
        // 調査点を生成する
        let y = (point.0 as i8 + direction.0) as usize;
        let x = (point.1 as i8 + direction.1) as usize;
        let new_point = (y, x);

        // マスを取得して
        self.get_stone(new_point)
            // 石が無ければ0
            .map_or(0, |stone| {
                // 自石なら
                if stone == self.teban {
                    // 反転数を返す
                    count
                }
                // 他石なら
                else {
                    // 次の場所を数える
                    self.count_reverse_line_stones((y, x), direction, count + 1)
                }
            })
    }

    fn get_stones<'a>(&'a self) -> Vec<Stone> {
        let stones = self.banmen.iter()
            .flat_map(|array| array.iter())
            .flat_map(|&stone| stone)
            .collect();
        return stones;
    }

    fn is_game_end(&self) -> bool {
        self.get_positions().len() == 0 && self.change_teban().get_positions().len() == 0
    }

    // 評価関数
    fn get_score(&self) -> usize {
        // ゲームが終了していれば
        if self.is_game_end() {
            // 自石の数を取得
            let my_count = self.get_stones().iter().filter(|&&stone| stone == self.teban).count();
            // 他石の数を取得
            let opponent_cont = self.get_stones().iter().filter(|&&stone| stone != self.teban).count();
            // 勝っていれば1000点, 負けていれば0点とする
            return if my_count > opponent_cont { 1000 } else { 0 };
        }

        // 石を置ける場所の数をscoreとする
        let point_score = self.get_positions().len();

        // 角を取っていれば100点追加する
        let kados = [(0, 0), (0, BAN_SIZE - 1), (BAN_SIZE - 1, 0), (BAN_SIZE - 1, BAN_SIZE - 1)];
        let kado_score: usize = kados.iter()
            .map(|kado| self.banmen[kado.0][kado.1])
            .flat_map(|stone| stone)
            .filter(|stone| *stone == self.teban)
            .map(|_| 100)
            .sum();
        return point_score + kado_score;
    }

    fn change_teban(&self) -> Ban {
        let new_banmen = self.banmen.clone();
        let new_teban = self.teban.reverse();
        let new_ban = Ban { banmen: new_banmen, teban: new_teban };
        return new_ban;
    }

    // 石を置いた後の盤を取得する
    fn put_stone(&self, point: (usize, usize)) -> Ban {

        // 指定場所に石があれば
        if self.get_stone(point).is_some() {
            // 何もしない
            return self.change_teban();
        }

        let points =
            (-1..2).flat_map(|y| {
                (-1..2)
                    .map(move |x| {
                        self.get_reverse_positions(point, (y, x))
                    })
                    .filter(|p| p.is_some())
                    .flat_map(|p| p.unwrap())
            });

        // 新しい盤を生成する
        let mut new_banmen = self.banmen.clone();
        // 盤に石を置く
        new_banmen[point.0][point.1] = Some(self.teban);
        // 盤上の石を裏返す
        points.for_each(|reversed_point| {
            new_banmen[reversed_point.0][reversed_point.1] = Some(self.teban)
        });

        return Ban { banmen: new_banmen, teban: self.teban.reverse() };
    }


    // 反転する位置リストを取得する
    fn get_reverse_positions(&self, position: (usize, usize),
                             direction: (i8, i8)) -> Option<Vec<(usize, usize)>> {
        // 調査点を生成する
        let y = (position.0 as i8 + direction.0) as usize;
        let x = (position.1 as i8 + direction.1) as usize;
        let new_position = (y, x);

        // この位置の石を取得する
        let masu = self.get_stone(new_position);
        masu.map_or(Option::None, |stone| {
            // 自分の色であれば
            if stone == self.teban {
                // 次の場所を
                Some(Vec::new())
            }
            // 相手の色であれば
            else {
                // 反転位置のリストを取得する
                self.get_reverse_positions(new_position, direction)
                    .map(|points| {
                        let mut new_points = points;
                        new_points.push(new_position);
                        return new_points;
                    })
            }
        })
    }
}
