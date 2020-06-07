use crate::board::Board;
use crate::mark::Mark;
use crate::player::{computer::Computer, human::Human, Player};
use crate::std_io::{console_io::ConsoleIo, StdIo};
use crate::ui::Ui;

pub(crate) struct Game<U: StdIo> {
    board: Board,
    players: Vec<Box<dyn Player>>,
    ui: Ui<U>,
}

impl<U> Game<U>
where
    U: StdIo,
{
    const WIN_MESSAGE: &'static str = "The winner is";
    const DRAW_MESSAGE: &'static str = "It is a draw";

    pub fn new(board: Board, players: Vec<Box<dyn Player>>, ui: Ui<U>) -> Self {
        Game { board, players, ui }
    }

    pub fn run(&mut self) {
        for player in self.players.iter().cycle() {
            if self.board.is_game_over() {
                break;
            }
            self.ui.print(&self.board.to_string());
            let player_move = loop {
                let position = player.get_move(&self.board);
                if self.is_valid_move(position) {
                    break position;
                }
            };
            self.board.set_mark(player_move, player.mark());
        }

        self.ui.print(&self.board.to_string());
        if let Some(winner) = self.board.winner() {
            self.ui.print(&format!("{} {}", Self::WIN_MESSAGE, winner));
        } else {
            self.ui.print(Self::DRAW_MESSAGE);
        }
    }

    fn is_valid_move(&self, player_move: usize) -> bool {
        player_move < self.board.size() && !self.board.is_occupied(player_move)
    }
}

impl Game<ConsoleIo> {
    pub fn with_defaults() -> Self {
        let board = Board::new(9);
        let ui = Ui::with_defaults();
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(Human::new(Mark::X, Ui::with_defaults())),
            Box::new(Computer::with_defaults(Mark::O, Ui::with_defaults())),
        ];
        Self::new(board, players, ui)
    }
}
