# murder

A practical util for GNU/Linux or macOS to kill processes by PID, name, or port — politely at first, then less so.
It's the author best excuse to learn Rust from the original ruby script at https://codeberg.org/EvanHahn/dotfiles/src/branch/main/home/bin/bin/murder

## Usage

```bash
murder 123    # kill by pid
murder rust   # kill by process name (whole-word, case-insensitive)
murder :3000  # kill whatever is listening on TCP port 3000
```

Example:

```text
$ murder :3000
murder node server.js (pid 4242)? y
```

Answer `y` or `yes` to kill; anything else skips the process.

`murder` escalates through signals until the process is gone:

| Signal    | Wait before next |
| --------- | ---------------- |
| `15` TERM | 3s               |
| `2` INT   | 3s               |
| `1` HUP   | 4s               |
| `9` KILL  | —                |

It stops as soon as the process exits. When matching by name or port, it asks for confirmation before each kill.

## Installation

### With cargo (from git)

```bash
cargo install --git https://github.com/Lyokolux/murder
```

### From source

```bash
git clone https://github.com/Lyokolux/murder.git
cd murder
cargo install --path .
```

Ensure the `.cargo/.bin` is in the `$PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Uninstall

```bash
cargo uninstall murder
```
