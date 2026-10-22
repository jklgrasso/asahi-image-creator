use clap::arg;
use loopdev::{LoopControl, LoopDevice};
use std::{
    env::consts::OS,
    fs::File,
    os::fd::AsFd,
    path::{Path, PathBuf},
    ptr::null,
};

// unused, but keeping for the moment for potential reference
// pub fn create() -> io::Result<()> {
//     let entries = fs::read_dir(".")?
//         .map(|res| res.map(|e| e.path().display().to_string()))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     for entry in entries {
//         println!("{}", entry);
//     }

//     let wd = std::env::current_dir()?;
//     println!("Current working directory: {}", wd.display());

//     Ok(())
// }

// https://apple.stackexchange.com/questions/9284/does-mac-have-something-similar-to-a-linux-loop-device-alternative-to-losetup
// probably needs to be os specific

// #[derive(Debug)]
pub struct Image {
    pub name: String,
    pub mount_device: (),
    pub detach: (),
}

impl Image {
    fn darwin(path: PathBuf) -> Image {
        let name = "none".to_string();
        let mount_device = null();
        let detach = null();

        return Image {
            name,
            mount_device,
            detach,
        };
    }

    fn linux(path: PathBuf) -> Image {
        let loopcontrol = LoopControl::open()
            .ok()
            .expect("Failed to open loop-control")
            .next_free()
            .expect("Failed to create loop device");

        let mount_device = loopcontrol
            .attach_fd(path.as_fd())
            .expect("Failed to attach file to loop device");

        let detach = LoopDevice::detach(
            &LoopDevice::open(PathBuf::from(
                &loopcontrol
                    .path()
                    .expect("Failed to get loop device path for detachment"),
            ))
            .map_err(|error| println!("{}", error))
            .unwrap(),
        )
        .expect("Failed to detach loop device");

        return Image {
            name,
            mount_device,
            detach,
        };
    }

    fn windows(path: PathBuf) {
        return Image {
            name,
            mount_device,
            detach,
        };
    }

    pub fn loopback() -> Image {
        let iso = clap::Command::new("iso")
            .args([arg!(
                -i
                --iso
                <ISO_PATH>
            )
            .required(true)
            .help("help text")])
            .get_matches();

        let os = OS;

        let name = iso.get_one::<String>("iso").unwrap().to_string();

        let path = File::open(name.clone())
            .unwrap()
            .as_fd()
            .try_clone_to_owned()
            .expect("Failed to open file");

        match os {
            macos => darwin(path),
            linux => linux(path),
            windows => windows(path),
            _ => Err((format!("System not supported."))),
        }

        return Image {
            name,
            mount_device,
            detach,
        };
    }
}
