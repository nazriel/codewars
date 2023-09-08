use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Entity {
    Wall,
    Empty,
}

const DIRECTION_MAP: [(i8, i8); 4] = [
    (0, 1),  // up
    (0, -1), // down
    (-1, 0), // left
    (1, 0),  // right
];

#[derive(Default, Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Default, Debug)]
struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Entity>>,
    start: Position,
    end: Position,
}

impl Maze {
    fn from(input: &str) -> Self {
        let mut m: Self = Default::default();
        m.start = Position { row: 0, col: 0 };
        m.height = input.lines().count();

        for (row, line) in input.lines().enumerate() {
            if row == 0 {
                m.width = line.len();
                m.grid = vec![vec![Entity::Empty; m.width]; m.height];
            }
            for (col, c) in line.chars().enumerate() {
                if col >= m.width {
                    // panic!("Invalid maze");
                    continue;
                }
                match c {
                    'W' => m.grid[row][col] = Entity::Wall,
                    '.' => m.grid[row][col] = Entity::Empty,
                    _ => panic!("Invalid character in maze"),
                }
            }
        }
        m.end = Position {
            row: m.height - 1,
            col: m.width - 1,
        };
        m
    }

    fn can_move(&self, pos: Position) -> bool {
        self.grid[pos.row][pos.col] == Entity::Empty
    }

    fn solve(self) -> Option<u32> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut queue = VecDeque::new();
        queue.push_back((Position { row: 0, col: 0 }, 0u32));
        visited[0][0] = true;
        let mut completed: Vec<u32> = Vec::new();

        while let Some((pos, tries)) = queue.pop_front() {
            if pos == self.end {
                completed.push(tries);
            }

            for (x, y) in DIRECTION_MAP {
                let next_row = pos.row as i32 + y as i32;
                let next_col = pos.col as i32 + x as i32;
                if next_row < 0 || next_row >= self.height as i32 {
                    continue;
                }
                let next_row = next_row as usize;

                if next_col < 0 || next_col >= self.width as i32 {
                    continue;
                }
                let next_col = next_col as usize;

                if !visited[next_row][next_col]
                    && self.can_move(Position {
                        row: next_row,
                        col: next_col,
                    })
                {
                    visited[next_row][next_col] = true;
                    queue.push_back((
                        Position {
                            row: next_row,
                            col: next_col,
                        },
                        tries + 1,
                    ));
                }
            }
        }
        dbg!(&completed);
        if completed.len() == 0 {
            None
        } else {
            Some(completed.iter().min().unwrap_or(&0).clone())
        }
    }
}

fn path_finder(maze: &str) -> Option<u32> {
    let m = Maze::from(maze);
    dbg!(&m);
    m.solve()
}

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn fixed_tests() {
        assert_eq!(
            path_finder(".W.\n.W.\n..."),
            Some(4),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W.\n.W.\nW.."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n......\n......"),
            Some(10),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n.....W\n....W."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".WWWW\n.W...\n.W.W.\n.W.W.\n...W."),
            Some(14),
            "\nYour answer (left) is not the expected answer (right)."
        );
    }

    #[test]
    fn failing_on_runner() {
        assert_eq!(
            path_finder(".W.\n.W.\n..."),
            Some(4),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W.\n.W.\nW.."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n......\n......"),
            Some(10),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n.....W\n....W."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W...\n.W...\n.W.W.\n...W.\n...W."),
            Some(12),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W...\n.W...\n.W.W.\n...WW\n...W."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("..W\n.W.\nW.."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".WWWW\n.W...\n.W.W.\n.W.W.\n...W."),
            Some(14),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W...\nW....\n.....\n.....\n......"),
            Some(14),
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
