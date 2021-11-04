use rand::Rng;

/// 游戏功能
/// 
/// 思路
/// 
/// 1. 一个面板，用于渲染数字，执行逻辑
/// 2. 游戏存活状态
pub struct Game {
    /// 是否存活
    pub alive: bool,
    /// 面板
    panel: Panel,
}

impl Game {
    pub fn new() -> Game {
        Game {
            alive: true,
            panel: Panel::new(),
        }
    }

    pub fn start(&mut self) {
        self.panel.init();

        self.panel.get_grid();
    }

    pub fn get_score(&self) -> i32 {
        self.panel
            .grid
            .iter()
            .fold(0, |acc, x| acc + x.iter().fold(0, |row, y| row + y))
    }

    /// 下一个时钟
    pub fn next_tick(&mut self, cmd: Command) {
        self.panel.next_tick(cmd);
        self.alive = self.panel.check_alive();

        if self.alive {
            self.panel.random_insert();
        }
    }

    /// 获取面板
    pub fn get_grid(&self) -> Grid {
        self.panel.get_grid()
    }
}

/// 游戏命令
#[derive(PartialEq, Debug)]
pub enum Command {
    /// 向左
    Left,
    /// 向上
    Up,
    /// 向右
    Right,
    /// 向下
    Down,
    /// 无效命令
    Nil,
}

/// 矩阵
pub type Grid = [[i32; 4]; 4];

/// 游戏面板
/// 
/// 思路
/// 
/// 1. 有数字矩阵
/// 2. 其他扩展功能
struct Panel {
    grid: [[i32; 4]; 4],
}

impl Panel {
    pub fn new() -> Panel {
        Panel { grid: [[0; 4]; 4] }
    }

    /// 初始化，插入两条数据
    pub fn init(&mut self) {
        self.random_insert();
        self.random_insert();
    }

    /// 获取矩阵
    pub fn get_grid(&self) -> Grid {
        self.grid
    }

    /// 随机插入一条数据
    /// 
    /// ｜TODO 插入数据应该按照当前最小值来定
    pub fn random_insert(&mut self) {
        let mut vec: Vec<(usize, usize)> = vec![];
        for (i, row) in self.grid.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                if *x == 0 {
                    vec.push((i, j));
                }
            }
        }

        let len = vec.len();

        if len == 0 {
            return
        }

        let rand_num: usize = rand::thread_rng().gen_range(0..len);
        let (i, j) = vec[rand_num];

        let rand_num = rand::thread_rng().gen_range(0..10);
        let val = if rand_num < 6 { 2 } else { 4 };

        self.grid[i][j] = val;
    }

    /// 检查是否存活
    /// 
    /// 思路
    /// 
    /// 1. 有0存在，游戏继续
    /// 2. 铺满格子后，有相邻的值相等，游戏继续
    pub fn check_alive(&self) -> bool {

        let has_zero = self.grid.iter().any(|row| row.iter().any(|x| *x == 0));

        if has_zero {
            return true;
        }

        let mut has_same = false;
        
        for (i, row) in self.grid.iter().enumerate() {
            if has_same {
                break;
            }
            for (j, x) in row.iter().enumerate() {
                
                let left = if j != 0 {
                    self.grid[i][ j - 1]
                } else {
                    0
                };
                let up = if i != 0 {
                    self.grid[i - 1][j]
                } else {
                    0
                };
                let right = if j != 3 {
                    self.grid[i][j + 1]
                } else {
                    0
                };
                let down = if i != 3 {
                    self.grid[i + 1][j]
                } else {
                    0
                };


                if left == *x || up == *x || right == *x || down == *x {
                    has_same = true;
                    break;
                }
            }
        }

        
        has_same
    }

    /// 计算下一个格子
    /// 
    /// 思路
    /// 
    /// 1. 根据移动方向，合并相邻且相等的值，并移动位置
    /// 2. 循环计算，直到移动方向上没有相邻且相等的值
    pub fn next_tick(&mut self, cmd: Command) -> Grid {
        let mut grid = self.grid.clone();

        match cmd {
            Command::Down => {
                for y in 0..4 {
                    let mut res = sum(vec![
                        self.grid[0][y],
                        self.grid[1][y],
                        self.grid[2][y],
                        self.grid[3][y],
                    ]);

                    loop {
                        if res.len() < 4 {
                            res.push(0);
                        } else {
                            break;
                        }
                    }

                    res = res.into_iter().rev().collect();

                    grid[0][y] = res[0];
                    grid[1][y] = res[1];
                    grid[2][y] = res[2];
                    grid[3][y] = res[3];
                }
            }
            Command::Up => {
                for y in 0..4 {
                    let mut res = sum(vec![
                        self.grid[3][y],
                        self.grid[2][y],
                        self.grid[1][y],
                        self.grid[0][y],
                    ]);

                    loop {
                        if res.len() < 4 {
                            res.push(0);
                        } else {
                            break;
                        }
                    }

                    grid[0][y] = res[0];
                    grid[1][y] = res[1];
                    grid[2][y] = res[2];
                    grid[3][y] = res[3];
                }
            }
            Command::Left => {
                for x in 0..4 {
                    let mut res = sum(vec![
                        self.grid[x][3],
                        self.grid[x][2],
                        self.grid[x][1],
                        self.grid[x][0],
                    ]);

                    loop {
                        if res.len() < 4 {
                            res.push(0);
                        } else {
                            break;
                        }
                    }

                    grid[x][0] = res[0];
                    grid[x][1] = res[1];
                    grid[x][2] = res[2];
                    grid[x][3] = res[3];
                }
            }
            Command::Right => {
                for x in 0..4 {
                    let mut res = sum(vec![
                        self.grid[x][0],
                        self.grid[x][1],
                        self.grid[x][2],
                        self.grid[x][3],
                    ]);

                    loop {
                        if res.len() < 4 {
                            res.push(0);
                        } else {
                            break;
                        }
                    }

                    res = res.into_iter().rev().collect();

                    grid[x][0] = res[0];
                    grid[x][1] = res[1];
                    grid[x][2] = res[2];
                    grid[x][3] = res[3];
                }
            }
            _ => {}
        }

        self.grid = grid;

        self.grid
    }
}

/// 计算累计和 需要循环计算，两个挨着的数字如果值一样，则合并
///
/// 1 2 2 4 -> 1 8
fn sum(arr: Vec<i32>) -> Vec<i32> {
    let mut added = false;
    let res = arr.into_iter().rev().fold(Vec::new(), |mut acc, curr| {
        if let Some(x) = acc.last_mut() {
            if x == &curr {
                *x = curr * 2;
                added = true;
            } else if curr != 0 {
                acc.push(curr);
            }
        } else if curr != 0 {
            acc.push(curr);
        }

        return acc;
    });

    if added {
        return sum(res.into_iter().rev().collect());
    }

    res
}
