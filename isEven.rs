use std::io;


fn is_even(num: i32) -> bool{
    return num%2==0;
}
fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let x:i32 = input.trim().parse().unwrap();
   
   if is_even(x){
        println!("{} is a even number",x);
   }
   else{
        println!("{} is not a even number",x);
   }
    
}
