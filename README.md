# Beaxplorer

Beaxplorer is an Ethereum Beacon chain explorer written in Rust and
relying on [Lighthouse](https://lighthouse.sigmaprime.io/).

It's split into two distinct components: An indexer and a web UI.
But there is a twist: Unlike other Beacon chain explorers, the web UI is
client side only, there is no backend, no SQL database. Instead,
the indexer serialize the data it extract from the Beacon chain into
[MessagePack](https://msgpack.org/) files that served on a static 
website and deserialized directly on the browser using 
[Rust's WebAssembly](https://rustwasm.github.io/wasm-pack/) capabilities.
