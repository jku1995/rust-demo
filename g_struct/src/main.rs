fn main() {
    let person1 =Person {
        name : String::from("liming"),
        addr:"US".to_string(),
        age:10,
        id:5566
    };
    let person2 = Person{
        addr:String::from("CN"),
        ..person1
    };
    println!("{}",person2.name);
    println!("{}{}",person1.age,person1.id);
    let color = Color(3,3,3);
    println!("color:{}",color.0);
    let none = None;
    println!("{}",area(10, 20));
    let rec = Rectangle{
        width:30,
        height:50
    };
    println!("{:#?}",rec);
}
struct Person {
    name:String,
    addr:String,
    age: u8,
    id: u16,
}
struct Color(u8,u8,u8);
struct None;
fn area(length:u8,width:u8)->u8{
    return length*width;
}
#[derive(Debug)]
struct Rectangle{
    width:u16,
    height:u16
}
