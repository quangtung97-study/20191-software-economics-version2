use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

const N: usize = 3;

#[allow(dead_code)]
pub fn simple_newton_method(f: impl Fn(f64) -> f64, df: impl Fn(f64) -> f64, x0: f64) -> f64 {
    let mut x = x0;
    for _ in 0..N {
        x = x - f(x) / df(x);
    }
    return x;
}

pub fn jacobi(
    f: &impl Fn(&Array1<f64>) -> Array1<f64>,
    x0: &Array1<f64>,
    dx0: &Array1<f64>,
) -> Array2<f64> {
    let len = x0.len();
    let mut jmatrix = Array2::zeros((len, len));
    for col in 0..len {
        let delta = dx0[col];
        let mut x = x0.clone();
        x[col] += delta;
        let df_col = (f(&x) - f(x0)) / delta;

        for row in 0..len {
            jmatrix[(row, col)] = df_col[row];
        }
    }

    jmatrix
}

#[allow(dead_code)]
pub fn newton_method(
    f: &impl Fn(&Array1<f64>) -> Array1<f64>,
    x0: &Array1<f64>,
    dx0: &Array1<f64>,
) -> Option<Array1<f64>> {
    let mut x = x0.clone();
    for _ in 0..N {
        let minus_fx = -f(&x);
        let jx = jacobi(f, &x, dx0);
        let dx = match jx.solve_into(minus_fx) {
            Ok(m) => m,
            Err(_) => return None,
        };
        x = x + dx;

        // for e in x.iter_mut() {
        //     if *e < 0.0 {
        //         *e = 0.0;
        //     }
        // }
    }

    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use ndarray::arr1;

    #[test]
    fn test_simple_newton_method() {
        let x = simple_newton_method(|x: f64| 2.0 * x - 4.0, |_: f64| 2.0, 4.0);
        assert_approx_eq!(x, 2.0);

        let f = |x| x * x + 2.0 * x - 3.0;
        let df = |x| 2.0 * x + 2.0;

        let x = simple_newton_method(f, df, 10.0);
        assert_approx_eq!(x, 1.0);

        let x = simple_newton_method(f, df, -10.0);
        assert_approx_eq!(x, -3.0);
    }

    #[test]
    fn test_newton_method() {
        let f = |x: &Array1<f64>| arr1(&[2.0 * x[0] + x[1] - 5.0, 4.0 * x[0] - 3.0 * x[1] + 5.0]);
        let x0 = arr1(&[10.0, 10.0]);
        let dx0 = arr1(&[0.000001, 0.000001]);
        let x = newton_method(&f, &x0, &dx0).unwrap();
        assert_approx_eq!(x[0], 1.0);
        assert_approx_eq!(x[1], 3.0);
    }

    #[test]
    fn test_jacobi() {
        let f =
            |x: &Array1<f64>| arr1(&[2.0 * x[0] * x[0] + 3.0 * x[1], -4.0 * x[0] + x[1] * x[1]]);
        let x0 = arr1(&[1.0, 1.0]);
        let dx0 = arr1(&[0.000001, 0.000001]);
        let j = jacobi(&f, &x0, &dx0);

        assert_approx_eq!(j[(0, 0)], 4.0, 0.0001);
        assert_approx_eq!(j[(0, 1)], 3.0, 0.0001);
        assert_approx_eq!(j[(1, 0)], -4.0, 0.0001);
        assert_approx_eq!(j[(1, 1)], 2.0, 0.0001);
    }
}
