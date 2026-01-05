// See README.md for exercise instructions
mod formula;
use formula::calculate_item_total::*;

fn main() {
    let fruits = vec![("apple", 15.5, 5), ("banana", 8.75, 8)];

    let mut grand_total = 0.0;
    for (name, price, qty) in fruits {
        let total = calculate_item_total(price, qty);
        println!("{}: ${:.2}", name, total);
        grand_total += total;
    }
    println!("Grand Total: ${:.2}", grand_total);
}
