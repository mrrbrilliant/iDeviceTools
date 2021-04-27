use lib::{mount_device, pair_device, unmount_device, unpair_device};
use std::env;

fn main() {
    #[allow(deprecated)]
    let mount_dir = env::home_dir().unwrap().join("my_device");

    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Help");
    } else {
        match args[0].as_ref() {
            "connect" | "c" | "--connect" | "-c" => {
                if pair_device().is_ok() {
                    mount_device(&mount_dir);
                }
            }
            "disconnect" | "d" | "--disconnect" | "-d" => {
                unmount_device(&mount_dir);
                unpair_device();
            }
            _ => {
                println!("Help");
            }
        }
    }
}
