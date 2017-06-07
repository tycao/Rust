struct Rectangle<T>{
	x : T,
	y : T,
	width : T,
	height : T,
}

impl <T : PartialEq> Rectangle<T>{
	fn is_equal(&self) -> bool {
		self.height == self.width
	}
}
fn main() {
	let mut r = Rectangle{
		x : 0,
		y : 0,
		width : 47,
		height : 47,
	};
	assert!(r.is_equal());
	r.height = 42;
	assert!(!r.is_equal());
}
