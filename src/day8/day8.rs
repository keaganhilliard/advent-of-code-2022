pub fn problem1() {
    let contents = include_str!("input.txt");
    let (columns, rows) = get_columns_and_rows(contents);

    let mut total_visible = 0;
    for column_index in 0..rows.len() {
        let row = &rows[column_index];
        for row_index in 0..row.len() {
            if is_visible(row_index, column_index, &row, &columns[row_index]) {
                total_visible += 1;
            }
        }
    }

    println!("Day 8, Problem 1: {}", total_visible);
}

pub fn problem2() {
    let contents = include_str!("input.txt");
    let (columns, rows) = get_columns_and_rows(contents);

    let mut highest_score = 0;
    for column_index in 0..rows.len() {
        let row = &rows[column_index];
        for row_index in 0..row.len() {
            let scenic_score = get_scenic_score(row_index, column_index, row, &columns[row_index]);
            if scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }
    println!("Day 8, Problem 2: {}", highest_score);
}

fn is_visible(row_index: usize, column_index: usize, row: &Vec<i32>, column: &Vec<i32>) -> bool {
    is_visible_in_vec(row_index, row) || is_visible_in_vec(column_index, column)
}

fn is_visible_in_vec(index: usize, vec: &Vec<i32>) -> bool {
    let tree = vec[index];
    let mut visible_to_beginning = true;
    for compare_tree in vec.iter().take(index).rev() {
        if compare_tree >= &tree {
            visible_to_beginning = false;
        }
    }
    let mut visible_to_end = true;
    for compare_tree in vec.iter().skip(index + 1) {
        if compare_tree >= &tree {
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
    get_vec_scenic_score(row_index, row) * get_vec_scenic_score(column_index, column)
}

fn get_vec_scenic_score(index: usize, vec: &Vec<i32>) -> i32 {
    let tree = vec[index];
    let mut visible_to_beginning = 0;
    for compare_tree in vec.iter().take(index).rev() {
        visible_to_beginning += 1;
        if compare_tree >= &tree {
            break;
        }
    }
    let mut visible_to_end = 0;
    for compare_tree in vec.iter().skip(index + 1) {
        visible_to_end += 1;
        if compare_tree >= &tree {
            break;
        }
    }
    visible_to_end * visible_to_beginning
}

fn get_columns_and_rows(contents: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut columns = Vec::new();
    let mut rows = Vec::new();

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

        rows.push(row_vec);
    }
    (columns, rows)
}
