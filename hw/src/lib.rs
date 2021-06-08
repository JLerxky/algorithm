fn main() {
    println!("Hello, HW!");
}

/// # 字符串最后一个单词的长度
/// ## 描述
/// 计算字符串最后一个单词的长度，单词以空格隔开，字符串长度小于5000。
/// ## 输入描述：
/// 输入一行，代表要计算的字符串，非空，长度小于5000。
/// ## 输出描述：
/// 输出一个整数，表示输入字符串最后一个单词的长度。
/// ## 示例
/// ```
/// 输入： hello nowcoder
/// 输出： 8
/// 说明： 最后一个单词为nowcoder，长度为8
/// ```
fn hj1(text: String) {
    let words = text.split_whitespace();
    match words.last() {
        Some(last) => println!("{}", last.len()),
        None => println!("0"),
    }
}

#[test]
fn test() {
    use std::io::{self, *};

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.unwrap();
        hj1(text);
    }
}
