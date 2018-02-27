# Ru(n)st

Ru(n)st *(pronunced: runst)* is a little shell command you can use in the Unix shells *shebang* to compile and
run "rust scripts" directly.

It was inspired by a [Go experiment by Cloudflare](https://blog.cloudflare.com/using-go-as-a-scripting-language-in-linux/)

It is meant to be simple and fast. If you are looking for something more powerful, please use
[Cargo script](https://github.com/DanielKeep/cargo-script)


## How it works

Just clone the repo, compile it:

```
cargo build --release
```

Copy it somewhere in the executables path and make it executable:

```
sudo cp target/release/runst /usr/local/bin
sudo chmod 755 /usr/local/bin/runst
```

And place it in a shebang. This is [the example](examples/hello.rs):

```
#!/usr/bin/env runst

use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if !args.is_empty() {
        println!("Hello {}", args.join(" "))
    } else {
        println!("Hello world");
    }
}
``` 

Then run the script (you may need to make it executable first)

```
chmod 755 examples/hello.rs
examples/hello.rs
```

It will compile the script on the fly (in `/tmp`), execute it and delete the executable at the end.


## Warnings

This software is **HIGHLY EXPERIMENTAL**. And a quick hack put together on a lazy Sunday morning.

- You must have rust installed and executable from your shell executable path.
- You cannot use external crates (it's calling rustc directly. No cargo). Only `std` and `core`.
- It's only been tested on Linux (Ubuntu 17.10 to be precise).
- Yes, it's GPL. I want it to remain open source. But if you don't redistribute modified versions you can do nearly whatever you want.
- It could eat your hamster, your gopher, your python and most of your jewellery.
- Use at your own risk.


## Contributions

Contributions are welcome.

For small sensible improvements just open a PR. 

For bigger improvements, let's talk, so you don't waste your time and get angry at me if I reject it. :D

But if you want to ignore the last paragraph because you want to experiment, be my guest. ;)

If you want to improve the documentation, please do it!
