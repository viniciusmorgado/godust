# Godust

Visit [Godust official website](https://godust.donatto.dev.br/) for more information and documentation.

Godust is an opinionated CLI tool that generates [Godot Engine](https://godotengine.org/) project templates in Rust, specifically using the godot-rust library (GDExtension bindings for Rust).

The [godot-rust](https://godot-rust.github.io/) library is a pure Rust implementation of the GDExtension bindings. This means you can write code in Rust that's equivalent to what you'd typically use GDScript or C# for in Godot.

A key advantage is that because your game project is fundamentally a Rust project, you can easily integrate other Rust tools and libraries. Beyond that, Rust's maintainability and ergonomics are a major benefits compared to GDScript, especially as projects grow.

Godust is opinionated about how templates are structured. The default layout favors a "vertical slice" approach where features like a splash screen or menu live in Rust modules with matching names, so the codebase stays easy to navigate for both new and experienced developers. Another opinionated choice is to expose any values that can be adjusted in the Godot editor instead of hard-coding them in Rust. While GDScript can be edited directly in the editor, Rust cannot, so exposing properties avoids unnecessary code changes and recompiles when you are tweaking simple 2D or 3D gameplay values during iteration.

You can still use GDScript and Rust together in the same project, and if you are using the .NET build of Godot, C# as well. A common approach is to prototype everything in GDScript, and re-write in Rust after you achieve your goal with that module. Another common approach is to prototype everything in GDScript, rewrite most parts in Rust, and use C# for components that benefits from a more robust reflection system.

## Templates

TODO: Os ECSs vão ser uma opção adicional via parametro.

- **minimal**: Godot-rust setup ready to use. No additional code or nodes, the most minimal template available. It's basically an automated version of the godot-rust setup [tutorial](https://godot-rust.github.io/book/intro/hello-world.html), all other templates are built on top of minimal.
- **default**: Built on top of minimal is a basic template that contains (or  will) sensible defaults like main menu, splash screen for engine and brand, and platform specific optimizations, right now only the splash screen is available.
- **ecs**: Built on top of minimal is a basic template that integrates, with examples, Bevy ECS for your Godot project.
- **open_world**: Complete swappable third and first person (Elder Scrolls-Like) character controller with camera, mouse look, WASD movement, interaction systems and Terrain3D extension for open world map creation.
- **open_world_ecs**: Complete swappable third and first person (Elder Scrolls-Like) character controller with camera, mouse look, WASD movement, interaction systems and Terrain3D extension for open world map creation build with Bevy ECS.
- **third_person**: Third-person character controller with camera, mouse look, WASD movement, and interaction systems.
- **third_person_ecs**: Third-person character controller with camera, mouse look, WASD movement, and interaction systems build with Bevy ECS.
- **first_person**: First-person character controller with mouse look, WASD movement, and basic mechanics.
- **first_person_ecs**: First-person character controller with mouse look, WASD movement, and basic mechanics build with Bevy ECS.
- **first_person_shooter**: Hitscan-based first-person shooter character controller with mouse look, WASD movement, and basic mechanics.
- **first_person_shooter_ecs**: Projectile-based first-person shooter character controller with mouse look, WASD movement, and basic mechanics build with Bevy ECS.
- **mobile**: Mobile template with correct platform optimization, screen resolution and other tweaks, for both Android and iOS.

## Templates Roadmap

| Template                 | Status |
|--------------------------|--------|
| **Minimal**              | ✅     |
| **Default**              | ✅     |
| **ECS**                  | ✅     |
| **First Person**         | 🚧     |
| **Third Person**         | 📋     |
| **Open World**           | 📋     |
| **Mobile (Android/iOS)** | 📋     |

---

**Status Legend:**
- ✅ **Complete** - Ready to use
- 🚧 **In Progress** - Currently being developed
- 📋 Planned - Scheduled for future development

## Pre-requisites

Before getting started, please ensure you have the following installed:

* The Rust package manager, Cargo (which comes with a standard Rust installation) - [To install Rust and Cargo click here.](https://www.rust-lang.org/tools/install)

## How to use it

To generate the template:

To use the default template you need to define the engine version, render method and the project name:

```
godust --name my_project_name --engine 4.6 --rendering-method "forward_plus"
```

To choose a template different from the default you should explicitly specify the template:

```
godust --name my_project_name --engine 4.6 --rendering-method "forward_plus" --template ecs
```

Or using the short form:

```
godust -n my_project_name -e 4.6 -r "forward_plus" -t ecs
```

Godust help:

```
Godust - CLI tool for Godot Engine template generation

Usage: godust --name <NAME> --engine <ENGINE> --rendering-method <RENDERING_METHOD> [OPTIONS]

Options:
  -n, --name <NAME>                          Project name
  -e, --engine <ENGINE>                      Engine version
  -r, --rendering-method <RENDERING_METHOD>  Render method (forward_plus, gl_compatibility, mobile)
  -t, --template <TEMPLATE>                  Template (available: minimal, ecs) [default: default]
  -h, --help                                 Print help
  -V, --version                              Print version
```

Everything works well if you see something like this:

```
✅ Successfully created project structure for 'my_project_name'...
🔨 Building template context...
📝 Generating project files from 'ecs' template...
🔨 Building Rust library (this may take a moment)...
✅ Rust library built successfully!

✅ Successfully generated 'my_project_name' from template 'ecs'!

📁 Project structure:
   my-project-name/
   ├── my-project-name/                (Godot project)
   └── my_project_name_core/           (Rust workspace)
       └── my_project_name_lib/        (Rust library — gameplay code)

💡 Next steps:
   cd my-project-name
   cd my_project_name_core && cargo build
   Open my-project-name in Godot Editor

📦 Add a gameplay dependency (no -p flag needed):
   cd my_project_name_core && cargo add <crate>
```

## Running Your Project

After generating your Godot-Rust project with Godust, the Rust library is automatically built during project creation. You can immediately open your project in the Godot editor:

**Open in Godot Engine**: The Godot project file (project.godot) is located in `my_project_name/my_project_name/project.godot`. Open the Godot editor and import this project.

**Note**: The initial Rust build (debug mode) is performed automatically when you create the project. If you make changes to the Rust code later, you'll need to rebuild:
- For development: `cd my_project_name_core && cargo build`
- For production/release: `cd my_project_name_core && cargo build --release`

**Adding gameplay dependencies**: `my_project_name_core` is a Cargo workspace containing a single member crate, `my_project_name_lib`, which holds your gameplay code. To add a dependency, run `cargo add` from the workspace root — Cargo automatically resolves the single workspace member, so there's no need for a `-p my_project_name_lib` flag:
```
cd my_project_name_core
cargo add rand
```
This adds the dependency to `my_project_name_lib/Cargo.toml`, and it becomes available to any Rust module under `my_project_name_lib/src/`.

Even though the template generation process customizes the name of the core project (the Rust part of the project), which is different from the default defined in the [godot-rust book HelloWorld tutorial](https://godot-rust.github.io/book/intro/hello-world.html), the overall structure still needs to respect the same layout.

## Contributing

Contributions are welcome. Please open an issue for bugs or feature ideas, then submit a pull request against `develop` from a work branch.

### PR naming and versioning

The release workflow derives the next version from commit messages on `develop`. To ensure your change bumps the correct version, use a squash merge with a PR title that matches one of these prefixes:

- `BREAKING CHANGE:` for a major bump
- `feat:` for a minor bump
- `fix:` (or any other prefix) for a patch bump

If you use multiple commits, keep the PR title aligned with the intended version bump since squash merges use the PR title as the commit message.

## License

This project is licensed under the [MIT License](https://github.com/viniciusmorgado/godust/blob/main/LICENSE).
