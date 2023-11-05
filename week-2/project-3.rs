<<<<<<< HEAD
fn main() {
	
	let p = 210_000.0;
	let r = 5.0;
	let n = 3.0;

	//Compound Interest

	let x = 1.0 - (r/100.0);

	let y = f32::powf(x, n);

	let a = p * y;

	println!("The depreciated amount is {}", a);

=======
fn main() {
	
	let p = 210_000.0;
	let r = 5.0;
	let n = 3.0;

	//Compound Interest

	let x = 1.0 - (r/100.0);

	let y = f32::powf(x, n);

	let a = p * y;

	println!("The depreciated amount is {}", a);

>>>>>>> de0adf3d8c0401736b40bad03664981b3b51efb0
}