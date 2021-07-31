mod elementary;
mod lists_strings;

fn main() {
    
    let palindrome_test = String::from("nurses run");

    let is_pal = lists_strings::strings::is_palindrome(&palindrome_test);

    println!("Is {} a palindrome? {} ", palindrome_test, is_pal );

}

