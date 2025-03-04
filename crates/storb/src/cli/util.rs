use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use rusqlite::Connection;
use std::fs;

pub fn cli() -> Command {
    Command::new("util")
        .about("storb utils")
        .subcommand_required(true)
        .subcommand(
            Command::new("migrate")
                .about("Run migration for CR-SQLite")
                .arg(
                    Arg::new("db")
                        .short('d')
                        .long("db")
                        .help("Path to the SQLite database")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("extension")
                        .short('e')
                        .long("extension")
                        .help(
                            "Path to the CR-SQLite extension (e.g., crsqlite.dylib or crsqlite.so)",
                        )
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("migration")
                        .short('m')
                        .long("migration")
                        .help("Path to the migration SQL file")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
}

pub fn run_migration(db_path: &str, extension_path: &str, migration_file: &str) -> Result<()> {
    // Open a connection to the disk database.
    let conn = Connection::open(db_path)?;

    // Load the CR-SQLite extension.
    unsafe {
        conn.load_extension_enable()?;
        conn.load_extension(extension_path, None)?;
        conn.load_extension_disable()?;
    }

    // Read the migration file.
    let migration_sql = fs::read_to_string(migration_file)
        .map_err(|e| anyhow::anyhow!("Failed to read migration file: {}", e))?;

    // Execute the migration SQL.
    conn.execute_batch(&migration_sql)?;

    Ok(())
}

pub fn exec(args: &ArgMatches) {
    if let Some(sub_args) = args.subcommand_matches("migrate") {
        let db_path = sub_args
            .get_one::<String>("db")
            .expect("Database path is required");
        let extension_path = sub_args
            .get_one::<String>("extension")
            .expect("Extension path is required");
        let migration_file = sub_args
            .get_one::<String>("migration")
            .expect("Migration file path is required");

        match run_migration(db_path, extension_path, migration_file) {
            Ok(()) => println!("Migration successful"),
            Err(e) => eprintln!("Migration failed: {}", e),
        }
    }
}
