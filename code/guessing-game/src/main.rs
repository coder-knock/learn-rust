use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");
    // 生成 1 到 100 直接随机数
    let secret_number = thread_rng().gen_range(1..=100);
    // 循环
    loop {
        println!("Please input your guess.");
        // rust 中变量默认不可变，加上 mut 使其可变
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed:{guess}");
        // rust 允许声明同一名称的变量来覆盖就变量
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 进行比较，并根据不同结果输出不同信息
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
