use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();
    println!("Enter the value of x: ");
    io::stdin().read_line(&mut x).expect("not a valid input");
    let x:f32 = x.trim().parse().expect("not a valid number");

    println!("Enter the value of y: ");
    io::stdin().read_line(&mut y).expect("not a valid input");
    let y:f32 = y.trim().parse().expect("not a valid number");

    println!("Enter the value of z: ");
    io::stdin().read_line(&mut z).expect("not a valid input");
    let z:f32 = z.trim().parse().expect("not a valid number");

    let d = y.powf(2.0)- 4. *  x * z;

    if d > 0. { 
     let answer_1 = (-1.*y + d.powf(0.5))/(2.*x);
     let answer_2 = (-1.*y - d.powf(0.5))/(2.*x); 
     println!("The first root is , {} The second root is,{}",answer_1,answer_2);
 
    }else if d<0. {
        println!("the roots are imaginary" );
    }else {
        println!("The answer is {}", (-1.*y + d.powf(0.5))/(2.*x));
    }
    

}
