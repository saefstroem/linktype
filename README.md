# `linktype` - All PCAP Link Types for your rust needs

## Introduction

This crate/repository provides a list of all known PCAP link types as per the [tcpdump linktype list](https://www.tcpdump.org/linktypes.html).
It was created because I required a list of all link types for a project and could not find a rust crate that provided this information. Including this manually
just for a single project, seemed like a waste of time. Therefore, to avoid repeating this process, I decided to create this crate.

 An unofficial publicly managed linktype list as per https://www.tcpdump.org/linktypes.html.


## Usage

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
linktype = "0.1.0"
```

Then, you can use the crate as follows:

```rust
use linktype::LinkType;

fn main() {
    let link_type = LinkType::Ethernet;
    println!("Link type: {:?}", link_type);

    match link_type {
        LinkType::Ethernet => {
            // Adjust code execution according to the link type
        },
        _ => println!("Unknown link type"),
    }

    let some_link_value=1;
    let link_type=LinkType::from_u16(some_link_value);
    println!("Link type: {:?}", link_type);
}
```

## License
This crate is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.

## Contribution
If you would like to contribute to this crate, feel free to open a pull request or an issue. I am always open to suggestions and improvements.