ATTENTION: This project is at the very beginning of its development. Please wait for updates.
---
# Godust

Godust is a CLI tool that generates [Godot Engine](https://godotengine.org/) project templates in Rust, specifically using the godot-rust library (GDExtension bindings for Rust).

The [godot-rust](https://godot-rust.github.io/) library is a pure Rust implementation of the GDExtension bindings. This means you can write code in Rust that's equivalent to what you'd typically use GDScript or C# for in Godot.

A key advantage is that because your game project is fundamentally a Rust project, you can easily integrate other Rust tools and libraries. For example, you can incorporate Bevy Engine packages, or use Vulkano and Ash for Vulkan graphics code. We're even planning future templates that will include pre-built scripts for common features, like an inventory system using Bevy ECS, among others.

## Templates Roadmap

Here's what's coming for our project templates:

* **Basic Hello World**: Currently in progress.
* **Third-Person Action Game**: Planned.
* **Third-Person Action Game + Inventory with Bevy ECS**: Planned.
* **First-Person Shooter + Inventory with Bevy ECS**: Planned.
* **Vulkan Shaders with Vulkano Library**: Planned.

## Pre-requisites

Before getting started, please ensure you have the following installed:

* Godot Engine and its command-line interface (Godot CLI)
* The Rust package manager, Cargo (which comes with a standard Rust installation)
