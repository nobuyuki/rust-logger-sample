#![allow(unused_imports)]

use std::io::Write;
use std::marker::PhantomData;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

struct Logger {
    phantom: PhantomData<()>,
}

impl Logger {
    fn default() -> Self {
        Logger {
            phantom: PhantomData,
        }
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        println!("Logger::enabled() called.");
        false
    }

    fn log(&self, record: &Record) {
        println!("Loger::log() called with: {:#?}", record);
    }

    fn flush(&self) {
        println!("Loger::flush() called.");
    }
}

pub fn try_init() -> Result<(), SetLoggerError> {
    let logger = Logger::default();

    let r = log::set_boxed_logger(Box::new(logger));

    if r.is_ok() {
        log::set_max_level(LevelFilter::Trace);
    }

    println!("try_init(): {:?}", r);

    r
}

pub fn init() {
    println!("init()");

    try_init().expect("init() failed.");
}
