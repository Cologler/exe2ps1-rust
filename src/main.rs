use std::path::Path;
use std::process::{Command, exit};

fn main() {
    let curexe = std::env::current_exe().unwrap();
    let exepath = Path::new(curexe.to_str().unwrap());
    let script = exepath.with_extension("ps1");
    if script.is_file() {
        let mut cmd = Command::new("powershell");
        cmd.arg("-ExecutionPolicy").arg("Bypass");
        cmd.arg("-File").arg(script);
        cmd.args(std::env::args().skip(1));
        let es = cmd.spawn().unwrap().wait().unwrap();
        if !es.success() {
            std::io::stdin().read_line(&mut String::new()).unwrap_or_default();
        }
        exit(es.code().unwrap_or(0));
    } else {
        println!("Missing {} file", script.to_str().unwrap());
    }
}
