use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, exit};

fn run_ps1(script: &PathBuf) -> Result<ExitStatus, Box<dyn Error>> {
    let mut cmd = Command::new("powershell");
    cmd.arg("-NoProfile");
    cmd.arg("-ExecutionPolicy").arg("Bypass");
    cmd.arg("-File").arg(script);
    cmd.args(std::env::args().skip(1));
    let es = cmd.spawn()?.wait()?;
    Ok(es)
}

fn run_cmd(script: &PathBuf) -> Result<ExitStatus, Box<dyn Error>> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(script);
    cmd.args(std::env::args().skip(1));
    let es = cmd.spawn()?.wait()?;
    Ok(es)
}

fn find_file(base: &PathBuf, ext: &str) -> Option<PathBuf> {
    let file = base.with_extension(ext);
    return if file.exists() {
        Some(file)
    } else {
        None
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let curexe = std::env::current_exe()?;
    let exepath = PathBuf::from(curexe.to_str().unwrap());

    let es = if let Some(ps1file) = find_file(&exepath, "ps1") {
        run_ps1(&ps1file)?
    } else if let Some(batfile) = find_file(&exepath, "bat") {
        run_cmd(&batfile)?
    } else if let Some(batfile) = find_file(&exepath, "cmd") {
        run_cmd(&batfile)?
    } else {
        Err("Missing script file")?
    };

    if !es.success() {
        std::io::stdin().read_line(&mut String::new()).unwrap_or_default();
    }
    exit(es.code().unwrap_or(0));
}
