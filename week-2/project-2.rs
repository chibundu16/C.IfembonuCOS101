fn main() {
	
	println!();

	// declaring laptops

	let t = 2.0 * 450_000.00;
	let m = 1.0 * 1_500_000.00;
	let h = 3.0 * 750_000.00;
	let d = 3.0 * 2_850_000.00;
	let a = 1.0 * 250_000.00;
	let n = 10.0;

	// Sum

	let sum = t + m + h + d + a;
	println!("The sum is {}",sum);

	// Average

	let avg = sum / n;
	println!("The average is {}",avg);
}