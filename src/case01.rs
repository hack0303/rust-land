fn main(){
    println!("input something ...");

    let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("hello");

    println!("your guess is {}",guess);
}