trait ApproxEqual{
	fn approx_equal(&self, othere:&Self) -> bool;
}
impl ApproxEqual for f32{
	fn approx_equal(&self, other:&Self) -> bool{
		// Appropriate for 'self' and 'other' being close to 1.0
		(self-other).abs() <= ::std::f32::EPSILON
	}
}

fn main() {
	println!("{}", 1.0f32.approx_equal(&1.00000001f32));
}
