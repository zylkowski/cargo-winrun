use std::{
    env,
    io::Read,
    process::{Command, Stdio},
};

use cargo_metadata::Message;
// use clap::Parser;
//
// #[derive(Parser)]
// enum Args {
//     Winrun {
//         // #[arg(allow_hyphen_values(true))]
//         args: Vec<String>,
//     },
// }

fn main() {
    let args: Vec<String> = env::args().skip(2).collect();
    let mut args_iter = args.split_inclusive(|arg| *arg == "--");
    let mut build_args = args_iter.next().unwrap_or(&[]).to_vec();
    if !build_args.is_empty() && build_args.last().unwrap() == "--" {
        build_args.pop();
    }
    let exe_args: Vec<_> = args_iter
        .next()
        .unwrap_or(&[])
        .iter()
        .map(|a| a.replace('/', "\\"))
        .map(|a| format!("\"{}\"", a))
        .collect();
    // println!("Build args: {:?}", build_args);
    // println!("exe args: {:?}", exe_args);

    println!("Starting compilation!");
    let mut output = Command::new("cargo")
        .arg("build")
        .args(build_args)
        .args(["--target", "x86_64-pc-windows-gnu"])
        .args(["--message-format=json-render-diagnostics"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Expected build to succeed");

    let mut buf: String = String::new();
    output
        .stdout
        .as_mut()
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let new_buf: Vec<_> = buf
        .split('\n')
        .rev()
        .collect::<Vec<_>>()
        .into_iter()
        .take(3)
        .rev()
        .collect();

    output.wait().unwrap();
    println!("Compilation complete!");

    let _success = {
        if let Message::BuildFinished(build_info) = serde_json::from_str(new_buf[1]).unwrap() {
            build_info.success
        } else {
            return;
        }
    };
    let exe_path = {
        if let Message::CompilerArtifact(artifact_info) = serde_json::from_str(new_buf[0]).unwrap()
        {
            artifact_info.executable.clone().unwrap().to_string()
        } else {
            return;
        }
    };

    println!("Build success, exe location: {:?}", exe_path);

    let win_vars: String = std::env::vars()
        .filter(|(k, _)| k.starts_with("WIN_"))
        .map(|(k, v)| format!("$env:{}='{}'; ", &k[4..], v))
        .fold("".to_owned(), |mut acc, v| {
            acc += &v;
            acc
        });
    println!("Windows envvars:\n{}", win_vars);
    let command = format!("& {{ {}; {} {} }}", win_vars, exe_path, exe_args.join(" "));

    // println!("Exe args: {:?}", exe_args);
    // println!("command: {:?}", command);

    let mut exec_out =
        Command::new("/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe")
            .arg("-Command")
            .arg(command)
            .spawn()
            .expect("Expected exec to work");
    exec_out.wait().unwrap();
}
