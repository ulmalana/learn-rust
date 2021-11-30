use ch11_adder_3;

mod common;

#[test]
fn this_adds_two() {
	common::setup();
	assert_eq!(4, ch11_adder_3::add_two(2));
}