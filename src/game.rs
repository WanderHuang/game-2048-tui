use rand::Rng;

pub struct Game {
    alive: bool,
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
        let mut score = 0;
        for x in 0..4 {
            for y in 0..4 {
                score += self.panel.grid[x][y];
            }
        }

        score
    }

    /// 下一个时钟
    pub fn next_tick(&mut self, cmd: Command) {
        self.panel.next_tick(cmd);
        self.alive = self.panel.check_alive();
        if !self.alive {
            panic!("Game Over!")
        } else {
            self.panel.random_insert();
        }
    }

    /// 获取面板
    pub fn get_grid(&self) -> Grid {
        self.panel.get_grid()
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Left,
    Up,
    Right,
    Down,
    Nil,
}

pub type Grid = [[i32; 4]; 4];

struct Panel {
    grid: [[i32; 4]; 4],
}

impl Panel {
    pub fn new() -> Panel {
        Panel {
            grid: [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }

    pub fn init(&mut self) {
        self.random_insert();
        self.random_insert();
    }

    pub fn get_grid(&self) -> Grid {
        self.grid
    }

    pub fn random_insert(&mut self) {
        let max = 16;

        let mut vec: Vec<i32> = vec![];

        for i in 0..max {
            let y = i % 4;
            let x = i / 4;

            if self.grid[x][y] == 0 {
                vec.push(i as i32);
            }
        }

        let len = vec.len();

        let x = rand::thread_rng().gen_range(0..len);
        let x = vec[x] as usize;

        let y = rand::thread_rng().gen_range(0..max);
        let val = if y < 10 {
            2
        }  else {
            4
        };



        self.grid[x / 4][x % 4] = val;
    }

    pub fn check_alive(&self) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                let cur = self.grid[x][y];
                if cur == 0 {
                    return true
                }
                let up = if y > 0 {
                    if y < 3 {
                        self.grid[x][y - 1]
                    } else {
                        0
                    }
                } else {
                    0
                };
                if up == 0 {
                    return true;
                }
                let left = if x > 0 {
                    if x < 3 {
                        self.grid[x - 1][y]
                    } else {
                        0
                    }
                } else {
                    0
                };
                if left == 0 {
                    return true;
                }
                let right = if x < 3 {
                        self.grid[x + 1][y]
                } else {
                    0
                };
                if right == 0 {
                    return true;
                }
                let down = if y < 3 {
                    self.grid[x][y + 1]
                } else {
                    0
                };

                if down == 0 {
                    return true;
                }

                if cur == up || cur == left || cur == right || cur == down {
                    return true;
                }

            }
        }

        false
    }

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
