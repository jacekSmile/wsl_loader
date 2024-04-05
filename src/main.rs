use std::env;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Please provide a linux_distro, a command and a file path as arguments.");
        return Ok(());
    }

    // 获取linux发行版
    let linux_distro = &args[1];
    // 获取命令
    let command = &args[2];
    // 获取文件路径
    let file_path = &args[3];

    // 执行命令 wsl -d <linux_distro> <command> <file_path>
    let output = Command::new("wsl")
        .arg("-d")
        .arg(linux_distro)
        .arg(command)
        .arg(&format!("`wslpath -au '{}'`", file_path))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("\x1B[34mCommand output:\x1B[0m\n\x1B[32m{}\x1B[0m", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("\x1B[34mCommand error:\x1B[0m\n\x1B[31m{}\x1B[0m", s);
    }

    // 暂停程序
    println!("Press enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    Ok(())
}
