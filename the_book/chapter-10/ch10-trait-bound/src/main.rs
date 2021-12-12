use ch10_trait_bound::Pair;

fn main() {
	let p1 = Pair::new(23, 2);

	p1.cmp_display();

	//println!("The pair: {}", p1);
}