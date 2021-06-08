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
fn hj1() {
    use std::io::{self, *};

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.unwrap();

        let words = text.split_whitespace();
        match words.last() {
            Some(last) => println!("{}", last.len()),
            None => println!("0"),
        }
    }
}

/// # 计算某字母出现次数
/// ## 描述
/// 写出一个程序，接受一个由字母、数字和空格组成的字符串，和一个字母，然后输出输入字符串中该字母的出现次数。不区分大小写，字符串长度小于500。
/// ## 输入描述：
/// 第一行输入一个由字母和数字以及空格组成的字符串，第二行输入一个字母。
/// ## 输出描述：
/// 输出输入字符串中含有该字符的个数。
/// ## 示例
/// ```
/// 输入： ABCabc
///       A
/// 输出： 2
/// ```
fn hj2() {
    use std::io::{self, *};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    if let Some(Ok(text)) = lines.next() {
        let text = text.to_lowercase();
        if let Some(Ok(word)) = lines.next() {
            let word = word.to_lowercase();
            let new_text = text.replace(&word, "");
            println!("{}", text.len() - new_text.len());
        }
    }
}

#[test]
fn test() {
    hj2();
}
