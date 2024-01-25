// constant variables should have an upper case name
const CONST_INT:i32 = 5;
const CONST_STR:&str = "const_str";

fn main() {
    let let_int = 5;
    let let_str = "let_str";
    println!("let_int has the value {}", let_int);
    println!("let_str has the value {}", let_str);

    println!("const_int has the value {}", CONST_INT);
    println!("const_str has the value {}", CONST_STR);

    let let_int = "let_str";
    let let_str = 10;

    println!("let_int has the value {}", let_int);
    println!("let_str has the value {}", let_str);
}
