// Create a simple self-checkout system. Prompt for the prices
// and quantities of three items. Calculate the subtotal of the
// items. Then calculate the tax using a tax rate of 5.5%. Print
// out the line items with the quantity and total, and then print
// out the subtotal, tax amount, and total.


// Checkout system that calculates the subtotal, tax, and total amount of three items.
// Inputs: item n, price and quantity
// Process: total the prices, tax, and grand total.
// Output: Subtotal, Tax, Total.

fn round_decimal(number: f64) -> f64 {
    (number * 1000.0).round() / 1000.0
}

fn calculate_tax(subtotal: f64, tax_percentage: f64) -> f64 {
    round_decimal(subtotal * (tax_percentage / 100.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_decimal() {
        assert_eq!(round_decimal(27.870912), 27.871);
        assert_eq!(round_decimal(2.3225), 2.323);
        assert_eq!(round_decimal(4.895), 4.895);
    }

    #[test]
    fn test_calculate_tax(){
        assert_eq!(calculate_tax(64.0, 5.5), 3.52);
        assert_eq!(calculate_tax(55.0, 5.5), 3.025);
        assert_eq!(calculate_tax(120.0, 5.5), 6.6);
        assert_eq!(calculate_tax(89.0, 5.5), 4.895);
    }
}

use std::io;
use std::io::Write;

fn main() {
    const TAX_PERCENTAGE: f64 = 5.5; // 5.5%
    let mut subtotal: f64 = 0.0;
    for i in 1..4 {
        print!("Enter the price of item {}: ", i);
        let mut price = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut price).expect("Failed to read input");
        let price: f64 = price.trim().parse().expect("Please enter a valid number");

        print!("Enter the quantity of item {}: ", i);
        let mut qty = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut qty).expect("Failed to read input");
        let qty: f64 = qty.trim().parse().expect("Please enter a valid number");

        subtotal += price * qty;
    }

    let tax: f64 = calculate_tax(subtotal, TAX_PERCENTAGE);
    let total: f64 = subtotal + tax;

    println!("Subtotal: ${}", subtotal);
    println!("Tax: ${}", tax);
    println!("Total: ${}", total);
}
