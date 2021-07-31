
pub fn is_palindrome(phrase: &str) -> bool{

    // Make string slice a mutable string
    let mut phrase = phrase.trim().to_owned();
    
    remove_whitespace(&mut phrase);

    // Compare the bytes of the two halves and compare
    phrase.bytes().take(phrase.len()/2).eq(phrase.bytes().rev().take(phrase.len()/2))
}

fn remove_whitespace(s: &mut String){
    // retains all characters in string that are NOT whitespace
    s.retain(|c| !c.is_whitespace());
}