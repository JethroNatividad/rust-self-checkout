use std::io;
use std::io::Write;

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

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    const TAX_PERCENTAGE: f64 = 5.5; // 5.5%
    let mut subtotal: f64 = 0.0;
    for i in 1..4 {
        let price: f64 = get_input(&format!("Enter the price of item {}: ", i));
        let qty: f64 = get_input(&format!("Enter the quantity of item {}: ", i));

        subtotal += price * qty;
    }

    let tax: f64 = calculate_tax(subtotal, TAX_PERCENTAGE);
    let total: f64 = subtotal + tax;

    println!("Subtotal: ${}", subtotal);
    println!("Tax: ${}", tax);
    println!("Total: ${}", total);
}
