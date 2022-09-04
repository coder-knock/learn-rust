use std::intrinsics::try;
use std::io;

fn main() {
    // const 必须添加类型，且值仅可设置为常量表达式
    const CONST_VARIABLES: u32 = 1000;
    let data = 5;
    println!("5 {data}");
    let data = data + 1;
    println!("6 {data}");
    {
        let data = 1;
        println!("10 {data}");
    }
    println!("12 {data}");
    println!("{CONST_VARIABLES}");
    //Scalar Type https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
    // the literal `1234` does not fit into the type `u8` whose range is `0..=255`
    // https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow 实际测试会提示异常
    // 整数除法会向下取整
    // let u8data: u8 = 10 * 80;
    // Rust char 类型四个字节，比其他语言能承接的字符更多
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    // 变量名 _ 开头会忽略其是否使用
    println!("{heart_eyed_cat}");
    let tup = (600, 6.4, 1, "sg");
    let (.., a) = tup;
    println!("{a}");
    // 覆盖变量可以更改类型 mut 重新赋值变量不可
    let data = tup.2;
    println!("{data}");
    let data = tup.1;
    println!("{data}");
    let data = tup.3;
    println!("{data}");
    // 数组必须类型一致
    // let array = [600, 6.4, 1, "sg"]; //expected integer, found floating-point number
    let mut array: [i32; 4] = [600, 100, 200, 900];
    let data = array[2];
    let size = array.len();
    println!("data:{data}  size:{size}  array:[i32l;4] 数据类型、数组长度");
    array[2] = 777;
    let data = array[2];
    println!("data:{data}  size:{size}  array:[i32l;4] 数据类型、数组长度 array 需要设置成 mut 才能修改其中的元素");
    // 快速创建 100 个 8 的数组
    let array_same_element = [8; 100];
    let data = array_same_element[2];
    let size = array_same_element.len();
    println!("array_same_element data:{data}  size:{size}  array:[i32l;4] 数据类型、数组长度");
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    // 测试数组越界
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    // 数组越界会直接中断程序，所以需要做好检查
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
