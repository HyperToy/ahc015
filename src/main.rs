#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use proconio::{input, source::line::LineSource};
use rand::random;
use std::{
    collections::VecDeque,
    fmt::Display,
    io::{stdin, BufReader},
};

const H: usize = 10;
const W: usize = 10;
const M: usize = 3;
const END_TURN: usize = 100;
const TIME_THRESHOLD: f64 = 1.9;
const F: usize = 0;
const B: usize = 1;
const L: usize = 2;
const R: usize = 3;
const LEGAL_ACTION_COUNT: usize = 4;
// const ACTION_CHARS: [char; LEGAL_ACTION_COUNT] = ['F', 'B', 'L', 'R'];
// const legal_actions: [usize; LEGAL_ACTION_COUNT] = [F, B, L, R];

#[derive(Debug)]
enum Action {
    F,
    B,
    L,
    R,
}
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::F => 'F',
                Self::B => 'B',
                Self::L => 'L',
                Self::R => 'R',
            }
        )
    }
}
impl Action {
    fn random_action() -> Self {
        let i = random::<usize>() % LEGAL_ACTION_COUNT;
        Self::get(i)
    }
    fn get(i: usize) -> Self {
        match i {
            F => Self::F,
            B => Self::B,
            L => Self::L,
            R => Self::R,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        fs: [usize; H * W],
    }

    let mut state = State::new(fs);

    let mut time_keeper = TimeKeeper::new(TIME_THRESHOLD, END_TURN);
    for t in 0..END_TURN {
        time_keeper.set_turn(t);
        input! {
            from &mut source,
            p: usize,
        }

        state.put_candy(p);
        let action = montecarlo::primitive_montecarlo(&time_keeper, state.clone());
        println!("{}", action);

        state.apply_move(action);
    }
    /*
    for i in 0..H * W {
        input! {
            from &mut source,
            p: Usize1,
        }
        println!(
            "{}",
            if i == H * W - 1 {
                'F'
            } else {
                match (fs[i], fs[i + 1]) {
                    (_, 0) => 'F',
                    (0, _) => 'B',
                    (_, 1) => 'R',
                    (_, 2) => 'L',
                    (_, _) => 'F',
                }
            }
        );
    }
    */
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    turn_start_time: std::time::Instant,
    time_threshold: f64,
    end_turn: usize,
    turn: usize,
    now_threshold: f64,
}
impl TimeKeeper {
    fn new(time_threshold: f64, end_turn: usize) -> Self {
        let start_time = std::time::Instant::now();
        let turn_start_time = start_time;
        let turn = 0;
        let now_threshold = time_threshold / end_turn as f64;
        TimeKeeper {
            start_time,
            turn_start_time,
            time_threshold,
            end_turn,
            turn,
            now_threshold,
        }
    }

    // ターンとターン開始時間を更新する
    fn set_turn(&mut self, t: usize) {
        self.turn = t;
        self.turn_start_time = std::time::Instant::now();

        let elapsed_time = (self.turn_start_time - self.start_time).as_secs_f64();
        let remaining_time = self.time_threshold - elapsed_time;
        let remaining_turn = self.end_turn - self.turn;
        self.now_threshold = remaining_time / remaining_turn as f64;
    }

