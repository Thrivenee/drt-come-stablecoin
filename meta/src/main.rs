// meta/src/main.rs
use std::process::Command;

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // Skip executable name

    match args.next().as_deref() {
        Some("build") => {
            let output = Command::new("dharitri-sc-meta")
                .arg("build")
                .arg("--crate-name")
                .arg("come-stablecoin")
                .arg("--crate-version")
                .arg("0.1.0")
                .output()
                .expect("Failed to execute dharitri-sc-meta build command");

            if output.status.success() {
                println!("Contract build successful");
            } else {
                eprintln!("Contract build failed");
                std::process::exit(1);
            }
        },
        Some("test") => {
            let output = Command::new("dharitri-sc-meta")
                .arg("test")
                .arg("--crate-name")
                .arg("come-stablecoin")
                .arg("--crate-version")
                .arg("0.1.0")
                .output()
                .expect("Failed to execute dharitri-sc-meta test command");

            if output.status.success() {
                println!("Contract tests successful");
            } else {
                eprintln!("Contract tests failed");
                std::process::exit(1);
            }
        },
        Some("clean") => {
            let output = Command::new("dharitri-sc-meta")
                .arg("clean")
                .arg("--crate-name")
                .arg("come-stablecoin")
                .arg("--crate-version")
                .arg("0.1.0")
                .output()
                .expect("Failed to execute dharitri-sc-meta clean command");

            if output.status.success() {
                println!("Contract cleaning successful");
            } else {
                eprintln!("Contract cleaning failed");
                std::process::exit(1);
            }
        },
        Some("abi") => {
            let output = Command::new("dharitri-sc-meta")
                .arg("abi")
                .arg("--crate-name")
                .arg("come-stablecoin")
                .arg("--crate-version")
                .arg("0.1.0")
                .output()
                .expect("Failed to execute dharitri-sc-meta abi command");

            if output.status.success() {
                println!("ABI generation successful");
            } else {
                eprintln!("ABI generation failed");
                std::process::exit(1);
            }
        },
        Some(other_command) => {
            panic!("Unknown command: {}", other_command);
        },
        None => {
            panic!("No command provided");
        },
    }
}