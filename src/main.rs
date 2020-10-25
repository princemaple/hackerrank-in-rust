mod utils;

fn get_input() -> Result<(i32, i32, Vec<Vec<i32>>), String> {
    let (h, w) = utils::get_vals_as_pair::<i32>()?;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..h {
        matrix.push(utils::get_vals_as_vec())
    }
    Ok((h, w, matrix))
}

fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed_matrix: Vec<Vec<i32>> = Vec::new();
    let len = matrix.first().unwrap().len();
    for c in 0..len {
        let mut row: Vec<i32> = Vec::new();
        for r in matrix.iter() {
            row.push(r[c]);
        }
        transposed_matrix.push(row);
    }
    transposed_matrix
}

fn calc_side(matrix: Vec<Vec<i32>>) -> i32 {
    matrix
        .iter()
        .map(|row| {
            Some(row.iter().fold(
                (row.first()? + row.last()?, row.first()?),
                |(sum, prev), curr| ((sum + (curr - prev).abs()), curr),
            ))
        })
        .fold(0, |acc, item| match item {
            Some((sum, _)) => acc + sum,
            None => panic!("not possible"),
        })
}

fn main() -> Result<(), String> {
    let (h, w, matrix) = get_input()?;
    let transposed_matrix = transpose(&matrix);
    // println!("{:?}", (h, w));
    // println!("{}", calc_side(matrix));
    // println!("{}", calc_side(transposed_matrix));
    println!(
        "{}",
        h * w * 2 + calc_side(matrix) + calc_side(transposed_matrix)
    );

    Ok(())
}
