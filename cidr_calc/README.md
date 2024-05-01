CLI to easily calculate CIDR subnets

# Install

### Using Cargo

If you have rust installed:
```
cargo install --locked cidr_calc
```

If not check it out!
https://www.rust-lang.org/tools/install

### Releases Binary

You can also download the release binary from GitHub releases.
https://github.com/pitoniak32/cidr_calc/releases

# Usage
```
CLI to easily calculate CIDR subnets

Usage: cidrc [OPTIONS] <IP_CIDR>

Arguments:
  <IP_CIDR>  Usage: X(.,-)X(.,-)X(.,-)X(/,-)X (ex: 10.0.0.1/24, 10-0-0-1-24)

Options:
  -o, --output <OUTPUT>  [default: text] [possible values: text, json]
  -h, --help             Print help
  -V, --version          Print version
```

# Examples
```
❯ cidrc 10.0.0.1/24
Network Summary
ip...............: 10.0.0.1
cidr.............: 24
subnet_mask......: 255.255.255.0
wildcard_mask....: 0.0.0.255
first_host_addr..: 10.0.0.1
last_host_addr...: 10.0.0.254
usable_hosts.....: 254
network_addr.....: 10.0.0.0
broadcast_addr...: 10.0.0.255
total_hosts......: 256
```

```
❯ cidrc 10.0.0.1/24 -o json | jq '.last_host_addr'
"10.0.0.254"
```

```
❯ echo "1.1.1.1/1" | xargs cidrc -o json
{
  "ip": "1.1.1.1",
  "cidr": 1,
  "subnet_mask": "128.0.0.0",
  "wildcard_mask": "127.255.255.255",
  "first_host_addr": "0.0.0.1",
  "last_host_addr": "127.255.255.254",
  "usable_hosts": 2147483646,
  "network_addr": "0.0.0.0",
  "broadcast_addr": "127.255.255.255",
  "total_hosts": 2147483648
}
```
