use std::io::Write;

fn main() {
    let lager = "33 Export \nDesperados \nGoldberg \nGulder \nHeineken \nStar";
    let stout = "Legend \nTurbo King \nWilliam";
    let nonalcoholic = "Maltina \nAmstel Malta \nMalta Gold \nFayrouz";

    let mut file = std::fs::File::create("leo.txt").expect("creation terminated");
    file.write_all("NIGERIAN BREWERIES Plc\n".as_bytes()).expect("write failed");
    file.write_all("\nThe high Quality Categories of our Drinks;".as_bytes()).expect("write failed");
    file.write_all("\n\n(1)lager;\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all("\n\n(2) stout;\n".as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all("\n\n(3) Non-Alcoholic;\n".as_bytes()).expect("write failed");
    file.write_all(nonalcoholic.as_bytes()).expect("write failed");

}