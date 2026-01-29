# Oxyon

Oxyon is a Rust converter that provides a GUI interface for major file processing libraries:

- **7zip** – compression/decompression
- **ExifTool** – metadata management
- **FFmpeg** – audio and video processing
- **MKVToolNix** – MKV file manipulation
- **Pandoc** – document conversion

Oxyon makes these powerful tools easy to use through a simple graphical interface.

---

## Installation

Clone the repository:

```bash
git clone https://github.com/Promethyxx/oxyon
cd oxyon
cargo build --release

⚠️ Make sure the external tools (7zip, ExifTool, FFmpeg, MKVToolNix, Pandoc) are installed and accessible in your PATH.