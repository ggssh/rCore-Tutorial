use crate::{sbi::console_getchar, task::suspend_current_and_run_next};

const FD_STDOUT: usize = 1;
const FD_STDIN: usize = 0;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "Read a char");
            let mut c: usize;
            loop {
                c = console_getchar();
                if c == 0 {
                    suspend_current_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            println!("Read a char from console : \x1b[31m{}\x1b[0m", ch as char);
            1
        }
        _ => {
            panic!("Unsupported fs in sys_read!");
        }
    }
}
