trait HasArea{
	fn area(&self) -> f64;
}

struct Circle{
	x : f64,
	y : f64,
	radius : f64,
}

struct Square{
	x : f64,
	y : f64,
	side : f64,
}

impl HasArea for Circle{
	fn area(&self) -> f64{
		self.radius * self.radius
	}
}

impl HasArea for Square{
	fn area(&self) -> f64{
		self.side * self.side
	}
}

fn print<T : HasArea>(shape : T){
	println!("{}", shape.area());
}
fn main() {
	let c = Circle{
		x : 0.1,
		y : 0.2,
		radius : 2.0,
	};
	let d = Square{
		x : 1.0,
		y : 2.0,
		side : 3.0,
	};
	print(c);
	print(d);
}
