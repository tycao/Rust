trait Foo{
	fn foo(&self);
}

trait FooBar : Foo {
	fn foobar(&self);
}

struct Baz{}

impl Foo for Baz{
	fn foo(&self){
		println!("foo calling......");
	}
}

impl FooBar for Baz{
	fn foobar(&self){
		println!("foobar calling.....");
	}
}
fn main() {
	let mut a = Baz{};
	a.foo();
	a.foobar();
}
