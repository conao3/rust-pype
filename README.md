# pype

A Python one-liner helper written in Rust. Inspired by Perl's command-line options, pype makes it easy to write quick Python scripts for text processing directly from the command line.

## Installation

```bash
cargo install pype
```

## Overview

pype generates Python code from concise command-line arguments and pipes it to the Python interpreter. Standard input is available as the variable `f` (a file object).

## Usage

### Basic Example

Create a sample file for the examples below:

```bash
cat > sample.txt << 'EOF'
1
2
3
EOF
```

### Execute Python Code (`-e`)

Run arbitrary Python code with access to stdin via the `f` variable:

```bash
cat sample.txt | pype -e 'print(f.read())' | python
# Output:
# 1
# 2
# 3
```

```bash
cat sample.txt | pype -e 'print(f.read().splitlines())' | python
# Output: ['1', '2', '3']
```

### Line-by-Line Processing (`-n`)

Process each line individually. The current line is available as the `line` variable:

```bash
cat sample.txt | pype -ne 'print("- " + line)' | python
# Output:
# - 1
# - 2
# - 3
```

### Auto-Chomp Mode (`-l`)

Use with `-n` to automatically strip trailing newlines from input lines. This makes line processing cleaner:

```bash
cat sample.txt | pype -nle 'print("- " + line + " [end]")' | python
# Output:
# - 1 [end]
# - 2 [end]
# - 3 [end]
```

Without `-l`, the trailing newline would remain:

```bash
cat sample.txt | pype -ne 'print("- " + line + " [end]")' | python
# Output:
# - 1
#  [end]- 2
#  [end]- 3
#  [end]
```

### Import Modules (`-m`)

Import Python modules for use in your code:

```bash
cat sample.txt | pype -m datetime -nle 'print(f"Day {line}: {datetime.date.today() + datetime.timedelta(days=int(line))}")' | python
# Output:
# Day 1: 2023-03-09
# Day 2: 2023-03-10
# Day 3: 2023-03-11
```

Works with any installed Python package. Here's an example using BeautifulSoup to scrape headlines:

```bash
curl -sL dev.to | pype -m bs4 -le 'soup = bs4.BeautifulSoup(f.read(), "html.parser"); [print(h.text.strip()) for h in soup.find_all("h2", class_="crayons-story__title")]' | python
```

## Options Summary

| Option | Description |
|--------|-------------|
| `-e <code>` | Execute the given Python code |
| `-n` | Process input line by line (exposes `line` variable) |
| `-l` | Strip trailing newlines from each line (use with `-n`) |
| `-m <module>` | Import a Python module before execution |

## License

Apache License 2.0
