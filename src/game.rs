use crate::board::Board;
use crate::mark::Mark;
use crate::messages;
use crate::player::{computer::Computer, human::Human, Player};
use crate::std_io::{console_io::ConsoleIo, StdIo};
use crate::ui::Ui;
use core::fmt::Debug;

#[derive(Debug)]
pub struct Game<U: StdIo> {
    board: Board,
    players: Vec<Box<dyn Player>>,
    ui: Ui<U>,
}

impl<U> Game<U>
where
    U: StdIo,
{
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
                if self.board.is_available_cell(position) {
                    break position;
                }
            };
            self.board.set_mark(player_move, player.mark());
        }

        self.ui.print(&self.board.to_string());
        if let Some(winner) = self.board.winner() {
            self.ui
                .print(&format!("{} {}", messages::WIN_MESSAGE, winner));
        } else {
            self.ui.print(messages::DRAW_MESSAGE);
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui<U> {
        &mut self.ui
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

impl Debug for dyn Player {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Player {}", self.mark())
    }
}
