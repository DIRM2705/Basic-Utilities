use std::io::{stdin, Write, stdout};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn read_int(instruction : *const u8) -> i32 {
    loop {
        let input = read_input(instruction);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Ingrese un número válido");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn read_float(instruction : *const u8) -> f32 {
    loop {
        let input = read_input(instruction);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Ingrese un número válido");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn read_bool(instruction : *const u8) -> bool {
    loop {
        let input = read_input(instruction);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Ingrese true o false");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn read_string(instruction : *const u8) -> *const u8 {
    return read_input(instruction).as_ptr() as *const u8;
}

fn read_input(instruction : *const u8) -> String {
    //Flush stdout
    stdout().flush().unwrap();

    // Print the instruction
    let c_str: &CStr = unsafe { CStr::from_ptr(instruction as *const i8) };
    let str_slice: &str = c_str.to_str().unwrap();
    println!("{}", str_slice);

    // Read the input
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => input = input.trim().to_string(),
        Err(err) => println!("{err}")
        
    }

    return input
}
