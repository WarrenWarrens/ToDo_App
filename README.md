# ToDo App

### Made entirely with Rust

## Install Guide 

#### I use Arch Linux, and this is what worked for me.
#### Prerequisites 
- `rustup` (Cargo and `rustc`)
- `sudo pacman -S base-devel pkgconf libxkbcommon`

### 1.) Clone and build
```
git clone <your-repo-url>
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



