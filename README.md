# Oxyon

![Oxyon Logo](oxyon_logo.png)

[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE.txt)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**Oxyon** is a unified multimedia toolbox built in Rust:  
file conversion, movie & TV scraping, and media tagging — all in one graphical interface.

## 🎯 Overview

Oxyon provides an easy-to-use GUI interface for major file processing libraries  
**and media databases**, making complex multimedia workflows accessible without the command line.
- **7zip** – Archive compression and decompression
- **ExifTool** – Metadata reading and writing for images, videos, and documents
- **FFmpeg** – Audio and video encoding, decoding, and conversion
- **MKVToolNix** – MKV file creation and manipulation
- **Pandoc** – Universal document converter

## ✨ Features

### 🔁 File Conversion
- 🎬 **Video Conversion**: Convert between common video formats (MP4, MKV, AVI, etc.)
- 🎵 **Audio Processing**: Extract, convert, and process audio tracks
- 📄 **Document Conversion**: Transform documents between formats (PDF, DOCX, HTML, Markdown…)
- 📦 **Archive Management**: Create and extract compressed archives

### 🏷️ Metadata & Tagging
- 🖼️ **Image Metadata**: Read and edit EXIF metadata
- 🎞️ **Media Tagging**: Write metadata directly into media files (titles, dates, artwork)

### 🎭 Media Scraping
- 🎬 **Movie & TV Metadata**: Fetch information from TMDB and FanArt
- 🖼️ **Artwork Retrieval**: Posters, backdrops, logos, and fanart

### 🎨 Interface
- 🧭 **Unified GUI**: One consistent interface for all tools
- ⚙️ **Tool Abstraction**: Powerful CLI tools without command-line complexity

## 📋 Prerequisites

Before installing Oxyon, ensure you have the following tools installed and accessible in your system PATH:

- [7-Zip](https://www.7-zip.org/) or [p7zip](https://p7zip.sourceforge.net/) (for archives)
- [ExifTool](https://exiftool.org/) (for metadata)
- [FFmpeg](https://ffmpeg.org/) (for audio/video)
- [MKVToolNix](https://mkvtoolnix.download/) (for MKV files)
- [Pandoc](https://pandoc.org/) (for documents)
- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)

### Verify Installation

```bash
7z --help
exiftool -ver
ffmpeg -version
mkvmerge --version
pandoc --version
```

## 🚀 Installation

1. **Clone the repository**

```bash
git clone https://github.com/Promethyxx/oxyon.git
cd oxyon
```

2. **Set up API keys** (optional, for media metadata features)

Create a `key.env` file in the root directory:

```bash
cp example.env key.env
```

Then edit `key.env` and add your API keys:

```env
TMDB_KEY=your_tmdb_api_key_here
FANART_KEY=your_fanart_api_key_here
```

**How to get API keys:**
- TMDB: Register at [https://www.themoviedb.org/settings/api](https://www.themoviedb.org/settings/api)
- FanArt: Register at [https://fanart.tv/get-an-api-key/](https://fanart.tv/get-an-api-key/)

> ⚠️ **Note**: API keys are only required if you want to use the media metadata features. Basic file conversion works without them.

3. **Build the project**

```bash
cargo build --release
```

4. **Run Oxyon**

```bash
./target/release/oxyon
```

Or on Windows:

```bash
.\target\release\oxyon.exe
```

## 📸 Screenshots

### Archive
![Archive Module](screenshots/Archive.png)

### Audio
![Audio Module](screenshots/Audio.png)

### Document
![Document Module](screenshots/Doc.png)

### Image
![Image Module](screenshots/Image.png)

### Scraping
![Scraping Module](screenshots/Scrap.png)

### Media Tagging
![Tagging Module](screenshots/Tag.png)

### Video
![Video Module](screenshots/Video.png)

## 💻 Usage

1. Launch the application
2. Select your input file(s)
3. Choose the desired output format
4. Configure conversion settings (optional)
5. Click "Convert" and wait for the process to complete

### 🎬 Typical Workflow

1. Load a movie or TV episode
2. Fetch metadata from TMDB
3. Automatically tag the file (title, year, artwork)
4. Convert or remux if needed
5. Export a clean, ready-to-archive media file

### Example Use Cases

- **Convert video to MP4**: Load a video file, select MP4 output, adjust quality settings
- **Extract audio from video**: Choose audio-only output format (MP3, FLAC, etc.)
- **Edit photo metadata**: Load images and modify EXIF data
- **Create archives**: Bundle files into compressed archives
- **Convert documents**: Transform Markdown to PDF, DOCX to HTML, etc.

## 🏗️ Project Structure

```
oxyon/
├── src/                 # Source code
├── Cargo.toml           # Rust dependencies
├── build.rs             # Build script
├── oxyon_config.toml    # Configuration file
├── oxyon_icon.ico       # Application icon
├── oxyon_logo.png       # Project logo
├── example.env          # Example environment variables
└── README.md            # This file
```

## 🛠️ Configuration

Edit `oxyon_config.toml` to customize:

- Default output directories
- Quality presets
- UI preferences
- Tool-specific settings

## 🐛 Known Issues

Check the [Issues](https://github.com/Promethyxx/oxyon/issues) page for current bugs and feature requests.

## 🗺️ Roadmap (planned)

The features are actively planned and tracked via GitHub issues.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📝 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE.txt](LICENSE.txt) file for details.

## 🙏 Acknowledgments

This project wouldn't be possible without these amazing open-source tools:

- [7-Zip](https://www.7-zip.org/) by Igor Pavlov
- [ExifTool](https://exiftool.org/) by Phil Harvey
- [FFmpeg](https://ffmpeg.org/) by FFmpeg team
- [MKVToolNix](https://mkvtoolnix.download/) by Moritz Bunkus
- [Pandoc](https://pandoc.org/) by John MacFarlane

## 📧 Contact

- GitHub: [@Promethyxx](https://github.com/Promethyxx)
- Project Link: [https://github.com/Promethyxx/oxyon](https://github.com/Promethyxx/oxyon)

---

**Made with ❤️ and Rust**