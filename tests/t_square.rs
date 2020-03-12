use square::*;

#[test]
fn print_square() {
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
fn row_col_sum() {
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
