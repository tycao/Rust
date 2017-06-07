trait Foo{
	fn is_valid(&self) -> bool;
	
	fn is_invalid(&self) -> bool{ return !self.is_valid() }
}

struct UseDefault;

impl Foo for UseDefault{
	fn is_valid(&self) -> bool {
		println!("called UseDefault.is_valid().........");
		true
	}
}

struct OverrideDefault{}

impl Foo for OverrideDefault{
	fn is_valid(&self) -> bool {
		println!("called OverrideDefault.is_valid().........");
		true
	}
	
	fn is_invalid(&self) -> bool{
		println!("called OverrideDefault.is_invalid()...........");
		false
	}
}

fn main() {
	let a = UseDefault;
	let b = OverrideDefault{};
	
	assert!(!a.is_invalid());
	assert!(!b.is_invalid());
}
