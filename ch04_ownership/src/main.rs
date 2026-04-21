fn main() {
    let my_string = String::from("Hello world");

    // 场景 1：找到空格的情况
    let word = first_word(&my_string);
    println!("第一个单词是: {}", word);

    // 场景 2：没有空格的情况
    let my_string2 = "Rust";
    let word2 = first_word(my_string2);
    println!("第一个单词是: {}", word2);
}

/// 编写一个函数，接收一个由空格分隔单词的字符串，并返回它在该字符串中找到的第一个单词。如果函数在该字符串中没有找到空格，那么整个字符串就是一个单词，因此应该返回整个字符串。
/// 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}