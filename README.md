## Latest Update

- Replace minimal blank template as default option by the new "default" template. Blank remains available but need to be explicitly invoke.
- Upgrade recommended version to Godot 4.6, and minimal to 4.2
- Add new version of Default template, build on top of blank with new sensible defaults.

# Godust

Godust is opinionated CLI tool that generates [Godot Engine](https://godotengine.org/) project templates in Rust, specifically using the godot-rust library (GDExtension bindings for Rust).

The [godot-rust](https://godot-rust.github.io/) library is a pure Rust implementation of the GDExtension bindings. This means you can write code in Rust that's equivalent to what you'd typically use GDScript or C# for in Godot.

A key advantage is that because your game project is fundamentally a Rust project, you can easily integrate other Rust tools and libraries. Beyond that, Rust's maintainability and ergonomics are a major benefit compared to GDScript, especially as projects grow.

Godust is opinionated about how templates are structured. The default layout favors a "vertical slice" approach where features like a splash screen or menu live in Rust modules with matching names, so the codebase stays easy to navigate for both new and experienced developers. Another opinionated choice is to expose any values that can be adjusted in the Godot editor instead of hard-coding them in Rust. While GDScript can be edited directly in the editor, Rust cannot, so exposing properties avoids unnecessary code changes and recompiles when you are tweaking simple 2D or 3D gameplay values during iteration.

You can still use GDScript and Rust together in the same project, and if you are using the .NET build of Godot, C# as well. A common approach is to prototype everything in GDScript, and re-write in Rust after you achive your goal with that module. Another common approach is to prototype everything in GDScript, rewrite most parts in Rust, and use C# for components that benefit from a more robust reflection system.

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

## Templates

| Template | Status | Description |
|----------|--------|-------------|
| **Blank** | ✅ | Godot-rust and Godot setup ready to use. No additional code or nodes, most minimal template. |
| **Default** | ✅ | Build on top of the blank template, adds sensible defaults such as a splash screen fro brand and engine, platform‑specific optimizations, and more. Default template when no template is selected. |
| **Blank + Bevy ECS** | ✅ | Blank template integrated with Bevy's Entity Component System. |
| **Third Person** | 🚧 | Complete third-person character controller with camera, movement, and interaction systems. |
| **First Person** | 🚧 | Complete first-person character controller with mouse look, WASD movement, and basic mechanics. |

---

**Status Legend:**
- ✅ **Complete** - Ready to use
- 🚧 **In Progress** - Currently being developed

## Pre-requisites

Before getting started, please ensure you have the following installed:

* The Rust package manager, Cargo (which comes with a standard Rust installation) - [To install Rust and Cargo click here.](https://www.rust-lang.org/tools/install)

## How to use it

To generate the template:

To use the default template you need to define the engine version, render method and the project name:

```
godust --name my_project_name --engine 4.6 --rendering-method "forward_plus"
```

You can also explicitly specify the template (optional, since it defaults to "default" when not provided a template):

```
godust --name my_project_name --engine 4.6 --rendering-method "forward_plus" --template blank
```

Or using the short form:

```
godust -n my_project_name -e 4.6 -r "forward_plus" -t blank
```

Godust help:

```
Godust - CLI tool for Godot Engine template generation

Usage: godust --name <NAME> --engine <ENGINE> --rendering-method <RENDERING_METHOD> [OPTIONS]

Options:
  -n, --name <NAME>                          Project name
  -e, --engine <ENGINE>                      Engine version
  -r, --rendering-method <RENDERING_METHOD>  Render method (forward_plus, gl_compatibility, mobile)
  -t, --template <TEMPLATE>                  Template (available: blank, blank_ecs) [default: blank]
  -h, --help                                 Print help
  -V, --version                              Print version
```

Everything works well if you see something like this:

```
✅ Successfully created project structure for 'my_project_name'...
🔨 Building template context...
📝 Generating project files from 'blank_ecs' template...
🔨 Building Rust library (this may take a moment)...
✅ Rust library built successfully!

✅ Successfully generated 'my_project_name' from template 'blank_ecs'!

📁 Project structure:
   my-project-name/
   ├── my-project-name/          (Godot project)
   └── my_project_name_core/     (Rust library)

💡 Next steps:
   cd my-project-name
   cd my_project_name_core && cargo build
   Open my-project-name in Godot Editor
```

## Running Your Project

After generating your Godot-Rust project with Godust, the Rust library is automatically built during project creation. You can immediately open your project in the Godot editor:

**Open in Godot Engine**: The Godot project file (project.godot) is located in `my_project_name/my_project_name/project.godot`. Open the Godot editor and import this project.

**Note**: The initial Rust build (debug mode) is performed automatically when you create the project. If you make changes to the Rust code later, you'll need to rebuild:
- For development: `cd my_project_name_core && cargo build`
- For production/release: `cd my_project_name_core && cargo build --release`

Even though the template generation process customizes the name of the core project (the Rust part of the project), which is different from the default defined in the [godot-rust book HelloWorld tutorial](https://godot-rust.github.io/book/intro/hello-world.html), the overall structure still needs to respect the same layout.

## Contributing

Contributions are welcome. Please open an issue for bugs or feature ideas, then submit a pull request against `main`.

### PR naming and versioning

The release workflow derives the next version from commit messages on `main`. To ensure your change bumps the correct version, use a squash merge with a PR title that matches one of these prefixes:

- `BREAKING CHANGE:` for a major bump
- `feat:` for a minor bump
- `fix:` (or any other prefix) for a patch bump

If you use multiple commits, keep the PR title aligned with the intended version bump since squash merges use the PR title as the commit message.

## License

This project is licensed under the [MIT License](https://github.com/viniciusmorgado/godust/blob/main/LICENSE).
