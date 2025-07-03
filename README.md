ATTENTION: This project is at the very beginning of its development. Please wait for updates.
---
# Godust

Godust is a CLI tool that generates [Godot Engine](https://godotengine.org/) project templates in Rust, specifically using the godot-rust library (GDExtension bindings for Rust).

The [godot-rust](https://godot-rust.github.io/) library is a pure Rust implementation of the GDExtension bindings. This means you can write code in Rust that's equivalent to what you'd typically use GDScript or C# for in Godot.

A key advantage is that because your game project is fundamentally a Rust project, you can easily integrate other Rust tools and libraries. For example, you can incorporate [Bevy Engine](https://bevy.org/) packages, or use [Vulkano](https://crates.io/crates/vulkano) and [Ash](https://crates.io/crates/ash) for Vulkan graphics code. We're even planning future templates that will include pre-built scripts for common features, like an inventory system using Bevy ECS, among others.

## Where to Find Help?

If you encounter problems or have suggestions for the CLI itself, including ideas for new templates, changes, or bug fixes, please [open an issue](https://github.com/viniciusmorgado/godust). Feel free to submit pull requests (PRs) for any changes or improvements you'd like to contribute.

However, if you need assistance with programming your game using Godot and Rust, here are some excellent resources:

- **godot-rust Official Book**: The [official book](https://godot-rust.github.io/book/index.html) is your primary guide for learning Godot with Rust.
- **godot-rust API Documentation**: For detailed information on the available functions and modules, consult the [API documentation](https://godot-rust.github.io/docs/).
- **godot-rust GitHub**: If you encounter bugs specifically related to godot-rust, you can check their [GitHub repository here](https://github.com/godot-rust/gdext).
- **godot-rust Demo Projects**: Remember that godot-rust also maintains its own repository of [demo projects](https://github.com/godot-rust/demo-projects). While our templates don't strictly adhere to these examples (we aim for more dynamic generation), you can still extract useful content and insights from them.
- **Community Communication Channels**: For further community engagement for godot-rust, you can find links to communication groups like Discord and Mastodon on the [godot-rust main website](https://godot-rust.github.io/).

- **Godot Engine Official Documentation**: You will likely also want to consult the [Godot Engine official documentation](https://docs.godotengine.org/en/latest/). It's arguably the best source for understanding the engine's core concepts and inner workings. If you're learning, adapting the GDScript and C# samples and tutorials to Rust can be an extremely helpful exercise.

**Disclaimer**: Please note that this CLI and its templates are an independent project and are not associated with godot-rust or its development team.

## Templates Roadmap

Here's what's coming for our project templates:

### Basic Templates
* **Blank**: Currently in progress.
* **Basic Hello World**: Planned.
* **Basic 3D Hello World**: Planned.
* **Basic 2D Hello World**: Planned.

### Full Templates
* **Third Person**: Planned.
* **Third Person + Inventory with Bevy ECS**: Planned.
* **First Person**: Planned.
* **Top Down**: Planned.
* **Vehicle**: Planned.
* **Vulkan Shaders with Vulkano Library**: Planned.

## Pre-requisites

Before getting started, please ensure you have the following installed:

* Godot Engine and its command-line interface (Godot CLI) - [To setup Godot CLI click here.]()
* The Rust package manager, Cargo (which comes with a standard Rust installation) - [To install Rust and Cargo click here.](https://www.rust-lang.org/tools/install)

## How to use it

```
Godust - CLI tool for Godot Engine template generation

Usage: godust --name <NAME> --engine <ENGINE> --rendering-method <RENDERING_METHOD>

Options:
  -n, --name <NAME>                          Define project name
  -e, --engine <ENGINE>                      Godot Engine version
  -r, --rendering-method <RENDERING_METHOD>  Rendering method (forward_plus or compatibility)
  -h, --help                                 Print help
  -V, --version                              Print version
```

## License

This project is licensed under the [MIT License](https://github.com/viniciusmorgado/godust/blob/main/LICENSE).
