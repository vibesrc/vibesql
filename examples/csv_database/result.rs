//! Query result formatting.

/// A row of data (column values as strings).
pub type Row = Vec<String>;

/// Output format for query results.
#[derive(Clone, Copy, Default)]
pub enum OutputFormat {
    #[default]
    Table,
    Csv,
}

/// Query result.
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}

impl QueryResult {
    /// Create a new result with columns and rows.
    pub fn new(columns: Vec<String>, rows: Vec<Row>) -> Self {
        Self { columns, rows }
    }

    /// Print in the specified format.
    pub fn print_format(&self, format: OutputFormat) {
        match format {
            OutputFormat::Table => self.print(),
            OutputFormat::Csv => self.print_csv(),
        }
    }

    /// Print as CSV.
    pub fn print_csv(&self) {
        print!("{}", self.format_csv());
    }

    /// Format as a string in the specified format.
    pub fn format_string(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::Table => self.format_table(),
            OutputFormat::Csv => self.format_csv(),
        }
    }

    /// Format as CSV string.
    pub fn format_csv(&self) -> String {
        if self.columns.is_empty() {
            return String::new();
        }

        let mut output = String::new();

        // Header
        output.push_str(
            &self
                .columns
                .iter()
                .map(|c| escape_csv(c))
                .collect::<Vec<_>>()
                .join(","),
        );
        output.push('\n');

        // Rows
        for row in &self.rows {
            output.push_str(
                &row.iter()
                    .map(|v| escape_csv(v))
                    .collect::<Vec<_>>()
                    .join(","),
            );
            output.push('\n');
        }

        output
    }

    /// Format as table string.
    pub fn format_table(&self) -> String {
        if self.columns.is_empty() {
            return "(empty result)\n".to_string();
        }

        let mut output = String::new();

        // Calculate column widths
        let mut widths: Vec<usize> = self.columns.iter().map(|c| c.len()).collect();
        for row in &self.rows {
            for (i, val) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(val.len());
                }
            }
        }

        // Header
        let header: Vec<String> = self
            .columns
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:width$}", c, width = widths[i]))
            .collect();
        output.push_str(&format!("| {} |\n", header.join(" | ")));

        // Separator
        let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
        output.push_str(&format!("+-{}-+\n", sep.join("-+-")));

        // Rows
        for row in &self.rows {
            let formatted: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let width = widths.get(i).copied().unwrap_or(0);
                    format!("{:width$}", v, width = width)
                })
                .collect();
            output.push_str(&format!("| {} |\n", formatted.join(" | ")));
        }

        output.push_str(&format!("\n({} rows)\n", self.rows.len()));
        output
    }

    /// Print as a formatted table.
    pub fn print(&self) {
        if self.columns.is_empty() {
            println!("(empty result)");
            return;
        }

        // Calculate column widths
        let mut widths: Vec<usize> = self.columns.iter().map(|c| c.len()).collect();
        for row in &self.rows {
            for (i, val) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(val.len());
                }
            }
        }

        // Print header
        let header: Vec<String> = self
            .columns
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:width$}", c, width = widths[i]))
            .collect();
        println!("| {} |", header.join(" | "));

        // Print separator
        let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
        println!("+-{}-+", sep.join("-+-"));

        // Print rows
        for row in &self.rows {
            let formatted: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let width = widths.get(i).copied().unwrap_or(0);
                    format!("{:width$}", v, width = width)
                })
                .collect();
            println!("| {} |", formatted.join(" | "));
        }

        println!("\n({} rows)", self.rows.len());
    }
}

/// Escape a value for CSV output.
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
