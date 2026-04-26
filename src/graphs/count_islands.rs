pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let mut count = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '1' {
                    Self::dfs(row, col, &mut grid);
                    count += 1;
                }
            }
        }

        return count;
    }

    fn dfs(row: usize, col: usize, grid: &mut Vec<Vec<char>>) {
        grid[row][col] = '0';

        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for dir in directions {
            let next_row = row as isize + dir.0;
            let next_col = col as isize + dir.1;

            if Self::is_within_bounds(next_row, next_col, &grid) {
                if grid[next_row as usize][next_col as usize] == '1' {
                    Self::dfs(next_row as usize, next_col as usize, grid);
                }
            }
        }
    }

    fn is_within_bounds(row: isize, col: isize, grid: &Vec<Vec<char>>) -> bool {
        row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize
    }
}
