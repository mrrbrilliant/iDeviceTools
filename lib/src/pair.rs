use std::{
    io::{Error, ErrorKind},
    process::Command,
};

pub fn pair_device() -> Result<(), Error> {
    let pair = Command::new("idevicepair").arg("pair").output();

    match pair {
        Ok(process) => match process.status.code() {
            Some(0) => {
                println!("Success");
                Ok(())
            }
            Some(_) => {
                println!("Failed to pair your device.");

                let exit_message = String::from_utf8_lossy(&process.stdout);
                if exit_message.contains("No device found.") {
                    println!("Please connect your device");
                }
                if exit_message
                    .contains("ERROR: Please accept the trust dialog on the screen of device")
                {
                    println!("Please accept the trust dialog on the screen of your device");
                }
                std::process::exit(1);
            }
            None => {
                println!("Bad status: no status response.");
                Err(Error::new(ErrorKind::Other, "No status code response"))
            }
        },
        Err(e) => {
            eprintln!("Error pairing device. {}", &e);
            Err(e)
        }
    }
}
