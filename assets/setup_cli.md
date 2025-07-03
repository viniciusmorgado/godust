# Godot CLI Setup Tutorial

This tutorial will guide you through setting up Godot's command-line interface (CLI) on different operating systems and installation methods.

## Steam Installation (Linux)

If you installed Godot through Steam on Linux:

1. Open terminal and edit your shell configuration:
   ```bash
   nano ~/.bashrc
   ```

2. Add these lines at the end of the file:
   ```bash
   export PATH="$PATH:/var/home/$USER/.local/share/Steam/steamapps/common/Godot Engine"
   alias godot=godot.x11.opt.tools.64
   ```

3. Save and exit (Ctrl+X, then Y, then Enter)

4. Reload your shell configuration:
   ```bash
   source ~/.bashrc
   ```

5. Test the installation:
   ```bash
   godot --version
   ```

## macOS

### Method 1: Official Download

1. Download Godot from the official website
2. Move the Godot app to your Applications folder
3. Open Terminal and edit your shell profile:
   ```bash
   nano ~/.zshrc
   ```

4. Add this line:
   ```bash
   export PATH="$PATH:/Applications/Godot.app/Contents/MacOS"
   alias godot="Godot"
   ```

5. Save, exit, and reload:
   ```bash
   source ~/.zshrc
   ```

### Method 2: Using Homebrew

1. Install Homebrew if you haven't already:
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

2. Install Godot:
   ```bash
   brew install godot
   ```

3. Godot should now be available in your PATH automatically

## Generic Linux (Non-Steam)

### Method 1: Downloaded Binary

1. Download Godot from the official website
2. Extract the archive and move it to a suitable location:
   ```bash
   sudo mkdir -p /opt/godot
   sudo mv Godot_v*.x11.64 /opt/godot/godot
   sudo chmod +x /opt/godot/godot
   ```

3. Edit your shell configuration:
   ```bash
   nano ~/.bashrc
   ```

4. Add these lines:
   ```bash
   export PATH="$PATH:/opt/godot"
   alias godot="/opt/godot/godot"
   ```

5. Reload your configuration:
   ```bash
   source ~/.bashrc
   ```

### Method 2: Using Package Manager

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install godot3
```

**Fedora:**
```bash
sudo dnf install godot
```

**Arch Linux:**
```bash
sudo pacman -S godot
```

**Flatpak (Universal):**
```bash
flatpak install flathub org.godotengine.Godot
```

For Flatpak, you can create an alias:
```bash
echo 'alias godot="flatpak run org.godotengine.Godot"' >> ~/.bashrc
source ~/.bashrc
```

## Windows

### Method 1: Manual Setup

1. Download Godot from the official website
2. Extract the archive to a permanent location (e.g., `C:\Tools\Godot\`)
3. Add Godot to your PATH:
   - Press `Win + X` and select "System"
   - Click "Advanced system settings"
   - Click "Environment Variables"
   - Under "System variables", select "Path" and click "Edit"
   - Click "New" and add the path to your Godot folder (e.g., `C:\Tools\Godot\`)
   - Click "OK" to save

4. Open Command Prompt or PowerShell and test:
   ```cmd
   godot --version
   ```

### Method 2: Using Chocolatey

1. Install Chocolatey if you haven't already (run as Administrator):
   ```powershell
   Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
   ```

2. Install Godot:
   ```cmd
   choco install godot
   ```

### Method 3: Using Scoop

1. Install Scoop if you haven't already:
   ```powershell
   irm get.scoop.sh | iex
   ```

2. Install Godot:
   ```cmd
   scoop install godot
   ```

## Verification

After setting up Godot CLI on any system, you can verify it's working by running:

```bash
godot --version
godot --help
```

## Common CLI Commands

Once set up, you can use these common commands:

- `godot --version` - Display version information
- `godot --help` - Show help menu
- `godot project.godot` - Open a specific project
- `godot --headless` - Run without GUI
- `godot --export "preset_name" output_path` - Export project
- `godot --script script.gd` - Run a script

## Troubleshooting

### Command Not Found
- Verify the PATH is correctly set
- Restart your terminal/command prompt
- Check if the Godot executable exists in the specified location

### Permission Denied (Linux/macOS)
- Ensure the Godot binary has execute permissions:
  ```bash
  chmod +x /path/to/godot
  ```

### Wrong Executable (Linux)
- Make sure you're using the correct binary name (e.g., `godot.x11.opt.tools.64` for older versions)

### Steam Deck / SteamOS
- The path might be different on Steam Deck:
  ```bash
  export PATH="$PATH:/home/deck/.local/share/Steam/steamapps/common/Godot Engine"
  ```

## Notes

- Some Linux distributions might have different executable names (e.g., `godot3`, `godot4`)
- On macOS, you might need to allow the app to run in System Preferences > Security & Privacy
- For development, consider using the "headless" version of Godot for server/CI environments
- Always verify the exact path to your Godot installation as it may vary based on your system and installation method
