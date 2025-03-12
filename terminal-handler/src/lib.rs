use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::path::PathBuf;

static FS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fs.bin"));
const HOME: &str = "/home/nishant/";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PackedFs {
    files: Vec<(String, Vec<u8>)>,
}

fn initialize_fs_structure(files: &mut HashMap<String, Vec<String>>) {
    let (packed_fs, _): (PackedFs, _) =
        bincode::serde::decode_from_slice(FS, bincode::config::standard()).unwrap();

    for (name, data) in packed_fs.files {
        let name = format!("{}{}", HOME, name);
        let data = String::from_utf8(data).unwrap();
        let lines: Vec<String> = data.lines().map(|s| s.to_string()).collect();
        files.insert(name, lines);
    }
}

// Define external JavaScript function that we'll call to print to console
#[link(wasm_import_module = "env")]
extern "C" {
    fn printToConsole(ptr: *const u8, len: usize);
    fn msgHistory();
    fn sendMessage(ptr: *const u8, len: usize);
}

// Helper function to print output
fn print_output(output: &str) {
    unsafe {
        printToConsole(output.as_ptr(), output.len());
    }
}

fn print_msg_history() {
    unsafe {
        msgHistory();
    }
}

fn send_message(msg: &str) {
    unsafe {
        sendMessage(msg.as_ptr(), msg.len());
    }
}

mod utils;
mod vfs;
use vfs::VirtualFS;

// Global filesystem state
static mut FS_STATE: Option<VirtualFS> = None;

// Initialize the virtual filesystem
fn init_fs() {
    let mut files = HashMap::new();

    initialize_fs_structure(&mut files);

    // Make sure root directory exists

    let fs = VirtualFS {
        current_dir: PathBuf::from(HOME),
        root_dir: PathBuf::from("/"),
        files,
    };

    unsafe {
        FS_STATE = Some(fs);
    }
}

// Get filesystem state reference
fn get_fs() -> &'static mut VirtualFS {
    unsafe {
        if FS_STATE.is_none() {
            init_fs();
        }
        FS_STATE.as_mut().unwrap()
    }
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

// Initialize the filesystem
#[no_mangle]
pub extern "C" fn init() {
    // Clear existing state and initialize a fresh filesystem
    unsafe {
        FS_STATE = None;
    }
    init_fs();
}

// Process commands
#[no_mangle]
pub extern "C" fn processCommand(cmd_ptr: *const c_char) {
    // Initialize filesystem if not already done
    unsafe {
        if FS_STATE.is_none() {
            init_fs();
        }
    }

    let cmd = unsafe {
        assert!(!cmd_ptr.is_null());
        CStr::from_ptr(cmd_ptr).to_string_lossy().into_owned()
    };

    let cmd = cmd.trim();

    if cmd.is_empty() {
        return;
    }

    // Split command and arguments
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    let command = parts[0].to_lowercase();
    let args = parts.get(1).unwrap_or(&"").to_string();

    match command.as_str() {
        "help" => {
            print_output("\nAvailable commands:\n");
            print_output("  cat [file]   - Display file contents\n");
            print_output("  cd [path]    - Change directory\n");
            print_output("  clear        - Clear the terminal screen\n");
            print_output("  echo [text]  - Display text\n");
            print_output("  exit         - Exit the terminal\n");
            print_output("  help         - Show this help message\n");
            print_output("  icat [file]  - Display image\n");
            print_output("  ls [path]    - List directory contents\n");
            print_output("  msg list     - List message history\n");
            print_output("  msg send [text] - Send a message\n");
            print_output("  pwd          - Print working directory\n");
            print_output("  whoami       - Display user information\n");
            print_output("\n");
        }
        "clear" => {
            print_output("##CLEAR##");
        }
        "exit" => {
            print_output("##EXIT##");
        }
        "echo" => {
            print_output(&format!("{}\n", args));
        }
        "pwd" => {
            let fs = get_fs();
            print_output(&format!("{}\n", fs.current_dir.to_string_lossy()));
        }
        "ls" => {
            let fs = get_fs();

            let path = args.trim();
            fs.list_dir(path).iter().for_each(|f| {
                print_output(&format!("{}\n", f));
            });
        }
        "cd" => {
            let fs = get_fs();
            fs.change_dir(args.trim());
        }
        "cat" => {
            let fs = get_fs();
            let path = args.trim();
            match fs.cat_file(path) {
                Some(it) => {
                    for line in it {
                        print_output(&format!("{}\n", line));
                    }
                }
                None => {
                    print_output(&format!("cat: {}: No such file or directory\n", path));
                }
            }
        }
        "icat" => {
            let fs = get_fs();
            let path = args.trim();
            match fs.icat_file(path) {
                Some(p) => {
                    print_output(&format!("##ICAT:fs/{}##\n", p));
                }
                None => {
                    print_output(&format!("icat: {}: No such file or directory\n", path));
                }
            }
        }
        "whoami" => {
            utils::bio().for_each(|x| print_output(&x));
        }
        "msg" => {
            // split args at the first space
            let parts: Vec<&str> = args.splitn(2, ' ').collect();
            let msg = parts.get(1).unwrap_or(&"").to_string();
            match parts[0] {
                "send" => send_message(&msg),
                "list" => print_msg_history(),
                _ => print_output("Unknown message command\n"),
            }
        }
        _ => {
            print_output(&format!("Unknown command: {}\n", command));
            print_output("Type 'help' for a list of available commands.\n");
        }
    }
}
