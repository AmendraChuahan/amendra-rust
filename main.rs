//  expressions
fn main() {
    println!("Hello, world!");
    ex();
    //calling
    let xy = return_value();
    println!(" the x ,y - {}", xy);
}
fn ex() {
    let y = {
        let x = 9;
        x + 1
    };
    println!(" value of y is {}",y);
}
// return value from function
 fn return_value () ->i32 {
    75 + 32
 }
