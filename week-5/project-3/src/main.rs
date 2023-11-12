use std::io;
fn main() {
    println!("Menu                            Price
P = Poundo Yam/Edinkaiko Soup       - N3,200
F = Fried Rice & Chicken            - N3,000
A = Amala & Ewedu Soup          - N2,500
E = Eba & Egusi Soup            - N2,000
W = White Rice & Stew           - N2,500
");
    println!("Enter (p)for poundo, (f) for fried rice, (a) for amala, (e) for eba, (w)for white rice, press any other key to view cost");
    let mut cost = 0;


    loop{
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input");

        if choice.trim() == "p"{
            cost += 3200;
        }else if choice.trim() == "f"{
            cost += 3000;
        }else if choice.trim() == "a"{
            cost += 2500;
        }else if choice.trim() == "e"{
            cost += 2000;
        }else if choice.trim() == "w"{
            cost += 2500;
        }else{
            break;
        }




    }
    if cost > 10_000{
        let price:f32 = (cost as f32) * 0.95;
        println!("You qualify for a discount of 5%");
        println!("Your total cost is {}", price);
    }else {
        println!("Your total cost is {}", cost);
    }
}
