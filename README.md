# credsdefault-cli
```
                                 █████             █████             ██████                       ████   █████   
                                ░░███             ░░███             ███░░███                     ░░███  ░░███    
  ██████  ████████   ██████   ███████   █████   ███████   ██████   ░███ ░░░   ██████   █████ ████ ░███  ███████  
 ███░░███░░███░░███ ███░░███ ███░░███  ███░░   ███░░███  ███░░███ ███████    ░░░░░███ ░░███ ░███  ░███ ░░░███░   
░███ ░░░  ░███ ░░░ ░███████ ░███ ░███ ░░█████ ░███ ░███ ░███████ ░░░███░      ███████  ░███ ░███  ░███   ░███    
░███  ███ ░███     ░███░░░  ░███ ░███  ░░░░███░███ ░███ ░███░░░    ░███      ███░░███  ░███ ░███  ░███   ░███ ███
░░██████  █████    ░░██████ ░░████████ ██████ ░░████████░░██████   █████    ░░████████ ░░████████ █████  ░░█████ 
 ░░░░░░  ░░░░░      ░░░░░░   ░░░░░░░░ ░░░░░░   ░░░░░░░░  ░░░░░░   ░░░░░      ░░░░░░░░   ░░░░░░░░ ░░░░░    ░░░░░
```

CLI client for factory-default credentials search that uses a comprehensive dataset. The dataset contains information on services, vendors, systems, OT and IoT devices, routers, and more.

## Features
- Caches dataset for offline retrieval
- Keyword search
- CSV report

## Usage
**Download/Update dataset**
```
./credsdefault-cli update
```

**Search dataset**
```
./credsdefault-cli search keyword keyword2 keyword3
```

**Export CSV report**
```
./credsdefault-cli search keyword keyword2 --csv report.csv
```

## Installation
Download latest release and run it. It's Rust.

```
iwr https://github.com/krystianbajno/credsdefault-cli/releases/download/release/credsdefault-cli-windows-x86_64 -outfile credsdefault-cli.exe
wget https://github.com/krystianbajno/credsdefault-cli/releases/download/release/credsdefault-cli-linux-x86_64
wget https://github.com/krystianbajno/credsdefault-cli/releases/download/release/credsdefault-cli-aarch64-apple-darwin
```

## Web Version
Check out the web version at [credsdefault-search](https://github.com/krystianbajno/credsdefault-search)

## Dataset
Check out the dataset at [credsdefault-dataset](https://github.com/krystianbajno/credsdefault-dataset)
