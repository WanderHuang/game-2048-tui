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
  /// 装数字的盒子大小 默认40
  pub box_size: f64,
  /// 游戏功能
  game: Game,
  /// 命令队列
  queue: Vec<Command>,
  /// 分数
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

  /// box_size * 4
  pub fn get_size(&self) -> f64 {
    self.box_size * 4.0
  }

  /// 执行下一个命令
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

  /// 当前分数
  pub fn get_score(&self) -> i32 {
    self.score
  }

  /// 游戏状态
  pub fn is_alive(&self) -> bool {
    self.game.alive
  }

  /// 添加命令
  pub fn add_command(&mut self, cmd: Command) {
    if self.is_alive() {
      self.queue.insert(0, cmd)
    }
  }

  /// 重新开始
  pub fn restart(&mut self) {
    if !self.is_alive() {
      self.game = Game::new();
      self.game.start();
      self.queue = vec![];
      self.score = 0;
    }
  }

  /// 数字矩阵
  pub fn get_grid(&self) -> Grid {
    self.game.get_grid()
  }

  /// 获取游戏结束面板
  pub fn get_game_over_modal(&self) -> Vec<(f64, f64)> {
    // 死掉了
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

