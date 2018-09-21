extern crate c4;
extern crate ncurses;

use c4::c4::*;
use c4::mcts::*;
use ncurses::*;
use std::char;

fn main() {
    initscr();
    noecho();
    loop {
        let winner = game_loop();
        printw(&format!("The winner is {:?}", winner));
        getch();
        endwin();
    }
}

fn game_loop() -> Player {
    let mut board = Board::new(vec![Player::Empty; 42], 0, 7, 6);
    draw(&board);
    for _ in 0..42 {
        board = match game_step(board) {
            State::Winner(w) => return w,
            State::Board(b) => b,
        }
    }
    Player::Empty
}

fn game_step(board: Board) -> State {
    let p = if board.turn_number % 2 == 0 {
        Player::P1
    } else {
        Player::P2
    };

    let mut input;
    let mut new_board = loop {
        input = match p {
            Player::P2 => mcts(&board, p),
            _ => read_input(),
        };
        let b_opt = board.place(input, p);
        match b_opt {
            Some(b) => break b,
            None => printw("col full!"),
        };
    };

    draw(&new_board);

    if new_board.is_over(input) {
        return State::Winner(p);
    } else {
        return State::Board(new_board);
    }
}

enum State {
    Board(Board),
    Winner(Player),
}

fn read_input() -> u32 {
    let input = loop {
        let ch = getch();
        if let Some(i) = char::from_u32(ch as u32).expect("Invalid char").to_digit(7) {
            break i;
        }
        printw("invalid");
        refresh();
    };
    input
}

fn draw(board: &Board) {
    erase();
    let p = if board.turn_number % 2 == 0 {
        Player::P1
    } else {
        Player::P2
    };

    printw(&format!("{:?}'s turn:\n\n", p));
    for i in (0..board.h).rev() {
        for k in 0..board.w {
            match board.get(k, i) {
                Player::P1 => printw("  X  "),
                Player::P2 => printw("  Y  "),
                Player::Empty => printw("  0  "),
            };
        }
        printw("\n\n");
    }
    refresh();
}
