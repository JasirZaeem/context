# Context

## Installation

```bash
# Clone the repository
git clone https://github.com/JasirZaeem/context.git
# Build
cd context
cargo build --release
# Copy to bin
sudo cp target/release/context /usr/local/bin
```

## Usage

### Show context variables for current directory
```bash
context
# or
context print
```
### show a specific context variable
```bash
context <variable>
# or
context print <variable>
```

### Set a context variable
```bash
context add <variable> <value>
```

### Remove a context variable
```bash
context rm <variable>
```

### Show config file path
```bash
context config
```

### Chose a different config file
```bash
context --config <path> <command>
# use absolute path, set an alias for ease of use
```

### Chose a different pwd
```bash
context --pwd <path> <command>
# use absolute path
```
