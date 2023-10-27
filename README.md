## About

`rip` is a WIP minimal BitTorrent client written in Rust.

## Installation

You can download an executable from the [releases](https://github.com/jonassterud/rip/releases) section here on GitHub.

## Contributing

Feel free to contribute!

For development, it's easiest to use the included [`.devcontainer`](https://containers.dev/).
If not, remember to set the Rust toolchain to "nightly" - e.g. by running `rustup override set nightly` in the project directory.

Use tools such as [Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) to improve your code:  
`cargo +nightly fmt`  
`cargo +nightly clippy`

Here are some useful resources:

* [The BitTorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
* [BitTorrentSpecification](https://wiki.theory.org/BitTorrentSpecification)

Commit messages should be structured like this: `<type>(<optional scope>): <description>`.

Where type is one of the following: `feat`, `fix`, `docs` or `refactor`.  
The optional scope can be one of the following: `lib`, `app`, `ci`.  
When mentioning files or folders in the description, type the name between the \`\` characters.

Example commit message: ``refactor(lib): move `torrent.rs` into `parse` directory``

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](./LICENSE) for details.
