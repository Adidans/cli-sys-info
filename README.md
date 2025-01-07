
 ____                           ______                 ___                      ____       __         ______
/\  _`\                        /\__  _\              /'___\                    /\  _`\    /\ \       /\__  _\
\ \,\L\_\    __  __      ____  \/_/\ \/      ___    /\ \__/   ___              \ \ \/\_\  \ \ \      \/_/\ \/
 \/_\__ \   /\ \/\ \    /',__\    \ \ \    /' _ `\  \ \ ,__\ / __`\             \ \ \/_/_  \ \ \  __    \ \ \
   /\ \L\ \ \ \ \_\ \  /\__, `\    \_\ \__ /\ \/\ \  \ \ \_//\ \L\ \             \ \ \L\ \  \ \ \L\ \    \_\ \__
   \ `\____\ \/`____ \ \/\____/    /\_____\\ \_\ \_\  \ \_\ \ \____/              \ \____/   \ \____/    /\_____\
    \/_____/  `/___/> \ \/___/     \/_____/ \/_/\/_/   \/_/  \/___/                \/___/     \/___/     \/_____/
                 /\___/
                 \/__/


This is a simple CLI tool to display system information using the `sysinfo` crate.

## Features

- Display system information
- Display CPU information
- Display memory information
- etc...

### Prerequisites

To use this tool, you need to have the following installed on your system:

- Rust (latest stable version)
- Cargo (comes with Rust)
- Git (for cloning the repository)

You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

### Installation

To install the CLI System Information tool, follow these steps:

1. Clone the repository:

   ```sh
   git clone https://github.com/Adidans/cli-sys-info.git
   ```

2. Navigate to the project directory:

   ```sh
   cd cli-sys-info
   ```

3. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

### Usage

To use the CLI System Information tool, run the following command:

```sh
./target/release/cli-sys-info
```

This will display the system, CPU, and memory information in your terminal.
