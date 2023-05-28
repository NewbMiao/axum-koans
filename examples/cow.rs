use std::borrow::Cow;

fn string_return(input: &str) -> Cow<str> {
    if input.len() > 10 {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(format!("only {} characters", input.len()))
    }
}

fn main() {
    let borrowed_string = "A critically acclaimed western-themed game that became the best-selling title on the Nintendo Switch platform, with over 33 million copies sold worldwide as of September 2021.";
    let owned_string = "short";

    let borrowed = string_return(borrowed_string);
    let owned = string_return(owned_string);

    println!("{}", borrowed);
    println!("{}", owned);
    println!("{}", borrowed);
    println!("{}", owned);
    println!("{}", owned);
}
