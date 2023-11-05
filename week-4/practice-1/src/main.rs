fn main() {
    let name = "aisha lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str="Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name );
    println!("University:{},\nAdress: {}",uni,addr );


    let department:&'static str = "computer science";
    let school:&'static str ="School of Scince and Technology";
     println!("Department:{},\nSchool: {}",department,school);
}
