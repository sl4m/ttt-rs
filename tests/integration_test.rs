use std::cell::RefCell;
use ttt_rs::{
    board::Board, game::Game, mark::Mark, player::human::Human, player::Player, std_io::StdIo,
    ui::Ui,
};

struct DoubleStdIo<'a> {
    inputs: RefCell<Vec<&'a str>>,
    outputs: RefCell<Vec<String>>,
}

impl<'a> DoubleStdIo<'a> {
    pub fn new(inputs: Vec<&'a str>) -> Self {
        Self {
            inputs: RefCell::new(inputs),
            outputs: RefCell::new(vec![]),
        }
    }

    pub fn does_contain(&mut self, text: &str) -> bool {
        self.outputs
            .borrow_mut()
            .iter()
            .any(|output| output.contains(text))
    }
}

impl StdIo for DoubleStdIo<'_> {
    fn println(&self, text: &str) {
        self.outputs.borrow_mut().push(text.to_owned());
    }

    fn prompt(&self) -> String {
        self.inputs.borrow_mut().pop().unwrap().to_owned()
    }
}

#[test]
fn it_runs_through_the_draw_game() {
    let board = Board::new(9);
    let x_std_io = DoubleStdIo::new(player_x_tie());
    let x_ui = Ui::new(x_std_io);
    let o_std_io = DoubleStdIo::new(player_o_tie());
    let o_ui = Ui::new(o_std_io);
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(Human::new(Mark::X, x_ui)),
        Box::new(Human::new(Mark::O, o_ui)),
    ];
    let mut game = Game::new(board, players, Ui::new(DoubleStdIo::new(vec![])));
    game.run();

    assert!(game.board().winner().is_none());
    assert!(game.ui_mut().io_mut().does_contain("draw"));
}

#[test]
fn it_runs_through_the_win_game() {
    let board = Board::new(9);
    let x_std_io = DoubleStdIo::new(player_x_win());
    let x_ui = Ui::new(x_std_io);
    let o_std_io = DoubleStdIo::new(player_o_lose());
    let o_ui = Ui::new(o_std_io);
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(Human::new(Mark::X, x_ui)),
        Box::new(Human::new(Mark::O, o_ui)),
    ];
    let mut game = Game::new(board, players, Ui::new(DoubleStdIo::new(vec![])));
    game.run();

    assert_eq!(&Mark::X, game.board().winner().unwrap());
    let io_mut = game.ui_mut().io_mut();
    assert!(io_mut.does_contain("winner"));
    assert!(io_mut.does_contain(&Mark::X.to_string()));
}

fn player_x_tie() -> Vec<&'static str> {
    vec!["3", "2", "7", "8", "0"]
}

fn player_o_tie() -> Vec<&'static str> {
    vec!["5", "6", "1", "4"]
}

fn player_x_win() -> Vec<&'static str> {
    vec!["6", "4", "2", "0"]
}

fn player_o_lose() -> Vec<&'static str> {
    vec!["5", "3", "1"]
}
