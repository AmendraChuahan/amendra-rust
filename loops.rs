fn main() {
    println!("loops!");
    //first();
   // second();
   //third();
    fourth();
}
// types of loops
//for, while, loop
// fn first(){
//     let mut x = 0;

//     loop {
//      x += 1;
//      println!("x = {}",x);
//      if x == 5{
//      println!("we did it");
//      break;
//     }

// }
// }
// fn second(){
//     let mut number= 0;
    
//     while number!=0{
//         println!("{}",number);
//           number ==  1;
//     }
//     println!(" hello");
// }
// fn third(){
//     let a = [10,20,30,40,50,55];
//     let mut index=0;
//     while index<6{// while loop
//         println!(" the value is {}",a[index]);
//         index +=1;

//     }
//     println!(" hello");
// }
// for loops
fn fourth(){
    for x in 1..11{
        if x==5 {
            continue;

        }
        println!(" the value  {}",  x);
    }
   
}