use crate::game::{Game, Command, Grid};


/// 2048 游戏面板
/// 
/// 创建规则：
/// 
/// 1. 横纵都有四个格子 40
/// 2. 横纵都有五条线 2
/// 3. 总大小 = [(格子个数 * 格子边宽) + (线条数 * 线条宽)] ^ 2
/// 
/// :> TODO 可以把游戏面板的值设置为可配置的，增加可玩性
pub struct App {
  /// 横轴在termion中起点位置
  pub x: f64,
  /// 纵轴在termion中起点位置
  pub y: f64,
  /// 装数字的盒子大小
  pub box_size: f64,
  game: Game,
  queue: Vec<Command>,
  score: i32,
}


impl App {
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

    // 初始化面板
    app.game.start();

    app
  }

  pub fn get_size(&self) -> f64 {
    self.box_size * 4.0
  }

  /// 执行下一个命令
  pub fn next(&mut self) {
    if !self.queue.is_empty() {
      let top = self.queue.pop().unwrap();
      if top != Command::Nil {
        self.game.next_tick(top);
        self.score = self.game.get_score();
      }
    }
  }

  pub fn get_score(&self) -> i32 {
    self.score
  }

  /// 添加命令
  pub fn add_command(&mut self, cmd: Command) {
    self.queue.insert(0, cmd)
  }

  pub fn get_grid(&self) -> Grid {
    self.game.get_grid()
  }

}

