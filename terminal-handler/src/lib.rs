use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

// Define external JavaScript function that we'll call to print to console
#[link(wasm_import_module = "env")]
extern "C" {
    fn printToConsole(ptr: *const u8, len: usize);
}

// Memory allocation function
#[no_mangle]
pub extern "C" fn allocateMemory(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr as *mut c_void
}

// Memory deallocation function
#[no_mangle]
pub extern "C" fn freeMemory(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            // This is a simplified version - in production you'd want to track sizes
            let _ = Vec::<u8>::from_raw_parts(ptr as *mut u8, 0, 0);
        }
    }
}

// Helper function to print output
fn print_output(output: &str) {
    unsafe {
        printToConsole(output.as_ptr(), output.len());
    }
}

// Process commands
#[no_mangle]
pub extern "C" fn processCommand(cmd_ptr: *const c_char) {
    let cmd = unsafe {
        assert!(!cmd_ptr.is_null());
        CStr::from_ptr(cmd_ptr).to_string_lossy().into_owned()
    };

    let cmd = cmd.trim().to_lowercase();
    
    if cmd.is_empty() {
        return;
    }

    // Split command and arguments
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let command = parts[0];
    let args = &parts[1..];

    match command {
        "help" => {
            print_output("\nAvailable commands:\n");
            print_output("  help     - Show this help message\n");
            print_output("  clear    - Clear the terminal screen\n");
            print_output("  echo     - Display text\n");
            print_output("  exit     - Exit the terminal\n");
            print_output("\n");
        }
        "clear" => {
            print_output("##CLEAR##");
        }
        "echo" => {
            if !args.is_empty() {
                let output = args.join(" ");
                print_output(&format!("{}\n", output));
            } else {
                print_output("\n");
            }
        }
        "exit" => {
            print_output("\nExiting terminal...\n");
            print_output("Thank you for using ACME Terminal.\n");
        }
        _ => {
            print_output(&format!("Unknown command: {}\n", command));
            print_output("Type 'help' for a list of available commands.\n");
        }
    }
}
