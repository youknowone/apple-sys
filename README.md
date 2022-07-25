# apple-sys

Apple platforms have a rather monotonous programming environment compared to other platforms. On several development machines, we will dependably obtain the same [bindgen](https://github.com/rust-lang/rust-bindgen) result. Then why not simply having bindgen configurations for the frameworks?
# How to use?

To add CoreFoundation and IOKit, try to run:
```sh
$ cargo add apple-sys --features=CoreFoundation,IOKit
```

If you are using older version of cargo, try to add to Cargo.toml:
```toml
apple-sys = { version = "*", features=["CoreFoundation", "IOKit"] }
```

The names of the features and the frameworks are same.
To see the full list, check the project’s features in the Cargo.toml file.
 
The feature names are the module names. Everything is single-depth. There are no nested modules.

```rust
use apple_sys::{CoreFoundation, IOKit};

// CoreFoundation::<any name>
// IOKit::<any name>
```

# apple-bindgen

The bindgen tool is installable and generating the same result to apple-sys crates.
To create a new `-sys` project, starting with `apple-bindgen` result will be a convenient way.

Install:
```
$ cargo install apple-bindgen
```

To generate CoreFoundation bindings,
```
$ apple-bindgen CoreFoundation --sdk macosx
```

To generate UIKit bindings for iOS,
```
$ apple-bindgen UIKit --sdk iphoneos
```

# Why apple-sys?

`apple-sys` contains auto-generated bindgen modules for Apple platforms. As long as we use the same versions of SDKs and bindgen, the result will be reproducible.

# Why not apple-sys?
Continually using the same SDKs doesn't sound realistic. I agree. Don’t trust apple-sys. Use the managed versions as best you can. For `CoreFoundation`, for instance, use [core-foundation-sys](https://github.com/servo/core-foundation-rs).

Then why do I use apple-sys? I created apple-sys for minor and unmanaged frameworks. apple-sys will be the last fallback.

# Contributing

There are no plans for apple-sys to distribute generated or manually changed code. We shall just manage bindgen rules.
Look in the project root directory's "bindgen/Bindgen.toml" file.
