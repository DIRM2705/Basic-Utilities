use std::ffi::{CStr, CString};
use std::io::stdin;
use std::os::raw::c_char;

#[repr(C)]
pub struct Menu {
    name: *const c_char,
    options_size: u32,
    options: *const *const c_char,
}

#[no_mangle]
pub extern "C" fn new_menu(name: *const c_char) -> *mut Menu {
    /*
        Creates a new menu struct with the given name
        PARAMS:
            name : *const c_char : The name of the menu
        RETURNS:
            *mut Menu : The created menu
    */

    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_owned();
    let name = CString::into_raw(name);

    let menu = Box::new(Menu {
        name,
        options: std::ptr::null_mut(),
        options_size: 0,
    });
    return Box::into_raw(menu);
}

// new_menu: duplicate name into CString::into_raw()

// add_option: duplicate incoming C string
#[no_mangle]
pub extern "C" fn add_option(menu: &mut Menu, option: *const c_char) {
    /*
       Adds a new option to the menu
       PARAMS:
           menu : &mut Menu : The menu to add the option to
           option : *const c_char : The string of the option to add
    */

    if option.is_null() {
        return;
    }
    unsafe {
        let c = CStr::from_ptr(option);
        let owned = CString::new(c.to_bytes()).unwrap();
        let ptr = owned.into_raw(); // now Rust owns it

        if menu.options.is_null() {
            let v = vec![ptr].into_boxed_slice();
            menu.options = Box::into_raw(v) as *const *const c_char;
            menu.options_size = 1;
        } else {
            let len = menu.options_size as usize;
            // reconstruct box, convert to Vec, push, box again
            let mut v = Vec::from_raw_parts(menu.options as *mut *mut c_char, len, len);
            v.push(ptr);
            let new_len = v.len() as u32;
            let boxed = v.into_boxed_slice();
            menu.options = Box::into_raw(boxed) as *const *const c_char;
            menu.options_size = new_len;
        }
    }
}

#[no_mangle]
pub extern "C" fn read_option(menu: &mut Menu) -> u32 {
    /*
        Reads an option from the console
        RETURNS:
            u32 : The option read
    */
    print_menu(menu);

    loop {
        let option = read_int();
        if option > 0 && option <= menu.options_size + 1 {
            return option;
        } else {
            println!("Ingrese una opción válida");
        }
    }
}

fn print_menu(menu: &mut Menu) {
    /*
        Prints the menu to the console
        PARAMS:
            menu : &mut Menu : The menu to print
    */

    unsafe {
        println!(
            "-------- {} --------",
            CStr::from_ptr(menu.name as *const i8).to_str().unwrap()
        );
        println!("Seleccione una opción:");
        for i in 0..menu.options_size {
            println!(
                "{}. {}",
                i + 1,
                CStr::from_ptr(*menu.options.offset(i as isize) as *const i8)
                    .to_str()
                    .unwrap()
            );
        }

        println!("{}. Salir", menu.options_size + 1);
    }
}

fn read_int() -> u32 {
    loop {
        // Read the input
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => input = input.trim().to_string(),
            Err(err) => println!("{err}"),
        }

        // Parse the input
        match input.parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Ingrese una opción válida");
            }
        }
    }
}
