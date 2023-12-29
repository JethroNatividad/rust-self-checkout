// Create a simple self-checkout system. Prompt for the prices
// and quantities of three items. Calculate the subtotal of the
// items. Then calculate the tax using a tax rate of 5.5%. Print
// out the line items with the quantity and total, and then print
// out the subtotal, tax amount, and total.


// Checkout system that calculates the subtotal, tax, and total amount of three items.
// Inputs: item n, price and quantity
// Process: total the prices, tax, and grand total.
// Output: Subtotal, Tax, Total.

fn calculate_tax(subtotal: f64) -> f64 {
    const TAX_RATE: i64 = 0.055; // 5.5%
    subtotal * TAX_RATE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tax(){
        assert_eq!(calculate_tax(64), 3.52)
        assert_eq!(calculate_tax(55), 3.03)
        assert_eq!(calculate_tax(120), 6.6)
        assert_eq!(calculate_tax(89), 4.9)
    }
}


fn main() {
    println!("Hello, world!");
}
