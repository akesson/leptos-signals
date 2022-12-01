[![crates.io](https://img.shields.io/crates/v/leptos-signals)](https://crates.io/crates/leptos-signals)
[![Discord](https://img.shields.io/discord/1031524867910148188?color=%237289DA&label=discord)](https://discord.gg/YdRAhS7eQB)

# leptos-signals

Different Signals for [Leptos](https://crates.io/crates/leptos):

[<img src="https://raw.githubusercontent.com/gbj/leptos/main/docs/logos/logo.svg" alt="Leptos Logo" style="width: 40%; height: auto; display: block; margin: auto;">](http://https://crates.io/crates/leptos)

<br/>

This is the start of what will hopefully become a collection of signals. PRs are welcome!

## KeyedSignal

A **KeyedSignal** associates a key with a value. This is typically used for caching, where a key is needed for searching in the cache.

See [example](examples/keyed-signal/src/app/mod.rs).
