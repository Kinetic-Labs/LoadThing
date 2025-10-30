use std::fmt;

use crate::helpers::ansi;

#[derive(Clone)]
pub struct TableLogger {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<usize>,
    has_logged_header: bool,
}

impl TableLogger {
    pub fn new(headers: Vec<String>) -> Self {
        let column_widths = headers.iter().map(|h| h.len()).collect();

        Self {
            headers,
            rows: Vec::new(),
            column_widths,
            has_logged_header: false,
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        for (i, cell) in row.iter().enumerate() {
            if i < self.column_widths.len() {
                self.column_widths[i] = self.column_widths[i].max(cell.len());
            }
        }

        self.rows.push(row);
    }

    pub fn log(&mut self) {
        let mut output = String::new();

        output.push_str(ansi::GREEN);

        if !self.has_logged_header {
            output.push_str("| ");
            for (i, header) in self.headers.iter().enumerate() {
                output.push_str(&format!(
                    "{:<width$}",
                    header,
                    width = self.column_widths[i]
                ));
                output.push_str(" | ");
            }
            output.push('\n');

            output.push_str("|-");
            for width in &self.column_widths {
                output.push_str(&"-".repeat(*width));
                output.push_str("-|-");
            }
            output.push_str("|");
            output.pop();
            output.pop();
            output.push('\n');
            self.has_logged_header = true;
        }

        for row in &self.rows {
            output.push_str("| ");
            for (i, cell) in row.iter().enumerate() {
                let width = if i < self.column_widths.len() {
                    self.column_widths[i]
                } else {
                    cell.len()
                };
                output.push_str(&format!("{:<width$}", cell, width = width));
                output.push_str(" | ");
            }
        }

        output.push_str(ansi::RESET);

        println!("{}", output);
        self.rows.clear();
    }
}

impl fmt::Display for TableLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for row in &self.rows {
            output.push_str("| ");
            for (i, cell) in row.iter().enumerate() {
                let width = if i < self.column_widths.len() {
                    self.column_widths[i]
                } else {
                    cell.len()
                };
                output.push_str(&format!("{:<width$}", cell, width = width));
                output.push_str(" | ");
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}
