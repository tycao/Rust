use std::ops::Mul;

trait HasArea<T> {
	fn area(&self) -> T;
}

struct Square<T> {
	x : T,
	y :  T,
	side : T,
}

impl<T> HasArea<T> for Square<T> 
		where T : Mul<Output = T> + Copy {
	fn area(&self) -> T {
		self.side * self.side
	}
}

fn main() {
	let s = Square {
		x : 0.0f32,
		y : 0.0f32,
		side : 12f32,
	};
	println!("Area of s: {}", s.area());
}
