use pyo3::prelude::*;

const HEADER: &str = " Func-count     x          f(x)          Procedure";

// Ported from scipy.optimize._optimize._minimize_scalar_bounded: https://github.com/scipy/scipy/blob/d98afeae0fcb2302d0f25ba561e7984d5fadf65c/scipy/optimize/_optimize.py
fn bounded_minimisation(
    func: impl Fn(f64) -> f64,
    lower_bound: f64,
    upper_bound: f64,
    verbose: bool,
) -> f64 {
    let golden_mean = 0.5 * (3.0 - 5.0_f64.sqrt());
    let sqrt_eps = (2.2e-16_f64).sqrt();

    let xatol = 1e-5;
    let maxfun: u64 = 500;
    let x1 = lower_bound;
    let x2 = upper_bound;

    let mut flag = 0;
    let mut step = "       initial";

    let mut a = x1;
    let mut b = x2;
    let mut fulc = a + golden_mean * (b - a);

    let mut rat = 0.0;
    let mut e: f64 = 0.0;

    let mut nfc = fulc;
    let mut xf = fulc;

    let mut x = xf;
    let mut fx = func(x);
    let mut num = 1;
    let mut fu = f64::INFINITY;

    let mut ffulc = fx;
    let mut fnfc = fx;

    let mut xm = 0.5 * (a + b);
    let mut tol1 = sqrt_eps * xf.abs() + xatol / 3.0;
    let mut tol2 = 2.0 * tol1;

    if verbose {
        println!(" ");
        println!("{HEADER}");
        println!("{0:5.0}   {xf:12.6} {fx:12.6} {step}", 1);
    }

    while (xf - xm).abs() > (tol2 - 0.5 * (b - a)) {
        let mut golden = true;
        if e.abs() > tol1 {
            golden = false;
            let mut r = (xf - nfc) * (fx - ffulc);
            let q = (xf - fulc) * (fx - fnfc);
            let mut p = (xf - fulc) * q - (xf - nfc) * r;
            let mut q = 2.0 * (q - r);
            if q > 0.0 {
                p = -p;
            }
            q = q.abs();
            r = e;
            e = rat;

            if (p.abs() < (0.5 * q * r).abs()) && (p > q * (a - xf)) && (p < q * (b - xf)) {
                rat = (p + 0.0) / q;
                x = xf + rat;
                step = "       parabolic";
                if ((x - a) < tol2) || ((b - x) < tol2) {
                    let si = (xm - xf).signum() + f64::from((xm - xf) == 0.0);
                    rat = tol1 * si;
                }
            } else {
                golden = true;
            }
        }

        if golden {
            if xf >= xm {
                e = a - xf;
            } else {
                e = b - xf;
            }
            rat = golden_mean * e;
            step = "       golden";
        }

        let si = rat.signum() + f64::from(rat == 0.0);
        x = xf + si * f64::max(rat.abs(), tol1);
        fu = func(x);
        num += 1;
        if verbose {
            println!("{num:5.0}   {x:12.6} {fu:12.6} {step}");
        }

        if fu <= fx {
            if x >= xf {
                a = xf;
            } else {
                b = xf;
            }
            fulc = nfc;
            ffulc = fnfc;
            nfc = xf;
            fnfc = fx;
            xf = x;
            fx = fu;
        } else {
            if x < xf {
                a = x;
            } else {
                b = x;
            }
            if (fu <= fnfc) || (nfc == xf) {
                fulc = nfc;
                ffulc = fnfc;
                nfc = x;
                fnfc = fu;
            } else if (fu <= ffulc) || (fulc == xf) || (fulc == nfc) {
                fulc = x;
                ffulc = fu;
            }
        }

        xm = 0.5 * (a + b);
        tol1 = sqrt_eps * xf.abs() + xatol / 3.0;
        tol2 = 2.0 * tol1;

        if num >= maxfun {
            flag = 1;
            break;
        }
    }

    if xf.is_nan() || fx.is_nan() || fu.is_nan() {
        flag = 2;
    }

    let fval = fx;

    // TODO: _endprint(x, flag, fval, maxfun, xatol, disp)

    // TODO: Improve return value
    x
}

#[test]
fn example1() {
    assert_eq!(
        bounded_minimisation(|x| (x - 1.0) * (x - 1.0), -4.0, 4.0, true),
        1.0
    );
}

#[test]
fn example2() {
    assert_eq!(
        bounded_minimisation(|x| (x - 1.0) * (x - 1.0), 3.0, 4.0, true),
        3.0
    );
}

fn calculate_log_expected_wealth(
    stake: f64,
    price: f64,
    is_back: bool,
    probability: f64,
    other_probabilities: &Vec<f64>,
    position: f64,
    other_positions: &Vec<f64>,
    bankroll: f64,
) -> f64 {
    let expected_log_wealth = if is_back {
        probability * (bankroll + position + stake * (price - 1.0)).ln()
            + (other_positions
                .iter()
                .zip(other_probabilities.iter())
                .map(|(other_position, other_probability)| {
                    other_probability * (bankroll + other_position - stake).ln()
                })
                .sum::<f64>())
    } else {
        probability * (bankroll + position - stake * (price - 1.0)).ln()
            + (other_positions
                .iter()
                .zip(other_probabilities.iter())
                .map(|(other_position, other_probability)| {
                    other_probability * (bankroll + other_position + stake).ln()
                })
                .sum::<f64>())
    };

    expected_log_wealth
}

#[pyfunction]
#[pyo3(signature = (price, is_back, probability, other_probabilities, position, other_positions, bankroll, kelly_fraction, verbose = false))]
fn calculate_kelly_stake(
    price: f64,
    is_back: bool,
    probability: f64,
    other_probabilities: Vec<f64>,
    position: f64,
    other_positions: Vec<f64>,
    bankroll: f64,
    kelly_fraction: f64,
    verbose: bool,
) -> PyResult<f64> {
    let kelly_stake = bounded_minimisation(
        |stake| {
            -calculate_log_expected_wealth(
                stake,
                price,
                is_back,
                probability,
                &other_probabilities,
                position,
                &other_positions,
                bankroll,
            )
        },
        0.0,
        bankroll,
        verbose,
    );
    Ok(kelly_stake)
}

/// Fast Kelly staking calculations for a range of scenarios .
#[pymodule]
fn kelly(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_kelly_stake, m)?)?;
    Ok(())
}
