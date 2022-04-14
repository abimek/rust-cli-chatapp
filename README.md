# rust-cli-chatapp

**A simple cli groupchat where messages are broadcasted to all connected users.**

This rust project was built for me to get a better understanding of the asynchronous programming aspect of rust development. I decided to use tokio as that was the first thing that came up from my initial searches on the godsend of programming websites, StackOverflow. I faced many challenges with tokio and learned a great deal from each one. I would later like to implement Arc and Mutex into this project or later asynchronous projects to better understand how those data types come into play in actual development.

### Development
Throughout the development process, I faced many challenges such as an improper implementation of reading and writing packets from ```tokio::net::TcpStream```. These challenges forced me to undergo multiple refractors of my code teaching me a great deal. My biggest challenge was easily fixed with some research which led me to ```tokio::select```. 

#### Binary Targets
Due to this project being a cli chatroom, I was in need of both a server and client, and would later even need to add a library. I hoped to keep the binaries inside the same cargo project and thanks to some quick googling, I found exactly what I was looking for. By creating a bin folder and placing binaries within it I was able to check and compile all of them simultaneously while having them in the same project.
```
├──Cargo.Toml
├──src
       └──bin
           ├── server
           │   └── main.rs
           └── client
                └── main.rs
```

##### Trait Functions that Return Self
For the packet system, I was in need of shared behavior where packets could be created from their string representations. I had decided to implement this in the form of a trait called Packets. This led me to one of the hardest challenges of this entire project, which was describing a type that would return self. After a couple of hours of scratching my head and despising rust, I was able to fix the issue by adding the Sized trait bound on Self.

```rust
fn from_string(data: &str) -> serde_json::Result<Self>
where
        Self: Sized;
```
I believe this was required due to rust return values needing to have a definite size.


