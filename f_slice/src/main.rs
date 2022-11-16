use std::io;

fn main() {
    println!("pls input a string which split with space:");
    let mut input  = String::new();
    match io::stdin().read_line(&mut input){
        Ok(n)=>{
          println!("{n} bytes");
        },
        Err(err)=>{
            println!("{err}");
        }
    }
    let c = first_word(&input);
    println!("{c}");
    let s = slice_word(&input[..5]);
    println!("{s}");
    let a = [1,2,3,4,5];
    let aslice = &a[0..3];
    assert_eq!(aslice,&[1,2,3]);
    let hello = String::from("hello");
    let he = &hello[..2];
    println!("{he}");
}
fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    s.len()
}
fn slice_word(s:& str)->&str{
   &s[..]
}
