# fogrn
`fogrn` is a simple terminal application for preventing your Mac's screen from sleeping. This is particularly useful when you are training machine learning models, keeping Microsoft Teams active while AFK, or performing other tasks requiring the screen to stay awake.

## Features
- Prevents screen sleep on macOS.
- Lightweight and easy to use via the terminal.
- Helpful for tasks like long-running processes, virtual meetings, or development workflows.

## Installation
To install fogrn on macOS, use Homebrew:
```bash
brew tap Bourbxn/fogrn
brew install fogrn
```

## Troubleshooting Installation
If you encounter the following error:
> Your Command Line Tools are too outdated
Follow these steps:
1. Remove the existing Command Line Tools:
```bash
sudo rm -rf /Library/Developer/CommandLineTools
```
2. Reinstall Command Line Tools:
```bash
sudo xcode-select --install
```
3. Wait for the installation to finish.
4. Update Homebrew:
```bash
brew update && brew upgrade
```

## Usage
The core principle of fogrn is to slightly move the mouse cursor periodically to prevent the screen from sleeping.

### Basic Usage
To prevent the screen from sleeping, simply run:
```bash
fogrn
```
The application will keep running until you stop it using Ctrl + C.

### Custom Delay
You can specify a delay (in seconds) between mouse movements using the --delay flag:
```bash
fogrn --delay 300
```
This sets a delay of 300 seconds (5 minutes) between each cursor movement.
### Other Commands
- **Show help information**  
    Use the `--help` flag to display available commands and options:  
    ```bash
    fogrn --help
    ```
- **Check the version**  
    Use the `--version` flag to check the current version of `fogrn`:  
    ```bash
    fogrn --version
    ```
## Contributions
Feel free to contribute to `fogrn`! If you encounter issues or have suggestions for improvements, open a pull request or submit an issue.


