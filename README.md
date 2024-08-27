# credsdefault-cli
CLI client for factory-default credentials search that uses a comprehensive dataset. The dataset contains information on services, vendors, systems, OT and IoT devices, routers, and more.

## Features
- Caches dataset for offline retrieval
- Keyword search
- CSV report

## Usage
**Update dataset**
```
./credsdefault-cli --update
```

**Search dataset**
```
./credsdefault-cli keyword keyword2 keyword3
```

**Export CSV report**
```
./credsdefault-cli keyword keyword2 --csv report.csv
```

## Installation
Download latest release and run it. It's Rust.

## Web Version
Check out the WEB version at [credsdefault-search](https://github.com/krystianbajno/credsdefault-search)
