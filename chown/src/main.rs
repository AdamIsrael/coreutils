// Note: the syscalls crate doesn't work with bsd/osx yet:
// https://github.com/jasonwhite/syscalls/issues/31
use std::fs;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use syscalls::{syscall, Errno, Sysno};

use clap::{ArgAction, CommandFactory, Parser};
use users::{get_group_by_name, get_user_by_name};

#[derive(Clone, Debug)]
struct UserGroup {
    user: String,
    group: String,
}

#[derive(Parser, Debug)]
#[command(author, version, arg_required_else_help(true), disable_help_flag(true), about, long_about = None)]
struct Args {
    /// like verbose but report only when a change is made
    #[arg(short = 'c', long)]
    changes: bool,

    /// suppress most error messages
    #[arg(short = 'f', long, visible_alias("quiet"))]
    silent: bool,

    /// affect the referent of each symbolic link (this is the default), rather than the symbolic link itself
    #[arg(long, action = ArgAction::SetFalse)]
    dereference: bool,

    /// affect symbolic links instead of any referenced file
    /// (useful only on systems that can change the
    /// ownership of a symlink)   
    #[arg(short = 'h', long)]
    no_dereference: bool,

    /// Display help
    #[arg(long)]
    help: bool,

    /// change the owner and/or group of each file only if
    /// its current owner and/or group match those specified
    /// here.  Either may be omitted, in which case a match
    /// is not required for the omitted attribute    
    #[arg(name(
        "from=CURRENT_OWNER:CURRENT_GROUP"),
        long,
        value_parser = parse_user_group
    )]
    from: Option<UserGroup>,

    /// do not treat '/' specially (the default)
    #[arg(long, action = ArgAction::SetFalse)]
    no_preserve_root: bool,

    /// fail to operate recursively on '/'
    #[arg(long)]
    preserve_root: bool,

    /// use RFILE's owner and group rather than
    /// specifying OWNER:GROUP values
    #[arg(name("reference=RFILE"), long)]
    reference: Option<String>,

    /// operate on files and directories recursively
    #[arg(short('R'), long)]
    recursive: bool,

    /// The following options modify how a hierarchy is traversed when the -R
    /// option is also specified.  If more than one is specified, only the final
    /// one takes effect.

    /// if a command line argument is a symbolic link to a directory, traverse it
    #[arg(short('H'), requires("recursive"))]
    start_symbolic: bool,

    /// traverse every symbolic link to a directory encountered
    #[arg(short('L'), requires("recursive"))]
    follow_all_symbolic_links: bool,

    /// do not traverse any symbolic links (default)
    #[arg(short('P'), requires("recursive"), action = ArgAction::SetFalse)]
    do_not_follow: bool,

    /// output a diagnostic for every file processed
    #[arg(short('v'), long)]
    verbose: bool,

    /// The owner and/or group to change
    #[arg(name("[OWNER][:[GROUP]]"), value_parser = parse_user_group)]
    ug: UserGroup,

    /// A list of file(s) to change the ownership of
    files: Vec<String>,
}

fn parse_user_group(arg: &str) -> Result<UserGroup, std::num::ParseIntError> {
    let v: Vec<&str> = arg.split(':').collect();

    let ug = UserGroup {
        user: v[0].to_string(),
        group: if v.len() == 2 {
            String::from(v[1])
        } else {
            String::from("")
        },
    };
    Ok(ug)
}

/// Change the ownership of a file (Linux-only right now)
///
/// TODO: return a better result that indicates if a change was made
fn chown(path: &str, uid: u32, gid: Option<u32>) -> Result<usize, Errno> {
    let path = Path::new(path);
    let display = path.display();
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    let fd = file.as_raw_fd();

    if gid.is_some() {
        match unsafe { syscall!(Sysno::fchown, fd, uid, gid.unwrap()) } {
            Ok(_) => {
                // Child process
                Ok(0)
            }
            Err(err) => {
                eprintln!("chmod() failed: {}", err);
                Err(err)
            }
        }
    } else {
        match unsafe { syscall!(Sysno::fchown, fd, uid) } {
            Ok(_) => {
                // Child process
                Ok(0)
            }
            Err(err) => {
                eprintln!("chmod() failed: {}", err);
                Err(err)
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    // I don't think this is being reached; Clap is detecting --help and printing
    // the short help. The only way to get the long help is to run `chown` by itself
    if args.help {
        let mut cmd = Args::command();
        cmd.print_long_help().unwrap();
        return;
    }

    // Get the uid from the user name
    let user = get_user_by_name(&args.ug.user);
    let uid: u32 = match user {
        Some(u) => u.uid(),
        None => {
            eprintln!("chown: invalid user: '{}'", args.ug.user);
            return;
        }
    };

    // Get the gid from the group name
    let group = get_group_by_name(&args.ug.group);
    let gid: Option<u32> = group.map(|g| g.gid());

    println!("{}:{:?}", uid, gid);

    if !args.files.is_empty() {
        for filename in &args.files {
            // Is this file a directory?
            let md = fs::metadata(filename).unwrap();

            // Only act recursively if we're given a directory
            if args.recursive && md.is_dir() {
                let paths = get_paths(filename.to_string());
                println!("Paths: {:?}", paths);

                for path in paths {
                    match chown(&path, uid, gid) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("chown: {}", err);
                        }
                    };

                    // need to return output from `chown` indicating if a change was
                    // made or if ownership was retained.
                    if args.verbose {
                        println!("ownership of 'test.txt' retained as stone:stone");
                    }
                }
            } else {
                match chown(filename, uid, gid) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("chown: {}", err);
                    }
                };
            }
        }
    }
}

fn get_paths(directory: String) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    // scan the directory recursively and add all files in it.
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let s_path = path.clone().into_os_string().into_string().unwrap();

        paths.push(s_path.clone());

        if path.is_dir() {
            paths.append(&mut get_paths(s_path));
        }
    }
    paths
}
