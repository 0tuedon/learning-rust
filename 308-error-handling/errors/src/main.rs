use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Panic in  rust
    // panic!("Error out of bounds");

    //let vec = vec![1];
    // let's cause an error by accessing out of bounds array;

    //vec[10];

    // Create an error opening file and match it when there is error;
    //   let open_file = File::open("error.txt");

    //   let open_file = match open_file {
    //       Ok(file) => file,
    //       Err(err) => match err.kind() {
    //           ErrorKind::NotFound => match File::create("error.txt") {
    //               Ok(file_created) => file_created,
    //               Err(error) => panic!("Can't create file"),
    //           },
    //           _ => panic!("Error creating file"),
    //       },
    //   };

    // use unwrap and expect
    let open_file = File::open("error.txt").unwrap(); // handles errors for us
    // let another_open_file = File::open("error.txt").expect("Error opening file")
}
