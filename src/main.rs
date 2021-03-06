//! Munin plugin to take load

use anyhow::Result;
use munin_plugin::{Config, MuninPlugin};
use procfs::LoadAverage;
use std::io::{BufWriter, Write};

#[derive(Debug)]
struct LoadPlugin;

impl MuninPlugin for LoadPlugin {
    fn config<W: Write>(&self, handle: &mut BufWriter<W>) -> Result<()> {
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
        Ok(())
    }

    fn acquire<W: Write>(
        &mut self,
        handle: &mut BufWriter<W>,
        _config: &Config,
        _epoch: u64,
    ) -> Result<()> {
        let load = (LoadAverage::new().unwrap().five * 100.0) as u32;
        writeln!(handle, "load.value {}", load)?;
        Ok(())
    }

    /// Check autoconf
    fn check_autoconf(&self) -> bool {
        true
    }
}

fn main() -> Result<()> {
    let mut load = LoadPlugin;
    load.simple_start("load".to_string())?;
    Ok(())
}
