use std::io::Command;

fn main() {
    Command::new("git").arg("commit").arg("-am").arg("Build on Travis CI.").status();
    Command::new("git").arg("push").status();
}
