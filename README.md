# Zenroom-rs #

This is a small Rust wrapper around [Zenroom](https://dev.zenroom.org/).

## Usage ##

```rust
use zenroom_rs::runtime::*;
use serde_json::Value;

fn main() {
    let script = String::from(r#"
rule check version 1
Scenario 'simple':Create the keypair
Given that I am known as 'Alice'
When I create the keypair
Then print all data
"#);
    let rt = Runtime::new(String::new());
    let output = rt.execute(script, Value::Null, Value::Null).unwrap();

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
```

## Building ##

To build this project, you need to first grab the Zenroom source code and build it for
your target. Here we will clone the project in parent folder of this project:

    $ cd ..
    $ git clone https://github.com/fdeantoni/Zenroom 
    
Note that the above is a forked repo of [Zenroom](https://github.com/DECODEproject/Zenroom) which
contains an OSX static library target. If you are building for linux just use the Zenroom repo.

Once cloned, make the Zenroom library. Here we will create the OSX static library:

    $ cd Zenroom
    $ make osx-lib
    
With the static library created, go back to this project and build it:

    $ cd ../zenroom-rs
    $ cargo build        
    
Note that by default this project assumes the Zenroom repo is available in `../Zenroom`. If you used
a different folder you can create a `.env` file with the following:

    ZENROOM_DIR=/path/to/your/zenroom/repo
    
# License #

As Zenroom is licensed as AGPLv3, so is this project.        