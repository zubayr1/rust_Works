use crate::iteration::{print_elements, print_elements_adapt};

mod iteration;

fn main() {
    let mut colors: Vec<String> = Vec::new();

    colors.push("white".to_string());
    colors.push("red".to_string());
    colors.push("green".to_string());
    colors.push("blue".to_string());

    print_elements(&colors[1..4]);
    print_elements_adapt(&colors[1..4]);

    let mut colors_iter = colors.iter();

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());

    println!("{:#?}", colors_iter);

    println!("{:#?}", colors_iter.next());

    match colors_iter.next() {
        Some(val) => {
            println!("{}", val);
        }
        None => {
            println!("none exists");
        }
    }
}
