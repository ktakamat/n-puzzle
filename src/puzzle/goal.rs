pub fn generate_snail_goal(n: usize) -> Vec<u16> {
    let mut grid = vec![vec![0; n]; n];
    let mut curr = 1;
    let (mut top, mut bottom) = (0, n as isize - 1);
    let (mut left, mut right) = (0, n as isize - 1);

    while top <= bottom && left <= right {
        for j in left..=right { 
            grid[top as usize][j as usize] = curr; curr += 1;
        }
        top += 1;
        for i in top..=bottom {
            grid[i as usize][right as usize] = curr; curr += 1;
        }
        right -= 1;
        if top <= bottom {
            for j in (left..=right).rev() {
                grid[bottom as usize][j as usize] = curr; curr += 1;
            }
            bottom -= 1;
        }
        if left <= right {
            for i in (top..=bottom).rev() {
                grid[i as usize][left as usize] = curr; curr += 1;
            }
            left += 1;
        }
    }
    
    let max = (n * n) as u16;
    grid.into_iter().flatten().map(|x| if x == max { 0 } else { x }).collect()
}
