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
#[test]
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
#[test]
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

/// # 识别有效的IP地址和掩码并进行分类统计
/// ## 描述
/// 请解析IP地址和对应的掩码，进行分类识别。要求按照A/B/C/D/E类地址归类，不合法的地址和掩码单独归类。
/// > 所有的IP地址划分为 A,B,C,D,E五类
/// > A类地址1.0.0.0 ~ 126.255.255.255;
/// > B类地址128.0.0.0 ~ 191.255.255.255;
/// > C类地址192.0.0.0 ~ 223.255.255.255;
/// > D类地址224.0.0.0 ~ 239.255.255.255；
/// > E类地址240.0.0.0 ~ 255.255.255.255
///
/// > 私网IP范围是：
/// > 10.0.0.0～10.255.255.255
/// > 172.16.0.0～172.31.255.255
/// > 192.168.0.0～192.168.255.255
/// 子网掩码为二进制下前面是连续的1，然后全是0。（例如：255.255.255.32就是一个非法的掩码）
/// 注意二进制下全是1或者全是0均为非法

/// > 注意：
/// > 1. 类似于【0.*.*.*】和【127.*.*.*】的IP地址不属于上述输入的任意一类，也不属于不合法ip地址，计数时可以忽略
/// > 2. 私有IP地址和A,B,C,D,E类地址是不冲突的
/// ## 输入描述：
/// 多行字符串。每行一个IP地址和掩码，用~隔开。
/// ## 输出描述：
/// 统计A、B、C、D、E、错误IP地址或错误掩码、私有IP的个数，之间以空格隔开。
/// ## 示例
/// ```
/// 输入： 10.70.44.68~255.254.255.0
///       1.0.0.1~255.0.0.0
///       192.168.0.2~255.255.255.0
///       19..0.~255.255.255.0
/// 输出： 1 0 1 0 0 2 1
/// ```
#[test]
fn hj18() {
    use std::io::{self, *};

    let stdin = io::stdin();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    let mut err = 0;
    let mut private = 0;

    'line: for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let text: Vec<&str> = line.split("~").collect();
            // 识别子网掩码
            let sub_mask = text[1];
            let sub_mask_split: Vec<&str> = sub_mask.split(".").collect();
            if sub_mask_split.len() != 4 {
                err += 1;
                break 'line;
            }
            let mut tmp = 255u8;
            'mask: for mask in sub_mask_split {
                if let Ok(mask) = mask.parse::<u8>() {
                    println!("{:?}", mask);
                    match mask {
                        0 => {
                            if tmp == 255 {
                                tmp = 0;
                            }
                            continue 'mask;
                        }
                        255 => {
                            if tmp == 255 {
                                continue 'mask;
                            }
                        }
                        _ => {}
                    }
                }
                err += 1;
                break 'line;
            }
            // 识别IP
            let ip = text[0];
            let ip_split: Vec<&str> = ip.split(".").collect();
            if ip_split.len() != 4 {
                err += 1;
                break 'line;
            }
            // ip第1位
            if let Ok(ip1) = ip_split[0].parse::<u8>() {
                println!("{:?}", ip1);
                match ip1 {
                    0 => {}
                    1..=126 => {
                        a += 1;
                        if ip1 == 10 {
                            private += 1;
                        }
                    }
                    127 => {}
                    128..=191 => {
                        b += 1;
                        if ip1 == 172 {
                            let ip2 = ip_split[0].parse::<u8>();
                        }
                    }
                    192..=223 => {
                        c += 1;
                    }
                    224..=239 => {
                        d += 1;
                    }
                    240..=255 => {
                        e += 1;
                    }
                }
            } else {
                err += 1;
                break 'line;
            }
        }
    }
    println!("{} {} {} {} {} {} {}", a, b, c, d, e, err, private);
}
