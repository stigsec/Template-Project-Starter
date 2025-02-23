# Template-Project-Starter
**tps** (template-project-starter) is a lightweight, fast Rust-based project starter that scaffolds new projects using customizable JSON templates. With **TPS**, you can quickly set up a project structure for multiple programming languages by defining your own templates.

## Features
- **Custom Templates:** Define JSON templates for your projects. Each template specifies the folder structure and starter files.
- **Cross-Platform:** Automatically uses platform-appropriate defaults for storing templates.
- **Placeholder Replacement:** Automatically replaces placeholders like {{project_name}} in file contents.
- **Multi-Language Support:** Example templates for Rust, Python, Node.js, Java, Go, C, TypeScript, and JavaScript.

## Dependencies
This project uses the following Rust crates:
- **clap:** For parsing command-line arguments
- **serde** and **serde_json:** For JSOn serialization and deserialization
- **dirs:** For safe static initialization of global variables

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/stigsec/Template-Project-Starter.git
   cd Template-Project-Starter
   ```
2. **Build the Project:**

   ```bash
   cargo build --release
   ```
   The compiled binary will be located in ```target/release/tps```

## Usage
**tps** takes two arguments: the template file and the project name.
```bash
tps rust.json rustProject
```

## Setting the Template Directory

**tps** searcher for templates in the directory specified by the ```TPS_PATH``` environment variable. If ```TPS_PATH``` is not set, **tps** falls back to a default directory:
- **On Windows:** Uses the local data directory.
- **On Linux/macOS:** Uses the configuration directory.

### Example (Linux/macOS):
Add the following line to your shell configuration file (e.g., ```~/.bashrc``` or ```~/.zshrc```):
```bash
export TPS_path="$HOME/.config/projectTemplates"
```
After updating your shell configuration file, reload it with:
```bash
source ~/.bashrc
```
### Example (Windows PowerShell):
```ps
$env:TPS_PATH = "C:\\Path\\To\\projectTemplates"
```

## Creating your Own Templates
Templates are defined as JSON files. Below is an example for a Rust project:
```json
{
  "name": "rust-basic",
  "folders": ["src", "tests"],
  "files": {
    "Cargo.toml": "[package]\nname = \"{{project_name}}\"\nversion = \"0.1.0\"\nedition = \"2021\"",
    "src/main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"
  }
}
```
Place your custom templates in the directory specified by ```TPS_PATH``` (or the default directory)

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE file](LICENSE) for more details.



---

Developed by [stigsec](https://github.com/stigsec).
