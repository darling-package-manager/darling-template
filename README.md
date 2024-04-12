# darling-template

This is a template for implementing [darling](https://github.com/darling-package-manager/darling) for package managers that don't yet have an implementation. The Darling specification can be sort of nitpicky (most notably that you need to have a `pub static PACKAGE_MANAGER` variable that `impl darling_api::PackageManager` accessible at your crate root), so this template serves to set things up for you. To use it, you can do so with or without automatic git setup:

## With git setup

On this repository, click "Use this template > Create new repository". This will create a new git repository using this template. You can then `git clone` it onto your local machine and work with it as normal.

## Without git setup

To use this template without git setup, you can simply clone it like normal and work onto your local machine.

## Next Steps

Whichever way you go, you must upload your crate to [crates.io](https://crates.io) with `cargo publish`. **Be sure to change the metadata in your `Cargo.toml` file to accurately reflect you as the author, your preferred license, etc.** 

**Also, your crate must start with `darling-`.**

Once your crate is published, **anyone with darling, with no update required, can use it.**

This should go without saying, but please be mindful of what you put in your code. Try to keep things performant and safe. 