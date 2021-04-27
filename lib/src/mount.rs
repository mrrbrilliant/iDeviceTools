use crate::unmount_device;
use crate::unpair_device;

use std::{fs::create_dir_all, io::ErrorKind, path::PathBuf, process::Command};

pub fn mount_device(mount_dir: &PathBuf) {
    if mount_dir.exists() {
        let is_empty = mount_dir.read_dir().unwrap().next().is_none();
        if !is_empty {
            println!("Mount point should be empty.");
            std::process::exit(1);
        }
    } else {
        match create_dir_all(&mount_dir) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    unmount_device(&mount_dir);
                    unpair_device();
                    create_dir_all(&mount_dir).unwrap();
                }
                _ => {
                    println!("Error: {}", e)
                }
            },
        }
    }

    let mount = Command::new("ifuse")
        .arg(mount_dir.to_str().unwrap())
        .output();

    match mount {
        Ok(process) => match process.status.code() {
            Some(0) => {
                println!("Your device is mounted at: {}", mount_dir.display())
            }
            Some(_) => {
                println!(
                    "Error mountong your device with status code: {}",
                    &process.status.code().unwrap()
                );
                std::process::exit(1);
            }
            None => {
                println!("Error mounting your device with no status response.");
                std::process::exit(1);
            }
        },
        Err(e) => {
            println!("Failed mounting your device: {}", e);
            std::process::exit(1);
        }
    }
}
