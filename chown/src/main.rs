use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use syscalls::{syscall, Errno, Sysno};

fn chown(path: &str, uid: u32, gid: u32) -> Result<usize, Errno> {
    let path = Path::new(path);
    let display = path.display();
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    let fd = file.as_raw_fd();
    println!("File descriptor: {fd}");

    match unsafe { syscall!(Sysno::fchmod, fd, uid, gid) } {
        Ok(_) => {
            // Child process
            Ok(0)
        }
        Err(err) => {
            eprintln!("chmod() failed: {}", err);
            Err(err)
        }
    }

    // let fd = syscall::open(path, syscall::O_STAT)?;
    // let res = syscall::fchown(fd, uid, gid);
    // let _ = syscall::close(fd);
    // res
}

fn main() {
    chown("/Users/adam/src/rust/coreutils/test.txt", 501, 20).unwrap();
}
