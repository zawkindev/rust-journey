fn main() {
    let s = "hello world";
    let first_word = &s[0..5];
    let seocond_word = &s[7..];
    println!("{}\n{}", first_word, seocond_word);

    let literal: &str = "hello Megumi KATOU";
    println!("literal: {}", literal);

    let str_from: String = String::from("hello Asuna");
    println!("str from: {}", str_from);

    let mut str_new: String = String::new();
    str_new.push_str("helloooo, ZeroTWO");
    println!("{}", str_new);

    let waifu: &str = "waifu";
    let str_format: String = format!("hello, {}", waifu);
    println!("{}", str_format);

    let str_to_str: String = "hello, world".to_string();
    println!("{}", str_to_str);
}
