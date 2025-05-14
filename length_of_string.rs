fn main(){
    let str = String::from("Robiul islam");

    let len = get_string_lenght(&str);
    print!("{}\n",len);
}

fn get_string_lenght(str: &str) -> usize{
    return  str.chars().count();
}
