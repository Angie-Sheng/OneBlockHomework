trait Shape {
    fn calculate_area(&self) -> f64;            
}
 
struct Circle {
    radius: f64,
}
 
impl Shape for Circle {               
    fn calculate_area(&self) -> f64 {           
    3.14 * self.radius * self.radius   
    } 
}

struct Rectangle {   
    width: f64,         
    height: f64,      
} 

impl Shape for Rectangle {             
    fn calculate_area(&self) -> f64 {
        self.width * self.height     
    }     
}      

fn print_result<T: Shape>(shape: T) {
        println!("Calculating area: {}",shape.calculate_area());     
    }      
        
fn main() {    
    let circle = Circle { radius: 327.7 };   
    print_result(circle);
    let rectangle = Rectangle{ width: 2.9, height: 8.0 };         
    print_result(rectangle);   
}