pub fn problem1() {
    let contents = include_str!("input.txt");
    let mut columns = Vec::new();
    let mut rows_vec = Vec::new();
    let mut total_visible = 0;

    for row in contents.split("\n") {
        let mut row_vec = Vec::new();
        for (i, c) in row.char_indices() {
            let height = c.to_string().parse::<i32>().unwrap();
            if columns.len() == i {
                columns.push(vec![])
            }
            let column = &mut columns[i];
            column.push(height);
            row_vec.push(height);
        }
        rows_vec.push(row_vec);
    }

    for column_index in 0..rows_vec.len() {
        let row_vec = &rows_vec[column_index];
        for row_index in 0..row_vec.len() {
            if is_visible(row_index, column_index, &row_vec, &columns[row_index]) {
                total_visible += 1;
            }
        }
    }

    println!("Day 8, Problem 1: {}", total_visible);
}

fn is_visible(row_index: usize, column_index: usize, row: &Vec<i32>, column: &Vec<i32>) -> bool {
    is_visible_in_vec(row_index, row) || is_visible_in_vec(column_index, column)
}

fn is_visible_in_vec(index: usize, vec: &Vec<i32>) -> bool {
    let tree = vec[index];
    let mut visible_to_beginning = true;
    for j in 0..index {
        let compare_tree = vec[j];
        if compare_tree >= tree {
            visible_to_beginning = false;
        }
    }
    let mut visible_to_end = true;
    for j in index + 1..vec.len() {
        let compare_tree = vec[j];
        if compare_tree >= tree {
            visible_to_end = false;
        }
    }
    visible_to_end || visible_to_beginning
}

fn get_scenic_score(
    row_index: usize,
    column_index: usize,
    row: &Vec<i32>,
    column: &Vec<i32>,
) -> i32 {
    let row_score = get_vec_scenic_score(row_index, row);
    let column_score = get_vec_scenic_score(column_index, column);
    row_score * column_score
}

fn get_vec_scenic_score(index: usize, vec: &Vec<i32>) -> i32 {
    let tree = vec[index];
    let mut visible_to_beginning = 0;
    for j in 1..=index {
        let compare_tree = vec[index - j];
        visible_to_beginning += 1;
        if compare_tree >= tree {
            break;
        }
    }
    let mut visible_to_end = 0;
    for j in index + 1..vec.len() {
        let compare_tree = vec[j];
        visible_to_end += 1;
        if compare_tree >= tree {
            break;
        }
    }
    visible_to_end * visible_to_beginning
}

pub fn problem2() {
    let contents = include_str!("input.txt");
    let mut columns = Vec::new();
    let mut rows_vec = Vec::new();
    let mut highest_score = 0;

    for row in contents.split("\n") {
        let mut row_vec = Vec::new();
        for (i, c) in row.char_indices() {
            let height = c.to_string().parse::<i32>().unwrap();
            if columns.len() == i {
                columns.push(vec![])
            }
            let column = &mut columns[i];
            column.push(height);
            row_vec.push(height);
        }

        rows_vec.push(row_vec);
    }

    for column_index in 0..rows_vec.len() {
        let row_vec = &rows_vec[column_index];
        for row_index in 0..row_vec.len() {
            let scenic_score =
                get_scenic_score(row_index, column_index, row_vec, &columns[row_index]);
            if scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }
    println!("Day 8, Problem 2: {}", highest_score);
}
