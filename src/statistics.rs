use num::Num;
use num::cast::FromPrimitive;
use std::cmp::Ordering;

fn mean<T: Num + Copy + FromPrimitive>(v: &[T]) -> Option<T> {
    let sum = v.iter().fold(T::zero(), |a, b| a + *b);
    // can this meaningfully fail?
    T::from_usize(v.len()).map(|count| if count == T::zero() { sum } else { sum / count })
}

// sorts v
fn median<T: Num + Copy + FromPrimitive + PartialOrd>(v: &mut [T]) -> Option<T> {
    let n = v.len();
    if n == 0 {
        None
    } else {
        let mid = n / 2;
        let mut found_nan = false;
        v.sort_by(|a, b| {
            a.partial_cmp(b).unwrap_or_else(|| {
                found_nan = true;
                Ordering::Equal
            })
        });
        if found_nan {
            None
        } else if n % 2 == 1 {
            Some(v[mid])
        } else {
            Some(
                (v[mid] + v[mid - 1]) / T::from_i32(2).expect("divisible by 2"),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&vec![]), Some(0));
        assert_eq!(mean(&vec![0, 1, 2, 3]), Some(6 / 4));
        assert_eq!(mean(&vec![0.0, 1.0, 2.0, 3.0, 4.0]), Some(10.0 / 5.0));
    }

    #[test]
    fn test_median() {
        let mut empty = Vec::<i32>::new();
        assert_eq!(median(&mut empty), None);
        assert_eq!(median(&mut vec![1]), Some(1));
        assert_eq!(median(&mut vec![1.0, 2.0]), Some(1.5));
        assert_eq!(median(&mut vec![1.0, 2.0, 666.0]), Some(2.0));
        assert_eq!(median(&mut vec![1.0, f64::NAN, 666.0]), None);
    }
}
