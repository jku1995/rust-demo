fn main() {
    let _c = Code::C;
    let _java = Code::Java;
    let _rust = Code::Rust;
    let _v4 = IpKind::V4(172, 2, 0, 10);
    let _v6 = IpKind::V6(String::from("::1"));
    _v4.call();
    //option
    let id_no = Some("1000");
    let name = Some("nil");
    let null:Option<i32> = None;
}
enum Code{
    Java,
    C,
    Rust
}
#[derive(Debug)]
enum IpKind {
    V4(u8,u8,u8,u8),
    V6(String)
}
impl IpKind {
    fn call(& self){
        println!("{:?}",self);
    }
}

