fn main() {
	let celsius_temp = 23.0;
	let fahrenheit_temp = convert(celsius_temp);

	assert_eq!(fahrenheit_temp, 73.4);
	println!("Test passed!");
}

fn convert(number: f64) -> f64 {
	number * 1.8 + 32.0
}
