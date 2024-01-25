fn main() {
    let original_price:i32 = 51;
    println!("Original price: {}", sale_price(original_price));
}

fn sale_price(price:i32) -> i32 {
    if is_even(price) {
        price - 10
    }else {
        price + 10
    }
}

fn is_even(num:i32) -> bool {num % 2 == 0}