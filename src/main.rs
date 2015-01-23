use std::io::Command;

fn main() {
    Command::new("git").arg("commit").arg("-am").arg("Remove --force from push.").status();
    Command::new("git").arg("push").status();
}
