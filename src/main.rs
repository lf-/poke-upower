use anyhow::anyhow;
use anyhow::{Context, Result};
use dbus::ffidisp::{ConnPath, Connection};
use dbus::strings::{BusName, Path};
use std::{thread::sleep, time::Duration};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        return Err(anyhow!("Usage: {} refresh_time_ms", args[0]));
    }
    let refresh_time = Duration::from_millis(args[1].parse::<u64>()?);

    let conn = Connection::new_system().context("Failed to connect to dbus")?;

    let cp_server = ConnPath {
        conn: &conn,
        dest: BusName::new("org.freedesktop.UPower").map_err(|err| anyhow!(err))?,
        path: Path::new("/org/freedesktop/UPower").map_err(|err| anyhow!(err))?,
        timeout: 500,
    };

    let (devices,): (Vec<dbus::Path<'static>>,) =
        cp_server.method_call("org.freedesktop.UPower", "EnumerateDevices", ())?;

    loop {
        for device in &devices {
            println!("refreshing {:?}", device);
            let cp = ConnPath {
                conn: &conn,
                dest: BusName::new("org.freedesktop.UPower").map_err(|err| anyhow::anyhow!(err))?,
                path: device.clone(),
                timeout: 500,
            };
            cp.method_call("org.freedesktop.UPower.Device", "Refresh", ())?;
        }
        sleep(refresh_time);
    }
}
