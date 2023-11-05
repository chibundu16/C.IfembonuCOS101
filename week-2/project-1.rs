<<<<<<< HEAD
fn main() {
	
	let p = 520_000_000.0;
	let r = 10.0;
	let n = 5.0;

	//Compound Interest

	let x = 1.0 + (r/100.0);

	let y = f32::powf(x, n);

	let a = p * y;

	println!("Compounded Amount is {}", a);

	let ci = a - p;

	println!("Interest is {}", ci);
=======
fn main() {
	
	let p = 520_000_000.0;
	let r = 10.0;
	let n = 5.0;

	//Compound Interest

	let x = 1.0 + (r/100.0);

	let y = f32::powf(x, n);

	let a = p * y;

	println!("Compounded Amount is {}", a);

	let ci = a - p;

	println!("Interest is {}", ci);
>>>>>>> de0adf3d8c0401736b40bad03664981b3b51efb0
}