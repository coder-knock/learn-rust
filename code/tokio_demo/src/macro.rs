macro_rules! add { // 宏定义
    ($($x:expr),*) => { // 匹配模式
        { // 匹配到后执行的代码
            let mut temp = 0;
            $(
                temp += $x;
            )*
            temp
        }
    };
}
fn main() {
    // cargo install cargo-expand 该工具可以展开宏生成的代码 cargo expand --bin macro
    println!("{}", add!(1));
    println!("{}", add!(1,2));
    println!("{}", add!(1,2,3));
}


// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2021::*;
// #[macro_use]
// extern crate std;
// fn main() {
//     {
//         ::std::io::_print(
//             format_args!(
//                 "{0}\n",
//                 {
//                     let mut temp = 0;
//                     temp += 1;
//                     temp
//                 },
//             ),
//         );
//     };
//     {
//         ::std::io::_print(
//             format_args!(
//                 "{0}\n",
//                 {
//                     let mut temp = 0;
//                     temp += 1;
//                     temp += 2;
//                     temp
//                 },
//             ),
//         );
//     };
//     {
//         ::std::io::_print(
//             format_args!(
//                 "{0}\n",
//                 {
//                     let mut temp = 0;
//                     temp += 1;
//                     temp += 2;
//                     temp += 3;
//                     temp
//                 },
//             ),
//         );
//     };
// }
