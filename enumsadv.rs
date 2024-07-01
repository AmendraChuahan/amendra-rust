fn main() {
    println!(" Enum advance!");
    let person1 = Person{
    timeframe:15,
    capital:4340,
    tradesite:String::from("tradingviwe"),
    broker:String::from("kotakneu"),
    trader:Tradercat::Intraday
};

println!("{:?}",person1);
let result = cal(45);
println!("{:?}",result);
}
#[derive(Debug)]// solve debug error
enum Tradercat{
    Intraday,Swing,Scelping
}
#[derive(Debug)]
struct Person {
    timeframe:i32,
    capital:i32,
    broker: String,
    tradesite:String,
    trader:Tradercat


}
//option enums
fn cal(no:i32)-> Option<bool>
{
    if no%2  ==0{
        Some(true)
    }
    else{
        None
    }
}
