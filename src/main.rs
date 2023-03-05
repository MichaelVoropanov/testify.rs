#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_imports)]
#[allow(dead_code)]
use std::io;

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:#?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        println!("{:#?}", item);
        println!("{:#?}", i);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{:#?}", hello);
    println!("{:#?}", world);

}
