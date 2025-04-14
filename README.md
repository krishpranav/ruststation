# RustStation(ps4 emulator)

RustStation is a modern, high-accuracy PlayStation emulator engineered for enthusiasts, developers, and curious minds. Inspired by the legendary PlayStation architecture, this emulator aims to bridge retro computing charm with today's software best practices. Whether you're seeking the thrill of nostalgia or the challenge of exploring hardware-level precision, RustStation delivers.

## Features
- ✨ **Accurate Emulation** — Faithful replication of CPU, GPU, SPU, and system behaviors.
- 🌏 **Cross-Platform Support** — Smoothly runs on Windows, macOS, and Linux.
- ⚡ **Hardware Acceleration** — Render games using OpenGL or Vulkan for peak performance.
- ⚖️ **Robust Debugging Tools** — Disassembly views, memory visualization, and live state inspection.
- ⚒️ **Controller Support** — Seamless compatibility with DualShock, Xbox, XInput, and custom mappings.
- 🤖 **BIOS Authenticity** — Designed to work with original PlayStation BIOS files for complete hardware fidelity.

## Build Requirements:
Before diving in, make sure you have the right tools ready:
- Rust (Stable toolchain recommended).
- SDL2 development libraries (for audio, input, and rendering context).
- OpenGL or Vulkan-capable GPU.

## Building From Source:
Clone, build, and launch with confidence:
```bash
git clone https://github.com/krishpranav/ruststation.git
cd ruststation
python3 build.py
```

## Running Games
Boot your favorite classics with ease:
```bash
./target/release/ruststation path/to/your/game.iso
```

## Development Philosophy
At the heart of RustStation is a simple idea: **Precision fuels Experience**.

We believe emulators are more than nostalgia machines — they are educational playgrounds. Every opcode, every pixel clock, every DMA transfer offers insight into how consoles shaped the modern gaming landscape. RustStation is written with clean architecture and highly readable code to inspire learning while achieving rock-solid emulation.

- Focus on **hardware-level fidelity**
- Write **maintainable, modern Rust code**
- Design systems for both **performance and extensibility**
- Make developer tooling a first-class citizen

## Why This Project?
The original PlayStation redefined what home consoles could achieve, yet its intricate internals are often underappreciated. RustStation is here to change that — not only by recreating the original PlayStation experience pixel-for-pixel but by demystifying the black box for anyone willing to look under the hood.

Whether you're debugging, developing, or reminiscing, RustStation provides the stable and insightful experience you deserve.

## Contribution Guide
We welcome contributions from seasoned developers, reverse engineers, and newcomers alike! If you believe in code elegance, precise emulation, and open collaboration — you'll fit right in.

- Fork the repository.
- Submit clear, well-tested pull requests.
- Open thoughtful issues or discussions.
- Share tools, plugins, or integration ideas.

Review `CONTRIBUTING.md` for more details.

## License
RustStation is proudly open-source and MIT-licensed.

> "Software is a mirror of the people who write it. Build boldly and responsibly."

View the full license text in the `LICENSE` file.

## Acknowledgments
A standing ovation for the global open-source community, emulator authors past and present, and the developers of the original PlayStation, whose pioneering hardware and design philosophies inspired an entire generation of programmers.

Special thanks to:
- Contributors of low-level PlayStation documentation.
- Authors of earlier emulators who laid the groundwork.
- The global Rust and systems programming community.

---
RustStation — **Precision. Performance. Passion.**

Explore the silicon. Understand the design. Play the classics. And write the future.

Happy Hacking!

