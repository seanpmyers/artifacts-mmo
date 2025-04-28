use env_logger::{Builder, Target};
use std::io::Write;

use crate::constants;

pub fn configure_logs() {
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[ {} ]:[{:^8}]:[ {} ] - {}",
                constants::APPLICATION_TITLE.to_uppercase(),
                record.level(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Warn)
        .init();
}
