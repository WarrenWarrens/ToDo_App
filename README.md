# ToDo App
### Made entirely with Rust!

## Overview:

## What it does: 


#### This application is a strictly native, zero-telemetry desktop task manager. It provides a frictionless interface to quickly add tasks, mark them as completed with a strikethrough toggle, delete individual items, or batch-clear all completed tasks. Designed to be a completely localized tool, it automatically intercepts operating system close commands to securely serialize and save the user's current task state, seamlessly reloading it upon the next launch.


## How it works:

#### Unlike traditional retained-mode desktop apps (like Electron or Tauri) that pass messages between a backend and a DOM, this application uses an Immediate Mode GUI (IMGUI) architecture via the `egui` and `eframe` crates.

* State Management: The entire application state is held in a single, continuously mutating Rust struct.

* Rendering Loop: The UI is completely discarded and redrawn at the refresh rate of the monitor. Buttons, inputs, and layout declarations are evaluated alongside their event handlers in a single linear pass.

* Data Persistence: State serialization is handled by `serde` and `serde_json`. To ensure standard Linux compliance, the `directories` crate dynamically locates the correct XDG Base Directory (e.g., `~/.local/share/TodoApp/`) for secure, non-intrusive file I/O.

* Native OS Integration: The application's icon is baked directly into the executable at compile time using Rust's `include_bytes!` macro, ensuring no missing assets. It is then registered to the Linux window manager via a `.desktop entry`.


## What I learned:

#### Building this project served as a deep dive into memory-safe UI development and systems-level Linux integration. Key takeaways include:

* Borrow Checker Mechanics in UI Loops: I learned how to structure UI interactions around Rust's strict mutability rules. For example, deferring the deletion of a vector element until after the UI rendering loop completes to prevent mutating a collection while iterating over it.

* List Virtualization: I implemented UI virtualization (`show_rows`) to calculate rendering bounds dynamically. Instead of rendering thousands of tasks and relying on a scrollbox, the app only computes the UI for the exact tasks visible on the screen, keeping CPU and RAM overhead practically at zero.

* Linux Desktop Specifications: I gained hands-on experience bridging the gap between a raw binary and a native Linux desktop experience by configuring standard `.desktop files`, mapping Wayland/X11 icon paths, and managing XDG-compliant file structures.


## Future Improvements:

#### While the core architecture is highly performant, the feature set could be expanded to support heavier workflows:

* Drag-and-Drop Reordering: Implementing state-swapping logic to allow users to prioritize tasks manually.

* Metadata & Deadlines: Expanding the `Task` struct to include `chrono` timestamps for due dates, allowing for automatic sorting by urgency.

* Theming Engine: Utilizing `egui's` visual context properties to implement custom color palettes, accent colors, and a user-controlled Light/Dark mode toggle.

* Keyboard Shortcuts: Adding global keybind listeners (e.g., `Ctrl+Enter` to quick-add a task or `Del` to remove hovered items) for power users.


## Install Guide:

#### I use Arch Linux, and this is what worked for me.
#### Prerequisites 
- `rustup` (Cargo and `rustc`)
- `sudo pacman -S base-devel pkgconf libxkbcommon`

### 1.) Clone and build
```
git clone https://github.com/WarrenWarrens/ToDo_App.git
cd todo_app
cargo build --release
``` 
### 2.) Install the Binary and Icon


```
mkdir -p ~/.local/bin ~/.local/share/icons
cp target/release/todo_app ~/.local/bin/
cp assets/todosprite.png ~/.local/share/icons/
``` 
### 3.) Register the DesktopEntry
#### Create a file at `~/.local/share/applications/todoapp.desktop` and add the following (replace `YOUR_USERNAME` with your actual Linux user)


```
[Desktop Entry]
Name=Rust To-Do List
Comment=A fast and lightweight to-do app written in Rust
Exec=/home/YOUR_USERNAME/.local/bin/todo_app
Icon=/home/YOUR_USERNAME/.local/share/icons/todosprite.png
Type=Application
Terminal=false
Categories=Utility;Productivity;
``` 

### 4.) Refresh Application Launcher


```
update-desktop-database ~/.local/share/applications
``` 

#### Launch the app from your standard Linux application menu, or run it via the terminal using `todo_app`
```
cargo run
``` 



