/// Get the current user
///
/// Went the route of using the existing users crate rather than
/// re-implementing this via the libc crate.
use users::{get_current_uid, get_user_by_uid};

fn main() {
    let user = get_user_by_uid(get_current_uid());
    if let Some(u) = user {
        let name = u.name().to_str();
        if name.is_some() {
            println!("{}", name.unwrap());
        }
    }
}
