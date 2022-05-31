//! ```cargo
//! [dependencies]
//! aya = { path = "../../../../aya" }
//! ```

use aya::{
    programs::{Xdp, XdpFlags},
    Bpf,
};
use std::process::Command;
use std::{convert::TryInto, process};
use std::{thread, time};

fn isLoaded() -> bool {
    let output = Command::new("bpftool").args(&["prog"]).output()?;
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout.contains("xdp  name ihaveaverylongn  tag")
}

fn assertLoaded(loaded: bool) {
    let state = isLoaded();
    if state != loaded {
        panic!("Expected loaded: {} but was loaded: {}", loaded, state);
        process::exit(0x100)
    }
}

fn main() {
    println!("Loading XDP program");
    let mut bpf = Bpf::load_file("name_test.o").unwrap();
    let dispatcher: &mut Xdp = bpf
        .program_mut("ihaveaverylongname")
        .unwrap()
        .try_into()
        .unwrap();

    dispatcher.load().unwrap();

    let link = dispatcher.attach("eth0", XdpFlags::default()).unwrap();

    dispatcher.unload(false);

    assertLoaded(true);

    dispatcher.detach(link);

    assertLoaded(false);

    dispatcher.load().unwrap();

    assertLoaded(true);

    dispatcher.attach("eth0", XdpFlags::default()).unwrap();

    assertLoaded(true);

    dispatcher.unload(true);

    assertLoaded(false);
}
