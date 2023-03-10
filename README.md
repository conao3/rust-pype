# rust-pype

Python one-liner helper written in Rust.

## Install

```bash
cargo install pype
```

## Usage

stdin is opened as `f`.

Prepare `sample1` file.
```bash
cat > sample1
1
2
3
```

### -e option

Execute specified python code.

```bash
$ cat sample1 | pype -e 'print(f.read())' | python
1
2
3
```

```bash
$ cat sample1 | pype -e 'print(f.read().splitlines())' | python
['1', '2', '3']
```

### -n option

Execute specified python code per line.  You can access via `line` variable to each line.

```bash
$ cat sample1 | pype -ne 'print("- " + line)' | python
- 1
- 2
- 3
```

```bash
$ cat sample1 | pype -ne 'print("- " + line + "$")' | python
- 1
$- 2
$- 3
```

### -l option

Available when `-n` is specified.  Removes trailing newlines from the input and adds newlines to `print`.

```bash
$ cat sample1 | pype -nle 'print("- " + line)' | python
- 1
- 2
- 3
```

```bash
$ cat sample1 | pype -nle 'print("- " + line + "$")' | python
- 1$
- 2$
- 3$
```

### -m option

Import specified module before executing python code.

```bash
cat sample1 | pype -m datetime -nle 'print(f"- {line}:", (datetime.date.today() + datetime.timedelta(days=int(line))))' | python
- 1: 2023-03-09
- 2: 2023-03-10
- 3: 2023-03-11
```

You can use any Python module such as `beautifulsoup4`.

```bash
$ curl -L dev.to | pype -m bs4 -le 'for a in bs4.BeautifulSoup(f.read(), "html.parser").find_all("h2", class_="crayons-story__title"):print(a.text.strip())' | python
Top 7 Featured DEV Posts from the Past Week
Six Years on DEV, Already?
Open Source Maintenance is Community Organizing
The Awesome Side of GitHub - Awesome Lists
Codility === Sadness
Meme Monday 😝
```

## License
Apache License 2.0
