// 建议的风格，函数名()一个空格{
fn main() {
    // 缩进使用四个空格
    println!("Hello, world!");
    let array = [1, 2, 3];
    let len = array.iter().clone().collect::<Vec<_>>().len();
    println!("len: {}", len);
}
// mac linux rustc 编译后生成 main
// Windows 编译后生成 main.exe main.pdb，main.pdb 为调试信息