    // 各ターンに割り振られた制限時間を超過したか判定する。
    fn is_time_over(&self) -> bool {
        let now = std::time::Instant::now();
        // let whole_count = (now - self.start_time).as_secs_f64();
        let last_count = (now - self.turn_start_time).as_secs_f64();

        // let remaining_time = self.time_threshold - whole_count;
        // let now_threshold = remaining_time / (self.end_turn - self.turn) as f64;
        last_count >= self.now_threshold
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Status {
    Tilted,
    Placed,
}
#[derive(Clone, Debug)]
struct State {
    fs: Vec<usize>,
    // ps: Vec<usize>,
    board: Vec<Vec<usize>>,
    // turn は 1つ目のキャンディーが置かれてから、2つ目のキャンディーが置かれる直前までが 0
    turn: usize,
    // last: (usize, usize),
    status: Status,
}
impl State {
    fn new(fs: Vec<usize>) -> Self {
        Self {
            fs,
            board: vec![vec![0; W]; H],
            turn: 0,
            status: Status::Tilted,
        }
    }
    fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }
    fn apply_move(&mut self, action: Action) {
        assert_eq!(self.status, Status::Placed);
        match action {
            Action::L => {
                for i in 0..H {
                    let mut k = 0;
                    for j in 0..W {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            Action::R => {
                for i in 0..H {
                    let mut k = W - 1;
                    for j in (0..W).rev() {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            Action::F => {
                for j in 0..W {
                    let mut k = 0;
                    for i in 0..H {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            Action::B => {
                for j in 0..W {
                    let mut k = H - 1;
                    for i in (0..H).rev() {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
        }
        self.status = Status::Tilted;
        self.turn += 1;
    }

    // ランダムな位置にキャンディーをセットする
    fn put_candy_randomly(&mut self) {
        assert!(self.turn < END_TURN);
        assert_eq!(self.status, Status::Tilted);

        let remaining_turn = END_TURN - self.turn;
        let p = random::<usize>() % remaining_turn + 1;
        self.put_candy(p);
    }

    // 指定した位置にキャンディーをセットする
    // parameter: キャンディーが降ってきた位置
    fn put_candy(&mut self, mut parameter: usize) {
        assert!(self.turn < END_TURN);
        assert!(parameter <= END_TURN - self.turn);
        assert_eq!(self.status, Status::Tilted);

        for i in 0..H * W {
            let y = i / W;
            let x = i % W;
            if self.board[y][x] == 0 {
                parameter -= 1;
                if parameter == 0 {
                    self.board[y][x] = self.fs[self.turn];
                    self.status = Status::Placed;
                    return;
                }
            }
        }
    }

    fn group_size(&self, y: usize, x: usize, checked: &mut Vec<Vec<bool>>) -> i32 {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        let candy = self.board[y][x];
        checked[y][x] = true;

        let (y, x) = (y as isize, x as isize);
        let mut queue = VecDeque::new();
        queue.push_back((y, x));
        let mut count = 0;
        while !queue.is_empty() {
            count += 1;
            let (y, x) = queue.pop_front().unwrap();
            for k in 0..4 {
                let ny = y + dy[k];
                let nx = x + dx[k];
                if !(0 <= ny && ny < H as isize && 0 <= nx && nx < W as isize) {
                    continue;
                }
                if checked[ny as usize][nx as usize] {
                    continue;
                }
                if self.board[ny as usize][nx as usize] != candy {
                    continue;
                }
                checked[ny as usize][nx as usize] = true;
                queue.push_back((ny, nx));
            }
        }
        count
    }
    fn score(&self) -> f64 {
        let mut score = 0;
        let mut checked = vec![vec![false; W]; H];
        for y in 0..H {
            for x in 0..W {
                if self.board[y][x] != 0 && !checked[y][x] {
                    let group_size = self.group_size(y, x, &mut checked);
                    score += group_size.pow(2);
                }
            }
        }
        score as f64
    }
}

mod montecarlo {
    use crate::{Action, State, Status, TimeKeeper, LEGAL_ACTION_COUNT};

    fn play1turn() -> Action {
        Action::random_action()
    }
    fn playout(mut state: State) -> f64 {
        state.apply_move(play1turn());
        // let before_turn = state.turn;
        while !state.is_done() {
            state.put_candy_randomly();
            state.apply_move(play1turn());
        }
        state.score()
    }
    pub(crate) fn primitive_montecarlo(time_keeper: &TimeKeeper, base_state: State) -> Action {
        assert_eq!(base_state.status, Status::Placed);
        let mut w = vec![0.; LEGAL_ACTION_COUNT];
        for _simulate_count in 0.. {
            if time_keeper.is_time_over() {
                break;
            }
            for d in 0..LEGAL_ACTION_COUNT {
                let mut state = base_state.clone();
                state.apply_move(Action::get(d));
                if state.is_done() {
                    w[d] += state.score();
                } else {
                    state.put_candy_randomly();
                    w[d] += playout(state);
                }
            }
        }
        let mut ret = None;
        let mut best = -1.;
        for d in 0..LEGAL_ACTION_COUNT {
            if w[d] > best {
                ret = Some(Action::get(d));
                best = w[d];
            }
        }
        ret.unwrap()
    }
}
