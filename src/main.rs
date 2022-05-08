//! Munin plugin to take load

use anyhow::Result;
use flexi_logger::Logger;
use log::{debug, info};
use munin_plugin::{config::Config, MuninPlugin};
use procfs::LoadAverage;
use std::io::{self, BufWriter, Write};

#[derive(Debug)]
struct LoadPlugin;

impl MuninPlugin for LoadPlugin {
    fn config(&self) -> Result<()> {
        // We want to write a possibly large amount to stdout, take and lock it
        let stdout = io::stdout();
        let bufsize = 8192;
        // Buffered writer, to gather multiple small writes together
        let mut handle = BufWriter::with_capacity(bufsize, stdout.lock());

        writeln!(handle, "graph_title Load average")?;
        writeln!(handle, "graph_args --base 1000 -l 0")?;
        writeln!(handle, "graph_vlabel load")?;
        writeln!(handle, "graph_scale no")?;
        writeln!(handle, "graph_category system")?;
        writeln!(handle, "load.label load")?;
        writeln!(handle, "load.warning 10")?;
        writeln!(handle, "load.critical 120")?;
        writeln!(handle, "graph_info The load average of the machine describes how many processes are in the run-queue (scheduled to run immediately.")?;
        writeln!(handle, "load.info Average load for the five minutes.")?;
        // And flush the handle, so it can also deal with possible errors
        handle.flush()?;
        Ok(())
    }

    fn run(&self) {
        unimplemented!()
    }

    fn daemonize(&self) {
        unimplemented!()
    }
    fn acquire(&self) {
        unimplemented!()
    }
    fn fetch(&self) {
        let load = (LoadAverage::new().unwrap().five * 100.0) as isize;
        println!("load.value {}", load);
    }

    /// Check autoconf
    fn check_autoconf(&self) -> bool {
        true
    }
}

fn main() -> Result<()> {
    Logger::try_with_env_or_str("trace")
        .unwrap()
        .set_palette("b1;3;2;4;6".to_owned())
        .start()
        .unwrap();
    info!("load started");
    let config = Config::new("load".to_string());
    debug!("Plugin: {:#?}", config);

    let load = LoadPlugin;
    load.start(config)?;
    Ok(())
}
