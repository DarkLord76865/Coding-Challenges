fn main() {
    let input = include_str!("../../../input.txt");

    let mut forest: Vec<Vec<u8>> = vec![];
    {
        let mut row: Vec<u8>;
        for line in input.trim().split("\n") {
            row = vec![];
            for tree in line.chars() {
                row.push(tree.to_digit(10).unwrap().try_into().unwrap())
            }
            forest.push(row);
        }
    }

    // part 1
    {
        let mut visible_trees: u32 = 0;
        for x in 0..forest.len() {
            for y in 0..forest[x].len() {
                // to left
                let mut visible_left: bool = true;
                for i in 0..y {
                    if forest[x][i] >= forest[x][y] {
                        visible_left = false;
                        break;
                    }
                }
                if visible_left {
                    visible_trees += 1;
                    continue
                }
                // to right
                let mut visible_right: bool = true;
                for i in (y + 1)..forest[x].len() {
                    if forest[x][i] >= forest[x][y] {
                        visible_right = false;
                        break;
                    }
                }
                if visible_right {
                    visible_trees += 1;
                    continue
                }
                // to top
                let mut visible_top: bool = true;
                for i in 0..x {
                    if forest[i][y] >= forest[x][y] {
                        visible_top = false;
                        break;
                    }
                }
                if visible_top {
                    visible_trees += 1;
                    continue
                }
                // to bot
                let mut visible_bot: bool = true;
                for i in (x + 1)..forest.len() {
                    if forest[i][y] >= forest[x][y] {
                        visible_bot = false;
                        break;
                    }
                }
                if visible_bot {
                    visible_trees += 1;
                    continue
                }
            }
        }

        println!("{}", visible_trees);
    }

    // part 2
    {
        let mut max_scenic_score: u32 = 1;
        for x in 0..forest.len() {
            for y in 0..forest[x].len() {
                let mut scenic_score: u32 = 1;
                // to left
                {
                    let mut visible: u32 = 0;
                    for i in (0..y).rev() {
                        visible += 1;
                        if forest[x][i] >= forest[x][y] {
                            break;
                        }
                    }
                    scenic_score *= visible;
                }
                // to right
                {
                    let mut visible: u32 = 0;
                    for i in (y + 1)..forest[x].len() {
                        visible += 1;
                        if forest[x][i] >= forest[x][y] {
                            break;
                        }
                    }
                    scenic_score *= visible;
                }
                // to top
                {
                    let mut visible: u32 = 0;
                    for i in (0..x).rev() {
                        visible += 1;
                        if forest[i][y] >= forest[x][y] {
                            break;
                        }
                    }
                    scenic_score *= visible;
                }
                // to bot
                {
                    let mut visible: u32 = 0;
                    for i in (x + 1)..forest.len() {
                        visible += 1;
                        if forest[i][y] >= forest[x][y] {
                            break;
                        }
                    }
                    scenic_score *= visible;
                }
                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }
        println!("{}", max_scenic_score);
    }
}
