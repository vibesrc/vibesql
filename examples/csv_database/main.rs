//! A simple CSV-backed database using vibesql.
//!
//! This example demonstrates how to use vibesql as a SQL frontend
//! for a simple CSV file storage backend.
//!
//! Run with: cargo run --example csv_database

mod database;
mod execution;
mod result;
mod seed;

use std::io::Write;
use std::path::PathBuf;

use database::CsvDatabase;
use result::OutputFormat;

fn print_usage() {
    eprintln!("Usage: csv_database [COMMAND] [OPTIONS]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  seed         Seed the database with sample relational data");
    eprintln!("  clear        Clear all tables from the database");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -c <SQL>     Execute SQL query and exit");
    eprintln!("  -o <FILE>    Output results to FILE (format from extension: .csv or .txt)");
    eprintln!("  -d <DIR>     Use DIR as data directory (default: examples/data)");
    eprintln!("  -q           Quiet mode (no banner, useful with -c)");
    eprintln!("  --csv        Output results in CSV format (to stdout)");
    eprintln!("  -h, --help   Show this help");
    eprintln!();
    eprintln!("Without a command or -c, starts an interactive shell.");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  csv_database seed                    # Seed with sample data");
    eprintln!("  csv_database -c 'SELECT * FROM employees'");
    eprintln!("  csv_database -c 'SELECT * FROM employees' -o results.csv");
    eprintln!("  csv_database --csv -c 'SELECT * FROM employees'");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut data_dir: Option<PathBuf> = None;
    let mut query: Option<String> = None;
    let mut quiet = false;
    let mut command: Option<String> = None;
    let mut output_format: Option<OutputFormat> = None;
    let mut output_file: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "seed" | "clear" => {
                command = Some(args[i].clone());
                i += 1;
            }
            "--csv" => {
                output_format = Some(OutputFormat::Csv);
                i += 1;
            }
            "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    eprintln!("Error: -o requires a file path argument");
                    std::process::exit(1);
                }
            }
            "-c" => {
                if i + 1 < args.len() {
                    query = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: -c requires a SQL query argument");
                    std::process::exit(1);
                }
            }
            "-d" => {
                if i + 1 < args.len() {
                    data_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    eprintln!("Error: -d requires a directory argument");
                    std::process::exit(1);
                }
            }
            "-q" => {
                quiet = true;
                i += 1;
            }
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            arg => {
                eprintln!("Unknown argument: {}", arg);
                print_usage();
                std::process::exit(1);
            }
        }
    }

    // Default data directory
    let data_dir = data_dir.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap_or_default()
            .join("examples")
            .join("data")
    });

    // Handle commands
    if let Some(cmd) = command {
        match cmd.as_str() {
            "seed" => {
                // Clear and seed the database
                let mut db = CsvDatabase::new(&data_dir)?;
                seed::clear_database(&mut db, &data_dir)?;
                seed::seed_database(&mut db)?;
                return Ok(());
            }
            "clear" => {
                let mut db = CsvDatabase::new(&data_dir)?;
                seed::clear_database(&mut db, &data_dir)?;
                println!("Database cleared.");
                return Ok(());
            }
            _ => unreachable!(),
        }
    }

    let mut db = CsvDatabase::new(&data_dir)?;

    // If -c was provided, just execute query and exit (don't auto-create tables)
    if let Some(sql) = query {
        match db.execute(&sql) {
            Ok(result) => {
                // Determine output format: explicit --csv, or infer from file extension
                let format = output_format.unwrap_or_else(|| {
                    if let Some(ref path) = output_file {
                        if path.extension().map(|e| e == "csv").unwrap_or(false) {
                            OutputFormat::Csv
                        } else {
                            OutputFormat::Table
                        }
                    } else {
                        OutputFormat::Table
                    }
                });

                if let Some(ref path) = output_file {
                    // Write to file
                    let content = result.format_string(format);
                    std::fs::write(path, content)?;
                    eprintln!("Results written to {}", path.display());
                } else {
                    // Print to stdout
                    result.print_format(format);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Create minimal sample tables if database is empty (interactive mode only)
    if !db.has_table("employees") && !db.has_table("departments") && !quiet {
        println!("No tables found. Run 'csv_database seed' for sample data.\n");
    }

    // Interactive mode
    if !quiet {
        println!("CSV Database Example");
        println!("====================\n");
        println!("Data directory: {}\n", data_dir.display());

        // Show available tables
        println!("Tables:");
        match db.execute("SELECT table_name FROM information_schema.tables") {
            Ok(result) => {
                for row in &result.rows {
                    if let Some(name) = row.first() {
                        println!("  - {}", name);
                    }
                }
            }
            Err(_) => println!("  (none)"),
        }
        println!();

        // Run some demo queries
        let queries = [
            "SELECT * FROM employees LIMIT 5",
            "SELECT e.name, d.name FROM employees e JOIN departments d ON e.department_id = d.id LIMIT 5",
        ];

        for sql in queries {
            println!("Query: {}\n", sql);
            match db.execute(sql) {
                Ok(result) => result.print(),
                Err(e) => println!("Error: {}", e),
            }
            println!();
        }
    }

    println!("Enter SQL queries (or 'quit' to exit):");
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        print!("csv> ");
        std::io::stdout().flush()?;
        input.clear();
        stdin.read_line(&mut input)?;

        let sql = input.trim();
        if sql.is_empty() {
            continue;
        }
        if sql.eq_ignore_ascii_case("quit") || sql.eq_ignore_ascii_case("exit") {
            break;
        }

        match db.execute(sql) {
            Ok(result) => result.print(),
            Err(e) => println!("Error: {}", e),
        }
        println!();
    }

    Ok(())
}
