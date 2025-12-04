# ansie-rs
print given ANSI escape code with ANSI escape sequance  

## Usage
```
$ ansie <ANSI ESCAPE CODE 1> <ANSI ESCAPE CODE 2> ...
```
This prints `\x1b<ANSI ESCAPE CODE 1>m\x1b<ANSI ESCAPE CODE 2>m...`

## Where do you need this
I thought "For example, Command Prompt", but we can do as `echo ^[[0m` (Ctrl+[ will be shown as ^[)...
