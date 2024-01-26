fn main() {
    let arr1:[i32;4] = [10,20,30,40];
    println!("an array with data type");
    println!("array is {:?}",arr1 );
    println!("array size is {}",arr1.len());

       let arr3:[i32;8] = [-1;8];
    println!("an array with default values");
    println!("array is {:?}",arr3 );
    println!("array size is {}",arr3.len());

       let arr2 = [10,20,30,40,54,22,45,34,11,2,2];
    println!("an array without data type");
    println!("array is {:?}",arr2 );
    println!("array size is {}",arr2.len());

    let city_arr:[&str;5] = ["bubu","clap","stab","wack","lab"];
     println!("array is {:?}",city_arr );
      println!("array size is {}",city_arr.len());

      for index in 0..5 {
        print!(" City index {} is located in : {}"
            ,index, city_arr [index]);
      }
}
