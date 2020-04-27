use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use std::io::Write;
use super::gpuclock::*;

pub struct Profiler {
    times: HashMap<String, Vec<Duration>>,
    clock: Instant,
    gpuclock: GPUClock,
    enabled: bool,
    name: String,
    name_gpu: String,
}

impl Profiler {
    pub fn new(enabled: bool) -> Profiler {
        Profiler {
            times: HashMap::new(),
            clock: Instant::now(),
            gpuclock: GPUClock::new(),
            enabled,
            name: Default::default(),
            name_gpu: Default::default(),
        }
    }
    
    pub fn begin(&mut self, name: &str) {
        if !self.enabled {return;}
        self.clock = Instant::now();
        self.name = name.to_owned();
    }
    
    
    pub fn begin_gpu(&mut self, name: &str) {
        if !self.enabled {return;}
        self.gpuclock.start();
        self.name_gpu = name.to_owned();
    }
    
    pub fn end(&mut self) {
        if !self.enabled {return;}
        let v = self.times.entry(self.name.clone()).or_insert(vec![]);
        v.push(self.clock.elapsed());
    }
    
    pub fn end_gpu(&mut self) {
        if !self.enabled {return;}
        let v = self.times.entry(self.name_gpu.clone()).or_insert(vec![]);
        v.push(self.gpuclock.time());
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.gpuclock.stop();
        }
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn print(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        for entry in &self.times {
            write!(file,"{}: [",entry.0)?;
            for t in entry.1 {
                write!(file,"{},",t.as_micros())?;
            }
            write!(file,"]\n")?;
        }
        Ok(())
    }
}
