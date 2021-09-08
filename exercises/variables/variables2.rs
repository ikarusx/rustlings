// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

//// I AM NOT DONE

fn main() {
    //let x; // error
    let x = 10; // V
    //let x : i32; // uninitialized
    //let x : i32 = 10; // V
    //let x = ""; // no &str == integer impl
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
