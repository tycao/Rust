macro_rules! foo {
	(x => $e : expr) => { println!("mode X : {}", $e)};
	(y => $e : expr) => { println!("mode Y : {}", $e)}
}

//这个巴洛克宏展示了外层重复中多余的变量。
macro_rules! o_O {
	(
		$($x : expr; [$($y : expr), *]); *
	) => {&[$($($x + $y), *), *]};
}

macro_rules! v {
	( $($x : expr), *) =>  {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x);
			)*
			temp_vec
		}
	}
}

macro_rules! five_times {
	($x : expr) => {5 * $x}
}
fn main() {
	
	assert_eq!(25, five_times!(2 + 3));

	let a : &[i32]
		=  o_O!(10; [1, 2, 3]; 20; [1, 2, 3]);
	assert_eq!(a, [11, 12, 13, 21, 22, 23]);
	println!("=============================");
	
	foo!(x => 1);
	println!("=============================");
	
	assert_eq!(v![1, 2, 3], [1, 2,  3]);
	println!("=============================");
	
	let x : Vec<u32>  = {
		let  mut temp_vec = Vec::new();
		temp_vec.push(1);
		temp_vec.push(2);
		temp_vec.push(3);
		temp_vec
	};
	let y : Vec<i32> = vec![1, 2, 3];
	assert_eq!(x, [1, 2, 3]);
	assert_eq!(y, [1, 2, 3]);
}
