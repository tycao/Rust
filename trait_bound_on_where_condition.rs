use std::fmt::Debug;
fn main() {
	foo("hello", "world!");
	println!("+++++++++++++++++++");
	bar("hello!!!", "world!!!!!!");
}

fn foo<T : Clone + Debug, K : Clone + Debug>(x : T, y : K){
	x.clone();
	y.clone();
	println!("y : {:?}", y);
	println!("x : {:?}", x);
}

fn bar<T, K>(x : T, y : K) where T : Clone, K : Clone + Debug{
	x.clone();
	y.clone();
	println!("y : {:?}", y);
}
