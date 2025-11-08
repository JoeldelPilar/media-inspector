# 🎬 Media File Inspector

A command-line tool written in Rust that inspects video files and displays technical information about them. Perfect for developers learning Rust and video streaming technologies.

## 📋 Description

Media File Inspector is a CLI tool that analyzes video files and displays detailed technical information. This project is created as a learning project to understand both Rust programming and video streaming technologies.

## ✨ Features

- 📊 **File Size** - Displays the size of the video file
- 🎥 **Codec Information** - Shows video codec details (H.264, H.265, AV1)
- 📐 **Video Resolution** - Displays width and height of the video
- ⏱️ **Duration** - Shows the total duration of the video
- 📈 **Bitrate** - Displays the bitrate information

## 🚀 Installation

### Prerequisites

- **Rust** (latest stable version recommended)
- **Cargo** (comes with Rust)

### Install Rust

If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd media-inspector

# Build the project
cargo build --release

# Or run directly
cargo run --release
```

## 💻 Usage

### Basic Usage

```bash
# Inspect a video file
cargo run --release -- <path-to-video-file>

# Example
cargo run --release -- video.mp4
```

### After Installation

Once built, you can run the binary directly:

```bash
# Run the binary
./target/release/media-inspector <path-to-video-file>

# Example
./target/release/media-inspector video.mp4
```

### Example Output

```
📹 Media File Inspector
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

File: video.mp4
Size: 125.5 MB
Codec: H.264
Resolution: 1920x1080
Duration: 00:05:23
Bitrate: 2500 kbps
```

## 🛠️ Technical Requirements

- **Rust Edition**: 2024
- **Build Tool**: Cargo
- **Platform**: Cross-platform (Windows, macOS, Linux)

## 🔧 Development Setup

### Getting Started

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd media-inspector
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests** (when implemented)
   ```bash
   cargo test
   ```

4. **Run in development mode**
   ```bash
   cargo run -- <path-to-video-file>
   ```

### Project Structure

```
media-inspector/
├── Cargo.toml          # Project dependencies and metadata
├── Cargo.lock          # Locked dependency versions
├── src/
│   └── main.rs         # Main application entry point
└── README.md           # This file
```

## 🎓 Learning Goals

This project is designed to learn:

- **Rust Programming**
  - Ownership and borrowing
  - Error handling
  - CLI development
  - File I/O operations
  - Structs and enums

- **Video Streaming Technologies**
  - Video codecs (H.264, H.265, AV1)
  - Container formats
  - Media metadata parsing
  - Bitrate calculations
  - Video file structure

## 📚 Dependencies

Current dependencies will be added as the project develops. Check `Cargo.toml` for the latest list.

## 🤝 Contributing

This is a learning project, but contributions and suggestions are welcome! Feel free to open issues or submit pull requests.

## 👤 Author

**Joel del Pilar**  
[@JoeldelPilar](https://github.com/JoeldelPilar)

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Rust community for excellent documentation
- Video streaming technology resources
- Open source media analysis tools

---

Made with ❤️ and Rust 🦀

