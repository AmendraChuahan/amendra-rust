
struct Rectangle{
    width:u32,height:u32
}
impl  Rectangle{
fn printdes (&self){
    println!("rectengle width {} height {}", self.height,self.width)

}
fn is_square(&self)-> bool{
    self.width== self.height

}
fn cal_square(&self) -> u32{
    self.width*self.height
}

}


fn main() {
    println!("Hello, world!");
    let my_rectangle= Rectangle{width:43,height:45};
    my_rectangle.printdes();
    println!("Rectnagle is square:{}",my_rectangle.is_square());
    println!(" sequre area of rectangle is {} ", my_rectangle.cal_square())
}
