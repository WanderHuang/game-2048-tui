use crate::game::{Game, Command, Grid};


/// 2048 Application
/// 
/// Rules：
/// 
/// 1. make a `4 * 4` grid
/// 2. each square is same size
/// 3. board_size = box_size * 4
/// 
/// :> TODO make each `config` as a input list so this game can be customized;
pub struct App {
  /// terminal x-axis start point
  pub x: f64,
  /// terminal y-axis start point
  pub y: f64,
  /// each square size
  pub box_size: f64,
  /// Application's game
  game: Game,
  /// Application's commands
  queue: Vec<Command>,
  /// Application's score
  score: i32,
}


impl App {
  /// create your application
  pub fn new() -> App {
    let game = Game::new();
    let mut app = App {
      x: 0.0,
      y: 0.0,
      box_size: 40.0,
      game,
      queue: Vec::new(),
      score: 0
    };

    // init your game
    app.game.start();

    app
  }

  /// box_size * 4
  pub fn get_size(&self) -> f64 {
    self.box_size * 4.0
  }

  /// calculate the next tick
  pub fn next(&mut self) {
    if self.is_alive() && !self.queue.is_empty() {
      if let Some(top) = self.queue.pop() {
        if top != Command::Nil {
          self.game.next_tick(top);
          self.score = self.game.get_score();
        }
      } else {
        println!("queue error");
      }
    }
  }

  /// get current score
  pub fn get_score(&self) -> i32 {
    self.score
  }

  /// get game status, alive or dead
  pub fn is_alive(&self) -> bool {
    self.game.alive
  }

  /// add some command
  pub fn add_command(&mut self, cmd: Command) {
    if self.is_alive() {
      self.queue.insert(0, cmd)
    }
  }

  /// restart application
  pub fn restart(&mut self) {
    if !self.is_alive() {
      self.game = Game::new();
      self.game.start();
      self.queue = vec![];
      self.score = 0;
    }
  }

  /// game grid
  pub fn get_grid(&self) -> Grid {
    self.game.get_grid()
  }

  /// calculate result board points when game over
  pub fn get_game_over_modal(&self) -> Vec<(f64, f64)> {
    let mut all: Vec<(f64, f64)> = vec![];
    let board_size = self.get_size();

    let x = board_size / 2.0 - self.box_size * 1.5;
    let y = board_size / 2.0 - self.box_size / 2.0;
    let width = self.box_size * 3.0;
    let height = self.box_size - 10.0;
    let mut p_x = x;
    loop {
        let i = p_x + 1.0;
        if i >= x + width {
            break;
        }
        let mut p_y = y;
        loop {
            let j = p_y + 1.0;
            if j >= y + height {
                break;
            }
            all.push((i, j));
            p_y = j;
        }

        p_x = i;
    }

    all
  }

}

