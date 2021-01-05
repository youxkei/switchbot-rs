use rumble::api::Central;
use rumble::api::Peripheral;
use rumble::bluez::manager::Manager;

use std::thread;
use std::time::Duration;

fn main() {
    let manager = Manager::new().unwrap();

    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    let central = adapter.connect().unwrap();
    central.start_scan().unwrap();

    thread::sleep(Duration::from_secs(2));

    for peripheral in central.peripherals().into_iter() {
        println!("{:?}", peripheral.properties().local_name);
    }
}
