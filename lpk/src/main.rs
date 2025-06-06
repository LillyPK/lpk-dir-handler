use std::fs;
use std::path::Path;

use clap::{Arg, Command as ClapCommand};
use dialoguer::Confirm;
use sysinfo::{Disks, System}; // Modern sysinfo usage (0.35+)

// Entry point
fn main() {
    // Setup command-line interface using clap
    let matches = ClapCommand::new("lpk")
        .version("0.1.0")
        .author("April Chandler")
        .about("Enhanced directory navigation")
        .arg(Arg::new("target").help("Target directory or command"))
        .arg(
            Arg::new("ls")
                .short('l')
                .long("ls")
                .help("List available drives")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("uninstall")
                .long("uninstall")
                .help("Uninstall lpk")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Print help information")
                .action(clap::ArgAction::Help),
        )
        .get_matches();

    // Handle uninstall
    if matches.get_flag("uninstall") {
        uninstall();
        return;
    }

    // Handle drive listing
    if matches.get_flag("ls") {
        list_drives();
        return;
    }

    // Handle path argument
    if let Some(path) = matches.get_one::<String>("target") {
        change_dir(path);
    }
}

// Uninstall the binary from /usr/local/bin/lpk
fn uninstall() {
    let path = "/usr/local/bin/lpk";
    if Path::new(path).exists() {
        fs::remove_file(path).expect("Failed to remove lpk binary");
        println!("lpk has been uninstalled.");
    } else {
        println!("lpk not found in /usr/local/bin.");
    }
}

// List mounted drives with name, mount point, and size
fn list_drives() {
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.iter() {
        println!(
            "{} [{}] - {}",
            disk.name().to_string_lossy(),
            disk.mount_point().to_string_lossy(),
            bytes_to_gb(disk.total_space())
        );
    }
}

// Convert bytes to human-readable gigabytes
fn bytes_to_gb(bytes: u64) -> String {
    format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
}

// Attempt to change directory, offer to create if it doesn't exist
fn change_dir(path_str: &str) {
    let path = Path::new(path_str);

    if path.exists() && path.is_dir() {
        // Output is meant to be interpreted by shell wrapper
        println!("cd {}", path.display());
        println!("To actually change directory, use the shell wrapper script.");
    } else {
        println!("ERROR: directory missing.");
        // Ask user if they want to create it
        if Confirm::new()
            .with_prompt("Would you like to create this directory?")
            .default(true)
            .interact()
            .unwrap()
        {
            fs::create_dir_all(&path).expect("Failed to create directory");
            println!("Created and cd into {}", path.display());
        } else {
            println!("Canceled.");
        }
    }
}
