use std::io;
use std::io::Write;


struct Login{
    username: String, password: String
}

impl Login{
    fn data_check(&mut self)->bool{
        let mut sustainable_password = false; 

        const MAXIMUM_PASSWORD_LENGTH:usize = 8;
        const MINIMUM_PASSWORD_LENGTH:usize = 3;

        let upper_case_present = self.password != self.password.to_lowercase();

        let disallowed_charachter_present = self.disallowed_charachter_check();

        let accepted_length = self.password.len() >= MINIMUM_PASSWORD_LENGTH && self.password.len() <= MAXIMUM_PASSWORD_LENGTH;

        let number_present = self.number_check();
        let letter_present = self.password.to_lowercase() != self.password.to_uppercase(); 

        if !upper_case_present && !disallowed_charachter_present && accepted_length && number_present && letter_present{
            sustainable_password = true;
        }

        let errors = (upper_case_present, disallowed_charachter_present,self.password.len() >= MINIMUM_PASSWORD_LENGTH,self.password.len() <= MAXIMUM_PASSWORD_LENGTH, number_present, letter_present);

        match errors { 
            (true,_,_,_,_,_) => println!("Password cannot have upper case letters"),
            (_,true,_,_,_,_) => println!("Password cannot contain charachters '$','#' or '@'"),
            (_,_,false,_,_,_) =>println!("password is too short"),
            (_,_,true,false,_,_) => println!("Password is too long"),
            (_,_,_,_,false,_) =>println!("Password must contain a number"),
            (_,_,_,_,_,false) =>println!("Password must contain a letter"),
            (false, false, true, true, true,true) => println!("Login Succesful"),
        }

        return sustainable_password;
    }

    fn disallowed_charachter_check(&mut self)->bool{ 
        let diallowed_charachters = ["$","#","@"];
        let mut status = false;

        for charachters in diallowed_charachters{
            if self.password.contains(charachters){
                status = true;
                break;
            }
        }

        return status;
    }

    fn number_check(&mut self)->bool{
        let numbers = ["0","1","2","3","4","5","6","7","8","9"];
        let mut status = false;
        for number in numbers{
            if self.password.contains(number){
                status = true;
                break;
            }
        }
        return status
    }
}

fn login(){
    let mut username_path = String::new();
    let mut password_path = String::new();
    let mut valid = false;
    const USERNAME_LENGTH:usize = 4; 

    let mut login = Login{
        username:String::new(), password: String::new(),
    };

    loop{
        println!("Enter Company Name");
        io::stdin().read_line(&mut username_path).expect("Failed to read line");
        if username_path.trim().len() < 4{
            println!("Company name is short. It must contain at least 4 Charachters");
            username_path = String::new();
    }   else{
            break;
    }

    };

    while !valid{
        println!("Enter Password");
        io::stdin().read_line(&mut password_path).expect("Failed to read line");

        login = Login {
            username: username_path[..USERNAME_LENGTH].trim().to_string(), password: password_path.trim().to_string()
        };

        valid = login.data_check();
        if !valid{
            password_path = String::new();
             }
            
    }
    println!("Username: {} \nPassword: {}", login.username, login.password);

}


fn file_creator(){//code to create files.
    let companys = ["Cadbury Nigeria Plc.", "Champion Breweries Plc." , "Dangote Sugar Refinery Plc.", "Flour Mills Nigeria Plc", "Nestle Nigeria Plc", "Unilever Nigeria Plc", "Honeywell Nigeria Plc", "Nigeria Breweries Plc"];//intialize vectors.
    let companys_shares = [15_000_000,25_000_000,18_000_000,32_000_000,8_000_000,37_000_000,34_000_000,30_000_000];
    let companys_liabilities = [5_500_000,8_000_000,10_000_000,4_000_000,1_500_000,11_000_000,9_000_000,12_000_000];
    let year_of_establishment = [1965,1974,1970,1960,1961,1923,1906,1946];
    let mut companys_leverages = vec![];

    for index in 0..companys.len(){//code to calculate each company's leverage and push it to the leverage vector.
        let leverage = (  companys_shares[index] - companys_liabilities[index])as f32 / companys_shares[index] as f32;
        companys_leverages.push(leverage*100.)//multiples the float by a hundred so it's saved a percentage.;

    };

    let mut file = std::fs::File::create("Company_Information.txt").expect("Failed to create file");//creates file.

    let column_names = ["Company","Company's Year of Creation", "Company's Shares", "Company's Liabilities","Company's Leverage"]; //defining column names, these are the names of the vectors.

    for column in column_names{
        file.write_all(format!("{}\t", column).as_bytes()).expect("Failed to write to file");//writes the headers of the columns into the file on each column.
    }
    file.write_all(format!("\n").as_bytes()).expect("Failed to create new line");//goes to a new line to begin recording actual data.

    for index in 0..column_names.len(){
        file.write_all(format!("{}\n {}\n {}\n {}\n {}%\n \n",companys[index],year_of_establishment[index],companys_shares[index],companys_liabilities[index],companys_leverages[index]).as_bytes()).expect("Failed to write to file");//places each company's data on a new row.
    }


    const TARGET_LIABILITIES:i32 = 10_000_000; //Constants to avoid Magic numbers in future operations.
    const TARGET_SHARES:i32 = 20_000_000;
    const PERCENTAGE_LEVERAGE_MULTIPLIER:f32 = 0.05;

    let mut file = std::fs::File::create("Shares-Leverage.txt").expect("Failed to create file");//creates file.
    file.write_all(format!("Company Name\t Shares-Leverage_Multiple\n").as_bytes()).expect("Failed to write to file");

   

    for pointer in 0..companys.len(){

        if companys_shares[pointer] > TARGET_SHARES{//If a company has shares greater thatn the Target share value, The Shares are saved as a multiple of the percentage leverage.
            file.write_all(format!("{}\t {}\n", companys[pointer], (companys_shares[pointer]as f32* companys_leverages[pointer]as f32)/100.).as_bytes()).expect("Failed to write to file"); //recall Leverage was multipled by 100 initially for display purposes. Here, we divide by 100 for calculation purposes.
        }

    }

    let mut file = std::fs::File::create("Shares-Leverage_Multiples.txt").expect("Failed to create file");//creates file.
    file.write_all(format!("Company Name\t {}% of Leverage\n", PERCENTAGE_LEVERAGE_MULTIPLIER*100.).as_bytes()).expect("Failed to write to file");//Creates columns

    for pointer in 0..companys.len(){//If a company's liability is less than the target, It's leverage gets multipled by the targer percentage leverage multiplier. As at the time of writing this, it's 5%.

        if companys_liabilities[pointer] < TARGET_LIABILITIES{
            file.write_all(format!("{}\t {}\n", companys[pointer], (companys_leverages[pointer] as f32 * PERCENTAGE_LEVERAGE_MULTIPLIER)).as_bytes()).expect("Failed to write to file");
        }

    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {//main code. Only purpose is to call functions.
    login();
    file_creator();
}