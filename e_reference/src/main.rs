
fn main() {
    let a = 3;
    let b = copyvalue_on_stack(a);
    println!("{b}{a}\n----\n");
    let c = String::from("hello");
    let mut d = move_ownship(c);
        /*
     * error,because c's ownship of "hello" was move to d
     */
    //println!("{c}{d}");
    let ref_d= copy_ref(&d);
    println!("{d}{ref_d}");
    let mut f = &mut d;
    String::push_str(&mut f,"world");
    println!("{f}");
    //copy trait
    
}
fn copyvalue_on_stack(arg:i8)->i8{
    arg
}
fn move_ownship(arg:String)->String{
       return arg ;
}
fn copy_ref(arg:& String)->& String {
    &arg
}
