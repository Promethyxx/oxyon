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

# Create a key.env file in the root directory and add:
# TMDB_KEY=your_tmdb_key
# FANART_KEY=your_fanart_key

cargo build --release```

⚠️ Make sure the external tools (7zip, ExifTool, FFmpeg, MKVToolNix, Pandoc) are installed and accessible in your PATH.