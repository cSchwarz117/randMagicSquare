use array2d::Array2D;
extern crate rand;
use rand::Rng;


pub fn fill_five_ez() -> Array2D<i64> {
    let mut two_d = Array2D::filled_with(0, 5, 5);
    let mut m = 0;
    let mut n = 2;

    for x in 1..26 {
        if two_d[(m, n)] != 0 {
            m = m + 2;
            n = n + 4;
        }
        if m > 4 {
            m = m - 5;
        }
        if n > 4 {
            n = n - 5;
        }

        two_d[(m, n)] = x;

        m = m + 4;
        n = n + 1;

        if m > 4 {
            m = m - 5;
        }
        if n > 4 {
            n = n - 5;
        }
    }

    return two_d;
}




pub fn fill_five() -> Array2D<i64> {
    let mut two_d = Array2D::filled_with(0, 5, 5);
    let mut rng = rand::thread_rng();
    let mut m = rng.gen_range(0, 4);
    let mut n = rng.gen_range(0, 4);

    for x in 1..26 {
        if two_d[(m, n)] > 0 {
            m = m + 3;
            n = n + 4;
        }
        if m > 4 {
            m = m - 5;
        }
        if n > 4 {
            n = n - 5;
        }

        two_d[(m, n)] = x;

        m = m + 3;
        n = n + 1;

        if m > 4 {
            m = m - 5;
        }
        if n > 4 {
            n = n - 5;
        }
    }

    return two_d;
}

pub fn fill_seven_ez() -> Array2D<i64> {
    let mut two_d = Array2D::filled_with(0, 7, 7);
    let mut m = 0;
    let mut n = 3;

    for x in 1..50 {
        if two_d[(m, n)] > 0 {
            m = m + 2;
            n = n + 6;
        }
        if m > 6 {
            m = m - 7;
        }
        if n > 6 {
            n = n - 7;
        }

        two_d[(m, n)] = x;

        m = m + 6;
        n = n + 1;

        if m > 6 {
            m = m - 7;
        }
        if n > 6 {
            n = n - 7;
        }
    }

    return two_d;
}

pub fn fill_seven() -> Array2D<i64> {
    let mut two_d = Array2D::filled_with(0, 7, 7);
    let mut rng = rand::thread_rng();
    let mut m = rng.gen_range(0, 6);
    let mut n = rng.gen_range(0, 6);

    for x in 1..50 {
        if two_d[(m, n)] > 0 {
            m = m + 5;
            n = n + 6;
        }
        if m > 6 {
            m = m - 7;
        }
        if n > 6 {
            n = n - 7;
        }

        two_d[(m, n)] = x;

        m = m + 3;
        n = n + 1;

        if m > 6 {
            m = m - 7;
        }
        if n > 6 {
            n = n - 7;
        }
    }

    return two_d;
}

#[cfg(test)]
mod tests {
    use array2d::Array2D;
    #[test]
    fn test_empty() {
        let test = Array2D::filled_with(0, 5, 5);
        assert_eq!(test[(1, 1)], 0);
    }
}
