# 34 Streaming Word Count

## Goal

Practice processing a file line by line with `BufReader` instead of loading the whole file.

## Project

Create project:

```bash
cargo new practice/advanced-extra/34_streaming_word_count --name streaming_word_count
```

## Requirements

- Define `fn count_words_in_file(path: &std::path::Path) -> std::io::Result<usize>`.
- Open the file with `std::fs::File::open`.
- Wrap it in `std::io::BufReader`.
- Use `BufRead::lines()` to process the file line by line.
- Count whitespace-separated words across all lines.

## Constraints

- Do not use `std::fs::read_to_string`.
- Use `?` for IO errors.
- Do not use `unwrap` or `expect`.
- Accept the input file path from CLI arguments.

## Review Focus

- Streaming IO instead of whole-file reading.
- Correct propagation of line-reading errors.
- Minimal allocations while counting words.
