mod compute{
	fn is_zero(number:i32) ->bool{
		if number == 0 {return true};
		false
	}
	
	pub fn add(a: i32, b:i32) -> i32{
		a + b
	}
	
	pub fn subtract(a: i32, b:i32) -> i32{
		a-b
	}
	
	pub fn divide(a: i32, b: i32) -> i32{
		a/b
	}
	
	pub fn multiply(a: i32, b: i32) -> i32{
		a*b
	}
	
}

use compute::add as my_add;
fn main() {
    let a: i32 = 10;
	let b: i32 = 4;
	println!("{} + {} = {}",a, b, my_add(a, b));
	println!("{} - {} = {}",a, b, compute::subtract(a, b));
	println!("{} / {} = {}",a, b, compute::divide(a, b));
	println!("{} * {} = {}",a, b, compute::multiply(a, b));
}
