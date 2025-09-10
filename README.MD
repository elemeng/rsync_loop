# rsync-loop

A simple Rust tool to automate repeated rsync transfers with SSH password authentication, featuring scheduled intervals and progress tracking.

## What is this?

rsync-loop is a command-line utility that automates repeated rsync transfers with SSH password authentication (via sshpass) at configurable intervals. It’s designed to keep files in sync *continuously* rather than waiting for a one-time transfer, making it ideal for scenarios where:

- New files are generated incrementally (e.g., cryo-EM data collection, log file generation, or real-time sensor data) – sync files as they appear, finishing transfers by the time all data generation completes, saving critical time.
- Periodic backups to remote servers need consistent scheduling without manual intervention.
- Development workflows require frequent syncing between local and remote environments (e.g., updating web assets or application code).
- You need to monitor transfer reliability over time (with built-in success/failure tracking).
- Repetitive file transfer tasks (like syncing to multiple destinations) need automation.

By running rsync repeatedly at set intervals, it ensures your files stay up-to-date across locations without waiting for a full dataset to be generated.

## Features

✅ Runs rsync multiple times with a single command  
✅ Configurable wait time between sync rounds  
✅ Tracks success/failure of each transfer  
✅ Clean progress display with countdown timer  
✅ Graceful exit on Ctrl+C  
✅ can accept all rsync parameters through passthrough arguments  

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for building from source)
- `sshpass` (required for password-based SSH authentication)
- `rsync` (obviously, since we're repeating it)

Install from crates.io (**recommand**):

```bash
cargo install rsync_loop
```

Install dependencies on Debian/Ubuntu:
```bash
sudo apt-get install sshpass
```

Install dependencies on RHEL/Centos/Rocky/Almal Linux:
```bash
sudo yum install sshpass
```

On macOS (using Homebrew):
```bash
brew install sshpass
```

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/elemeng/rsync_loop.git
cd rsync_loop

# Build the binary
cargo build --release

# Install to your system (optional)
sudo cp target/release/rsync_loop /usr/local/bin/
```

## Usage

Basic syntax:
```bash
rsync_loop -p <ssh-password> [OPTIONS] -- <rsync-arguments>
```

### Options

| Option   | Short | Long         | Description                    | Default         |
| -------- | ----- | ------------ | ------------------------------ | --------------- |
| Password | `-p`  | `--password` | SSH password (required)        | -               |
| Times    | `-t`  | `--times`    | Number of sync rounds          | 20              |
| Wait     | `-w`  | `--wait`     | Seconds to wait between rounds | 300 (5 minutes) |

### Examples

1. Basic remote backup, running 10 times with 10-minute intervals:
```bash
rsync_loop -p "mypass" -t 10 -w 600 -- -avz /local/files/ user@remote:/backup/location/
```

2. Sync a website directory every 30 minutes, 5 times total:
```bash
rsync_loop -p "serverpass" -t 5 -w 1800 -- -avz --delete ./public/ webuser@example.com:/var/www/html/
```

3. Test connection reliability with minimal transfers:
```bash
rsync_loop -p "testpass" -t 100 -w 60 -- -av --dry-run user@host:/remote/path/ /tmp/
```

## Output Example

```
===== Round 1/5 =====
sending incremental file list
file1.txt
file2.txt

sent 123 bytes  received 45 bytes  112.00 bytes/sec
total size is 7890  speedup is 53.99
Round 1 succeeded.
Next sync in 04:59
```

## Notes

- **Security Warning**: Using passwords on the command line may expose them in process lists. Consider using SSH keys for better security when possible.
- All rsync arguments work as normal - this tool simply wraps rsync with scheduling functionality
- The countdown timer shows remaining time in MM:SS format
- Ctrl+C will exit gracefully and show final statistics

## License

[MIT](LICENSE)
