# Flamegraph 

## Setup

```sh
cargo install flamegraph
```

```toml
# Should only be used for running flamegraph
[profile.release]
debug = true
```

## First attempt

Bad, hard to reason about

# Tracy

I fucked the flamegraph one because I didn't have tracing feature, maybe?

Can I do tracing as a feature I pass, rather than touching the cargo.toml?

Woah it's so cool that this just works

Oh my god this is incredible, this is like exactly what I need to get to the bottom of this wow

Yeah reducing the number of meshes is really important, it spends a ton of time on those

Onscreen calc is fairly expensive, should make it happen less often and maybe be cheaper? Not as an exclusive system?

Okay yeah just install tracy then run `cargo run --release --features bevy/trace_tracy` and prosper holy shit.

# On the plane work

Lazily had a system that was always inserting/removing whatever it needed to, taking ~700us a frame. Not terrible, but given how simple the system is it should be super easy.
