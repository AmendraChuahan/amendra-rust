fn main() {
    println!("stacks");
    stack_men();
}
fn stack_men(){
    let x=9;
    let y=6;
    let sum = add(x,y);
    println!(" the sum of x {} and y {} is -{}",x,y,sum);
    fn add(a:i32,b:i32)-> i32 {
        a+b
    }
}
