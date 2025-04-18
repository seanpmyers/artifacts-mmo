use anyhow::Result;
use env_logger::{Builder, Target};
use std::io::Write;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn configure_logging() -> Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[ARTIFACTS_MMO]:[{:<5}]:[{:<27}] - {}",
                record.level(),
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();

    Ok(())
}
