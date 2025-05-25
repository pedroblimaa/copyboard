# ✂️ Copyboard

**Copyboard** is a lightweight Rust application that synchronizes your clipboard across multiple devices using Dropbox. It's designed for simplicity and minimal setup—just install it on your devices, and your clipboard will stay in sync.

---

## 🚀 Features

- 🔄 Real-time clipboard synchronization via Dropbox  
- 🖥️ Cross-platform support (Linux, Windows)  
- ⚙️ Minimal configuration required  
- 📁 Uses Dropbox's file change detection for efficient syncing  

---

## 🛠️ Installation

### Steps

1. **Download the installer:**  

Visit the [releases page](https://github.com/pedroblimaa/copyboard/releases) of this project and download the latest installer for your operating system.

> Supported platforms: Windows and Linux.

2. **Install the application:**  

- **Windows:** Double-click the downloaded `.exe` or `.msi` file and follow the setup wizard.  
- **Linux:** Depending on your distribution, run the installer from your terminal or use your package manager if available.

3. **Launch and log in:**  

Open the app. A browser window will automatically open prompting you to log in to your Dropbox account to authorize clipboard syncing.

---

## 💡 How It Works

1. **On Device A (Sender):**  
- Monitors the clipboard for changes.  
- Upon detecting a change, uploads the new content to Dropbox as a file.  

2. **On Device B (Receiver):**  
- Listens for file changes in Dropbox using the Longpoll API.  
- When a change is detected:  
  - Downloads the updated file.  
  - Updates the cursor using `/list_folder/continue`.  
  - Updates the local clipboard with the new content.  

This approach ensures that your clipboard stays synchronized across devices with minimal delay.  

---

## Configuration for Development
Before running the app, you need to create a file named ` local_config.rs` at:
```bash
src-tauri/src/config/local_config.rs
```
This file should define the following environment variables:

```rust
pub const DROPBOX_KEY: &str = "<Dropbox app key>";
pub const DROPBOX_SECRET: &str = "<Dropbox app secret>";
```

You can obtain these by creating an app in the [Dropbox App Console](https://www.dropbox.com/developers/apps).


> ⚠️ This file is required **only for development** and should not be committed to version control.

> Note: The console may not work on Chrome, so you may need another browser to use it.
---

## 📈 Performance

- File update: ~1s
- Detect file changes: ~1s
- File download: ~1.2s  

- Total time between copy and sync: ~3.2s

---

## 📋 Roadmap

- [ ] Implement end-to-end encryption for clipboard content  
- [ ] Add support for image and rich text clipboard data  
- [ ] Develop a GUI for easier configuration and status monitoring  
- [ ] Explore alternative backends (e.g., Microsoft Graph) for improved real-time performance  

---

## 📝 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.  

---

## 🙋‍♂️ Author

**Pedro B Lima** – [@pedroblimaa](https://github.com/pedroblimaa)
