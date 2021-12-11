fn main() {
	//type synonym with type alias
	type Kilometers = i32;

	let x: i32 = 5;
	let y: Kilometers = 5;

	println!("x + y = {}", x+y);

	//type alias to make code shorter
	type Thunk = Box<dyn Fn() + Send + 'static>;

	let f: Thunk = Box::new(|| println!("hi"));
}