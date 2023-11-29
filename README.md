Easily calculate CIDR subnets

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
Usage: cidrc [OPTIONS] <IP_CIDR>

Arguments:
  <IP_CIDR>

Options:
  -o, --output <OUTPUT>  [default: text] [possible values: text, json, yaml]
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
❯ cidrc 10.0.0.1/24 -o json
{
  "ip": "10.0.0.1",
  "cidr": 24,
  "subnet_mask": "255.255.255.0",
  "wildcard_mask": "0.0.0.255",
  "first_host_addr": "10.0.0.1",
  "last_host_addr": "10.0.0.254",
  "usable_hosts": 254,
  "network_addr": "10.0.0.0",
  "broadcast_addr": "10.0.0.255",
  "total_hosts": 256
}
```
