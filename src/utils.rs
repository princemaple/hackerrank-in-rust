use std::str::FromStr;

const ERR_READ: &'static str = "Read Error";
const ERR_INPUT_INVALID: &'static str = "Invalid Input Error";
const ERR_MATRIX_EMPTY: &'static str = "Unexpected Empty Matrix Error";

fn readln() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .ok()
        .expect(ERR_READ);
    buffer.trim().into()
}

#[allow(dead_code)]
pub fn get_val<T: FromStr>() -> T {
    readln().parse::<T>().ok().expect(ERR_INPUT_INVALID)
}

#[allow(dead_code)]
pub fn get_vals_as_vec<T: FromStr>() -> Vec<T> {
    readln()
        .split_whitespace()
        .map(|n| n.parse::<T>().ok().expect(ERR_INPUT_INVALID))
        .collect()
}

#[allow(dead_code)]
pub fn get_vals_as_pair<T: FromStr + Clone>() -> Result<(T, T), String> {
    let vals = get_vals_as_vec::<T>();

    match vals.len() {
        2 => Ok((vals[0].clone(), vals[1].clone())),
        _ => Err(ERR_INPUT_INVALID.to_string()),
    }
}

#[allow(dead_code)]
pub fn get_vals_as_triplet<T: FromStr + Clone>() -> Result<(T, T, T), String> {
    let vals = get_vals_as_vec::<T>();
    match vals.len() {
        3 => Ok((vals[0].clone(), vals[1].clone(), vals[2].clone())),
        _ => Err(ERR_INPUT_INVALID.to_string()),
    }
}

#[allow(dead_code)]
pub fn get_vals_as_matrix<T: FromStr + Clone>(r: usize, c: usize) -> Result<Vec<Vec<T>>, String> {
    let mut matrix = Vec::<Vec<T>>::with_capacity(r);
    for _ in 0..r {
        matrix.push(get_vals_as_vec::<T>());
        match matrix.last() {
            Some(row) => match row.len() != c {
                true => return Err(ERR_INPUT_INVALID.to_string()),
                false => (),
            },
            None => return Err(ERR_MATRIX_EMPTY.to_string()),
        }
    }
    Ok(matrix)
}
