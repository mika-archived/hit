# hit

Highlight keywords, that's all.

## How to use

Highlight keywords in terminal, from a single file

```bash
> hit Highlight ./README.md
```

Read from stdin

```bash
> curl -s https://sh.rustup.rs | hit RUSTUP
```

`hit` can use regular expressions for highlighting keywords

```bash
> curl -s https://sh.rustup.rs | hit "[Rr]ust"
```

`hit` can be combined with `tail -f` to continuously monitor a given file for highlight keywords

```bash
> tail -f /var/log/nginx/error.log | hit 401
```

and chaining with other tools

```bash
> tail -f /var/log/nginx/error.log | rg "GET /" --line-buffered | hit "HTTP/[^\s\"]"
```

If you prefer green to red for highlight, you can change the color.

```bash
> hit red --color red ./README.md
# or ANSI 256 colors
> hit red --color 196 ./README.md
# or HEX RGB
> hit red --color 255,00,00 ./README.md
```

`hit` supports multiple keywords.

```bash
# HTTP/1.1 and HTTP/2.0 is highlighted as green
> tail -f /var/log/nginx/access.log | hit -e "HTTP/1.1" -e "HTTP/2.0" -c green
# HTTP/1.1 is highlighted as red and HTTP/2.0 is highlighted as green
> tail -f /var/log/nginx/access.log | hit -e "HTTP/1.1" -c red -e "HTTP/2.0" -c green
```

## Installation

### From source

```
$ cargo install hit
```
