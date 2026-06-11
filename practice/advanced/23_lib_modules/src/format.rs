pub fn format_summary(summary: &crate::stats::Summary) -> String {
    format!("lines: {}, words: {}", summary.lines, summary.words)
}
