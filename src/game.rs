use crate::board::Board;
use crate::mark::Mark;
use crate::player::{computer::Computer, console::human::Human, Player};
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
            self.print_board();
            let player_move = loop {
                self.ui.print_turn_message(&player.mark().to_string());
                let position = player.get_move(&self.board);
                if self.board.is_available_cell(position) {
                    break position;
                }
            };
            self.board.set_mark(player_move, player.mark());
        }

        self.print_board();
        if let Some(winner) = self.board.winner() {
            self.ui.print_winner(&winner.to_string());
        } else {
            self.ui.print_draw();
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn ui_mut(&mut self) -> &mut Ui<U> {
        &mut self.ui
    }

    fn print_board(&self) {
        self.ui.print(&format!("\n{:#}", &self.board));
    }
}

impl Game<ConsoleIo> {
    pub fn run_with_defaults() {
        let board = Board::new(9);
        let ui = Ui::with_defaults();
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(Human::with_defaults(Mark::X)),
            Box::new(Computer::with_defaults(Mark::O)),
        ];
        let mut game = Self::new(board, players, ui);
        game.run();
    }
}

impl Debug for dyn Player {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Player {}", self.mark())
    }
}
