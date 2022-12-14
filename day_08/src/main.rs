use std::fs;

fn is_tree_visible(heights: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize) -> bool {
    let tree_height = heights[tree_y][tree_x];

    // check North direction
    let mut max_height_north = 0;
    for y in 0..tree_y {
        let height = heights[y][tree_x];

        if height > max_height_north {
            max_height_north = height;
        }
    }

    // check South direction
    let mut max_height_south = 0;
    for y in tree_y + 1..heights.len() {
        let height = heights[y][tree_x];

        if height > max_height_south {
            max_height_south = height;
        }
    }

    // check West direction
    let row = &heights[tree_y];
    let mut max_height_west = 0;
    for x in 0..tree_x {
        let height = heights[tree_y][x];

        if height > max_height_west {
            max_height_west = height;
        }
    }

    // check East direction
    let mut max_height_east = 0;
    for x in tree_x + 1..row.len() {
        let height = heights[tree_y][x];

        if height > max_height_east {
            max_height_east = height;
        }
    }

    max_height_north < tree_height
        || max_height_east < tree_height
        || max_height_south < tree_height
        || max_height_west < tree_height
}

fn calc_scenic_score(heights: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize) -> usize {
    let tree_height: usize = heights[tree_y][tree_x] as usize;

    // check North direction
    let mut n_visible_trees: usize = 0;
    for y in (0..tree_y).rev() {
        let height = heights[y][tree_x] as usize;

        n_visible_trees += 1;

        if height >= tree_height {
            // stop here, cant see further than this
            break;
        }
    }
    let scenic_score_north = n_visible_trees;

    // check South direction
    let mut n_visible_trees: usize = 0;
    for y in tree_y + 1..heights.len() {
        let height = heights[y][tree_x] as usize;

        n_visible_trees += 1;

        if height >= tree_height {
            // stop here, cant see further than this
            break;
        }
    }
    let scenic_score_south = n_visible_trees;

    // check West direction
    let mut n_visible_trees: usize = 0;
    for x in (0..=tree_x - 1).rev() {
        let height = heights[tree_y][x] as usize;

        n_visible_trees += 1;

        if height >= tree_height {
            // stop here, cant see further than this
            break;
        }
    }
    let scenic_score_west = n_visible_trees;

    // check East direction
    let row = &heights[tree_y];
    let mut n_visible_trees: usize = 0;
    for x in tree_x + 1..row.len() {
        let height = heights[tree_y][x] as usize;

        n_visible_trees += 1;

        if height >= tree_height {
            // stop here, cant see further than this
            break;
        }
    }
    let scenic_score_east = n_visible_trees;

    // dbg!(scenic_score_north);
    // dbg!(scenic_score_east);
    // dbg!(scenic_score_south);
    // dbg!(scenic_score_west);

    let scenic_score =
        scenic_score_north * scenic_score_east * scenic_score_south * scenic_score_west;

    if scenic_score == 1560900 {
        println!(
            "score = {scenic_score} = {scenic_score_west} * {scenic_score_east} * {scenic_score_north} * {scenic_score_south} at ({tree_x}, {tree_y})"
        );
    }
    scenic_score
}

fn convert_input_lines(lines: &Vec<&str>) -> Vec<Vec<u8>> {
    let mut heights: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        let mut this_line = Vec::new();
        for c in line.as_bytes() {
            let i: u8 = c - 48u8;

            this_line.push(i);
        }

        heights.push(this_line);
    }

    heights
}
fn main() {
    let lines = vec!["30373", "25512", "65332", "33549", "35390"];
    let heights = convert_input_lines(&lines);
    let scenic_score = calc_scenic_score(&heights, 2, 2);
    println!("scenic_score: {scenic_score}");

    let scenic_score = calc_scenic_score(&heights, 2, 3);
    println!("scenic_score: {scenic_score}");

    let input_file_content = fs::read_to_string("input.txt").unwrap();
    let lines = input_file_content.lines().collect();

    let heights: Vec<Vec<u8>> = convert_input_lines(&lines);

    // println!("Lines:\n{:?}", heights);

    let mut n_visible_trees = 0;

    // dont check the trees on the borders of the grid, they are always visible
    for y in 1..heights.len() - 1 {
        let row = &heights[y];
        for x in 1..row.len() - 1 {
            // let height = heights[y][x];
            let is_visible = is_tree_visible(&heights, x, y);

            // println!("Tree with height {height} at ({x}, {y}) is visible: {is_visible}");

            if is_visible {
                n_visible_trees += 1;
            }
        }
    }

    // add the number of trees that are visible on the borders (all of them)
    let n_border_trees_visible =
        heights[0].len() + heights[0].len() + heights.len() - 2 + heights.len() - 2;
    dbg!(n_visible_trees);
    dbg!(n_border_trees_visible);
    n_visible_trees += n_border_trees_visible;

    let n_trees = heights.len() * heights[0].len();

    println!("Out of the {n_trees} trees, {n_visible_trees} are visible.");

    let mut max_scenic_score = 0;
    // dont check the trees on the borders of the grid, they are always visible
    for y in 1..heights.len() - 1 {
        let row = &heights[y];
        for x in 1..row.len() - 1 {
            let scenic_score = calc_scenic_score(&heights, x, y);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("Max Scenic Score: {max_scenic_score}");
}

#[cfg(test)]
mod tests {
    use crate::calc_scenic_score;
    use crate::convert_input_lines;
    use crate::is_tree_visible;
    use test_case::test_case;

    #[test_case(1, 1, true)]
    #[test_case(2, 1, true)]
    #[test_case(3, 1, false)]
    #[test_case(1, 2, true)]
    #[test_case(2, 2, false)]
    #[test_case(3, 2, true)]
    #[test_case(1, 3, false)]
    #[test_case(2, 3, true)]
    #[test_case(3, 3, false)]
    fn test_is_tree_visibile_at(tree_x: usize, tree_y: usize, should_be_visible: bool) {
        let lines = vec!["30373", "25512", "65332", "33549", "35390"];

        let heights: Vec<Vec<u8>> = convert_input_lines(&lines);

        let is_visible = is_tree_visible(&heights, tree_x, tree_y);

        assert_eq!(should_be_visible, is_visible);
    }

    #[test_case(1, 1, 1*1*1*1)]
    #[test_case(2, 1, 1*2*2*1)]
    #[test_case(3, 1, 1*1*1*1)]
    #[test_case(1, 2, 1*3*2*1)]
    #[test_case(2, 2, 1*1*1*1)]
    #[test_case(3, 2, 2*1*1*1)]
    #[test_case(1, 3, 1*1*1*1)]
    #[test_case(2, 3, 2*2*1*2)]
    #[test_case(3, 3, 3*1*1*1)]
    fn test_calc_scenic_score(tree_x: usize, tree_y: usize, expected_scenic_score: usize) {
        let lines = vec!["30373", "25512", "65332", "33549", "35390"];

        let heights: Vec<Vec<u8>> = convert_input_lines(&lines);

        let scenic_score = calc_scenic_score(&heights, tree_x, tree_y);

        assert_eq!(expected_scenic_score, scenic_score);
    }
}
