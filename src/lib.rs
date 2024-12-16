use std::ffi::CStr;

#[repr(C)]
pub struct Menu{
    pub name : *const u8,
    pub options_size : u32,
    pub options : *const *const u8
}

#[no_mangle]
pub extern "C" fn new_menu(name : *const u8) -> *mut Menu {
    /*
        Creates a new menu struct with the given name
        PARAMS:
            name : *const u8 : The name of the menu
        RETURNS:
            *mut Menu : The created menu
    */
    let menu = Box::new(Menu {
        name,
        options: std::ptr::null(),
        options_size: 0,
    });    
    return Box::into_raw(menu);
}

#[no_mangle]
pub extern "C" fn add_option(menu : &mut Menu, option : *const u8) {
    /*
        Adds a new option to the menu
        PARAMS:
            menu : &mut Menu : The menu to add the option to
            option : *const u8 : The string of the option to add
     */

    let mut options;
    if menu.options.is_null() {
        options = vec![option];
    }
    else {
        options = unsafe {
            //Convert the raw pointer to a Vec
            Vec::from_raw_parts(menu.options as *mut *const u8, menu.options_size as usize, menu.options_size as usize)
        };
        options.push(option);
    }

    menu.options = options.as_ptr(); //Convert the Vec to a raw pointer
    menu.options_size = options.len() as u32; //Update the size of the options
    std::mem::forget(options);
}

#[no_mangle]
pub extern "C" fn print_menu(menu : &mut Menu)
{
    /*
        Prints the menu to the console
        PARAMS:
            menu : &mut Menu : The menu to print
    */
    
    unsafe {
        println!("-------- {} --------", CStr::from_ptr(menu.name as *const i8).to_str().unwrap());
        for i in 0..menu.options_size {
            println!("{}. {}", i+1, CStr::from_ptr(*menu.options.offset(i as isize) as *const i8).to_str().unwrap());
        }

        println!("{}. Salir", menu.options_size + 1);
    }
}
