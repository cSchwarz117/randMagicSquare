use square::*;

#[test]
fn print_square_five() {
    let filled = fill_five();
    println!("All elements:");
    for row_iter in filled.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }
}
#[test]
fn print_square_seven() {
    let filled = fill_seven();
    println!("All elements:");
    for row_iter in filled.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }
}


#[test]
fn row_col_sum_five() {
    let check = fill_five();
    let mut sum = 0;
    for x in 0..5 {
        for y in 0..5 {
            sum += check[(x, y)];
        }
        assert_eq!(65, sum);
        sum = 0;
    }

    for x in 0..5 {
        for y in 0..5 {
            sum += check[(y, x)];
        }
        assert_eq!(65, sum);
        sum = 0;
    }
}

#[test]
fn row_col_sum_seven() {
    let check = fill_seven();
    let mut sum = 0;
    for x in 0..7 {
        for y in 0..7 {
            sum += check[(x, y)];
        }
        assert_eq!(175, sum);
        sum = 0;
    }

    for x in 0..7 {
        for y in 0..7 {
            sum += check[(y, x)];
        }
        assert_eq!(175, sum);
        sum = 0;
    }
}
