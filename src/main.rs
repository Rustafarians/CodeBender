use std::io::Command;

fn main() {
    Command::new("git").arg("commit").arg("-am").arg("Git commit & push on program execution.").status();
    Command::new("git").arg("push").arg("--force").status();
}
