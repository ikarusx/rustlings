// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

//// I AM NOT DONE

fn main() {
    //let a = ??? // error
    //let a = [0; 99];
    let a = [0; 100];
    //let a = 0..100;
    //let a: [i32; 100] = [0; 100];
    //let a = ['\0'; 333];
    //let a = "A string with a hundred characters would also work for this exercise, since it is essentially chars.";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
