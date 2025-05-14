use std::io;

fn main(){

    print!("Enter the lenght of fibonacchi: ");
    use std::io::Write;
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    
    fibo(n);   
}

fn fibo(n:i32){
    let mut first = 0;
    let mut second = 1;
    
    if n==0{
        print!("Pleas inter greater than zero lenght")
    }
    else if n == 1{
        print!("{} ",first);
    }
    else if n>=2 { 
        print!("{} {} ",first,second);
        for _ in 0..(n-2){
            let fibo = first+second;

            print!("{} ",fibo);

            first = second;
            second = fibo;
        }
        print!("\n");
    }
    // println!("{}\n");
}