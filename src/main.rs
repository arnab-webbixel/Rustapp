// fn main (){
//      let url = "https://jsonplaceholder.typicode.com/todos/1";
//      match reqwest::blocking::get(url) {
//          Ok(response) => {
//              match response.text() {
//                  Ok(text) => println!("Response Text: {}", text),
//                  Err(e) => eprintln!("Failed to read response text: {}", e),
//              }
//          },
//          Err(e) => eprintln!("Request failed: {}", e),
//      }
// }



// fn main (){
//     println!("Hello, world!");
//     let x = 5;
//     let y = 10;
//     if (x == 5){
//         panic!("x should not be 5");
//     }
//     println!("x + y = {}", x + y);
// }




// fn main(){
//     let maybe_number: Option<i32> = Some(10);
//     let value = maybe_number.unwrap();
//     println!("The number is {}", value);

//     let none_value: Option<i32> = None;
//     let crash = none_value.unwrap(); // This will panic!
//     println!("This will never run: {}", crash);
// }


// fn find_char(s: &str, target:char) -> Option<usize> {
//     for (i, c) in s.chars().enumerate() {
//         if c == target {
//             return Some(i);
//         }
//     }
//     None
// }


// fn main(){
// let s = "Hello, world!";
// match find_char(s, 'w') {
//     Some(index) => println!("Found 'w' at index: {}", index),
//     None => println!("'w' not found in the string"),
// }






fn some_change(s: &mut String){
    s.push_str(", world");
}

fn print_length(s: &String) {
    println!("The length of the string is: {}", s.len());
}

fn main (){
    let s1 = String::from("hello");
    let mut  s2 = s1;
    some_change(&mut s2);
    print_length(&s2);
}




