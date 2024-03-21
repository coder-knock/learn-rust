fn main() {
    let s = "asd";
    let _f = || {
        println!("{}", s)
    };
    _f();
}