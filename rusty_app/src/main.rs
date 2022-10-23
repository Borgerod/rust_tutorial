// fn main() {
// 	println!("Hello, world!");
// 	println!("your mother!");
// }


/* 
	*	 ____  OOP WITH RUST  ____
	*	instead of classes, rust uses structs along with traits and impl 
	*	https://medium.com/geekculture/how-to-use-object-oriented-programming-in-rust-6257eac3ef03#:~:text=Rust%20supports%20oops%20through%20structs,like%20Java%20and%20C%2B%2B%20.
*/

mod http_req;

//* 1. Struct (class)
struct Cylinder {
	radius: f32,
	height: f32,
	unit: String,
}



//* */ We can access the attributes by instantiating structs
// fn main() {
// 	let cylinder:Cylinder = Cylinder { 
// 		radius: 2.0, 
// 		height: 2.0, 
// 		unit: "cm".to_string() 
// 	};
// 	println!("radius of cylinder: {} {}", cylinder.radius, cylinder.unit);
// 	println!("radius of cylinder: {} {}", cylinder.height, cylinder.unit);	
// }

// * 2. Trait (methods / functions / attributes )
// * Traits only holds the behavior / methods of an object

trait Formula {
	fn get_area(&self) -> f32;
	fn get_perimeter(&self) -> f32;
	//* */ Here, we defined a Formula traits with get_area and get_perimeter function.
}

//* */ We can implement the method too struct using impl <trait> for <struct>.
const PI: f32 = 3.1415;

impl Formula for Cylinder{
	fn get_area(&self) -> f32 {
		2.0 * PI * self.radius * (self.height + self.radius)
	}
	fn get_perimeter(&self) -> f32 {
		2.0 * PI *self.radius
	}
}


// 3. Impl (implementation)
//  as seen in previous example "impl" is used to add method(traits) to struct
impl Cylinder {
	fn new(radius: f32, height: f32) -> Cylinder{
		Cylinder { radius: radius, 
			height: height, 
			unit: "cm".to_string(),
		}	
	}
	fn get_summary(&self){
		println!("Summary:");
		println!("r: {}{} * h: {}{}", self.radius, self.unit, self.height, self.unit);
		println!("area: {}\nperimeter:{}", self.get_area(), self.get_perimeter());		
	}
} 

fn main(){
	let cylinder = Cylinder::new(1.0, 2.0);
	cylinder.get_summary();
}

