use std::os;
use std::io::Command;

fn main() {
    let args = os::args();
    Command::new("git").arg("commit").arg("-am").arg(args[2].to_string()).status();
    Command::new("git").arg("push").status();
}
