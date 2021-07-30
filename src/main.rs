mod elementary;
mod lists_strings;

fn main() {
    let mut v = vec![-1,-100,-129,-1003];
    lists_strings::lists::reverse_list(&mut v);
    println!("Borrowing check {:?}", v);

}