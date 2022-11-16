fn main() {
   let condition = RMB::Ten;

   let a =  match condition {
            RMB::Ten=>10,
    };
    println!("{a}");
    let quart = Coin::Quarter(UsState::Alaska);
    let p = Some(5);
    let p1 = plus_one(p);
    println!("{:?},{:?}",p1,p);
    println!("---");
    let dist_number = 9;
    match dist_number {
        3=>add_fancy_hat(),
        7=>remove_fancy_hat(),
        //other=>println!("{other}"),
        _=>(),
    }
}
enum RMB {
    Ten,
}
enum UsState {
    Alabama,
    Alaska
}
enum Coin {
    Penny,
    Nickey,
    Dime,
    Quarter(UsState)
}
/*get a u32 number in option
 * if not exist return
 * else take that value to plus one
 */
fn plus_one(op:Option<u8>)->Option<u8>{
    match op {
        Some(i) => Some(i+1) ,
        None => None
    }
}
fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn reroll(){}
