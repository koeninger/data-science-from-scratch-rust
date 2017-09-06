use num::{Float, Num};

fn vector_add<T: Num + Copy>(v: &[T], w: &[T]) -> Vec<T> {
    v.iter().zip(w).map(|(a, b)| *a + *b).collect()
}

fn vector_subtract<T: Num + Copy>(v: &[T], w: &[T]) -> Vec<T> {
    v.iter().zip(w).map(|(a, b)| *a - *b).collect()
}

fn reduce<T, F>(vs: &[Vec<T>], mut f: F) -> Option<Vec<T>>
where
    T: Num + Copy,
    F: FnMut(&[T], &[T]) -> Vec<T>,
{
    match vs.len() {
        0 => None,
        // it's a little silly to clone this, could maybe return None?
        1 => Some(vs[0].clone()),
        2 => Some(f(&vs[0], &vs[1])),
        _ => {
            let mut accum = f(&vs[0], &vs[1]);
            for v in vs[2..].iter() {
                accum = f(&accum, v);
            }
            Some(accum)
        }
    }
}

fn vector_sum<T: Num + Copy>(vs: &[Vec<T>]) -> Option<Vec<T>> {
    reduce(vs, vector_add)
}

fn scalar_multiply<T: Num + Copy>(c: T, v: &[T]) -> Vec<T> {
    v.iter().map(|x| *x * c).collect()
}

/// vector whose ith element is mean of the ith element of all vs
fn vector_mean(vs: &[Vec<f64>]) -> Option<Vec<f64>> {
    // it's this usize to float conversion that led me to fix this as f64 rather than T
    let n = vs.len() as f64;
    vector_sum(vs).map(|s| scalar_multiply(1.0 / n, &s))
}

fn dot<T: Num + Copy>(v: &[T], w: &[T]) -> T {
    v.iter()
        .zip(w)
        .map(|(a, b)| *a * *b)
        .fold(T::zero(), |acc, i| acc + i)
}

fn sum_of_squares<T: Num + Copy>(v: &[T]) -> T {
    dot(v, v)
}

fn magnitude<T: Num + Copy + Float>(v: &[T]) -> T {
    sum_of_squares(v).sqrt()
}

fn distance<T: Num + Copy + Float>(v: &[T], w: &[T]) -> T {
    magnitude(&vector_subtract(v, w))
}

fn shape<T: Num + Copy>(m: &[Vec<T>]) -> (usize, usize) {
    let num_rows = m.len();
    let num_cols = if num_rows > 0 { m[0].len() } else { 0 };
    (num_rows, num_cols)
}

fn get_row<T: Num + Copy>(m: &[Vec<T>], i: usize) -> &Vec<T> {
    &m[i]
}

fn get_column<T: Num + Copy>(m: &[Vec<T>], i: usize) -> Vec<T> {
    m.iter().map(|r| r[i]).collect()
}

fn make_matrix<T: Num + Copy, F: FnMut(usize, usize) -> T>(
    num_rows: usize,
    num_cols: usize,
    mut entry_fn: F,
) -> Vec<Vec<T>> {
    (0..num_rows)
        .map(|r| (0..num_cols).map(|c| entry_fn(r, c)).collect())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(vector_add(&vec![0, 1, 2], &vec![1, 2, 3]), vec![1, 3, 5]);
        assert_eq!(
            vector_add(&vec![0.0, 2.0, 4.0], &vec![1.0, 2.0, 3.0]),
            vec![1.0, 4.0, 7.0]
        );
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            vector_subtract(&vec![0, 1, 2], &vec![1, 2, 3]),
            vec![-1, -1, -1]
        );
        assert_eq!(
            vector_subtract(&vec![0.0, 2.0, 4.0], &vec![1.0, 2.0, 3.0]),
            vec![-1.0, 0.0, 1.0]
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(vector_sum(&(Vec::<Vec<i32>>::new())), None);
        assert_eq!(vector_sum(&vec![vec![0, 1, 2]]), Some(vec![0, 1, 2]));
        assert_eq!(
            vector_sum(&vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![2, 3, 4, 5, 5],
            ]),
            Some(vec![3, 6, 9, 12])
        )

    }

    #[test]
    fn test_scalar_multiply() {
        assert_eq!(scalar_multiply(3, &vec![0, 1, 2, 3]), vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_mean() {
        assert_eq!(
            vector_sum(&vec![vec![0.0_f64, 2.0, 4.0], vec![1.0, 3.0, 8.0]]),
            Some(vec![1.0, 5.0, 12.0])
        );
        assert_eq!(
            vector_mean(&vec![vec![0.0_f64, 2.0, 4.0], vec![1.0, 3.0, 8.0]]),
            Some(vec![0.5, 2.5, 6.0])
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&vec![1, 2, 3], &vec![4, 5, 6]), 32);
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]), 0.0);
    }

    #[test]
    fn test_shape() {
        assert_eq!(shape(&Vec::<Vec<i32>>::new()), (0, 0));
        assert_eq!(shape(&vec![vec![0, 1, 2], vec![0, 1, 2]]), (2, 3));
    }

    #[test]
    fn test_get_row_col() {
        let m = &vec![vec![0, 1, 2], vec![4, 5, 6]];
        assert_eq!(get_row(m, 1), &vec![4, 5, 6]);
        assert_eq!(get_column(m, 2), vec![2, 6]);
    }

    #[test]
    fn test_make_matrix() {
        assert_eq!(
            make_matrix(2, 3, |r, c| r + c),
            vec![vec![0, 1, 2], vec![1, 2, 3]]
        );
    }
}
