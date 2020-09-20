# wanderball

![](/wanderball.png)

## Dependencies

- [amethyst](https://github.com/amethyst/amethyst)
  - MacOs / Linux: The same requirements documented in the [amethyst-starter-2d](https://github.com/amethyst/amethyst-starter-2d#for-mac-users), but wanderball defaults to `metal` rather than `vulkan` (you can change that in [Cargo.toml](/Cargo.toml))

## Running

### metal (probably metal if on macos)

`cargo run`

### vulkan (probably vulkan if not on macos)

1. Change `features = ["metal"]` in [Cargo.toml](/Cargo.toml) to `features = ["vulkan"]`
1. `cargo run`

## Playing

← ↑ ↓ →
