extern crate ansi_term;
extern crate env_logger;
extern crate floating_duration;
extern crate log;

use self::ansi_term::Color;
use self::log::Level;
use std::fmt;
use std::time::Instant;

struct ColorLevel(Level);

impl fmt::Display for ColorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Level::Trace => Color::Purple.paint("TRACE"),
            Level::Debug => Color::Blue.paint("DEBUG"),
            Level::Info => Color::Green.paint("INFO"),
            Level::Warn => Color::Yellow.paint("WARNING"),
            Level::Error => Color::Red.paint("ERROR"),
        }.fmt(f)
    }
}

pub fn init_logger() {
    let init_time = Instant::now();
    let env = env_logger::Env::default();
    let mut builder = env_logger::Builder::from_env(env);
    builder.format(move |buf, record| {
        use std::io::Write;
        let line_string = match record.line() {
            Some(x) => format!("{}", x),
            None => String::from("-"),
        };

        let timestamp = Instant::now() - init_time;
        writeln!(
            buf,
            "{} {} [{}:{}] {}",
            ColorLevel(record.level()),
            floating_duration::TimeFormat(timestamp),
            record.file().unwrap_or("-"),
            line_string,
            record.args()
        )
    });

    builder.init();
}
