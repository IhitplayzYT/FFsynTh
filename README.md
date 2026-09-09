# FFsynTh

A Cava-style TUI (Terminal User Interface) audio spectrum visualizer written in Rust. FFsynTh captures real-time audio from your system and displays it as a dynamic frequency visualization in your terminal.

## Why FFsynTh?

- **Lightweight**: Runs in your terminal without heavy GUI dependencies
- **Customizable**: Full control over colors, bar count, and amplification
- **Performance**: Built in Rust for efficient real-time audio processing
- **Integration**: Perfect for tiling window managers and terminal-based workflows

## Features

### Current Features
- Real-time audio capture from system input devices (PipeWire, ALSA, default)
- Configurable number of visualization bars
- Color gradient support with automatic interpolation
- Amplification control for sensitivity adjustment

### Planned Features
- **Visualization Variations**: Multiple visualization modes (bars, wave, circular, etc.)
- **Live TUI Configuration**: Change settings on-the-fly without restarting
- **Metrics Dashboard**: Real-time audio statistics (peak levels, frequency distribution, etc.)
- **Preset System**: Save and load visualization presets
- **Color Themes**: Built-in color schemes and theme switching
- **Audio Source Selection**: Interactive device picker
- **Configuration File**: Persistent settings via config file

## Installation

### Prerequisites
- Cargo
- Rust
- Audio system: PipeWire (recommended) or ALSA

### Build from Source
```bash
git clone https://github.com/yourusername/FFsynTh.git
cd FFsynTh
cargo build --release
```

The compiled binary will be available at `target/release/ffsynth`.

### Install System-Wide (Optional)
```bash
sudo cp target/release/ffsynth /usr/local/bin/
```

## Usage

### Basic Usage
```bash
ffsynth
```
Runs with default settings (5 bars, red-to-blue gradient, pipewire audio source).

### Command-Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--help` | `-h` | Show help message | - |
| `--debug` | `-d` | Enable debug mode | `false` |
| `--n_bars` | `-n` | Number of bars | `5` |
| `--amplification` | `-amp` | Amplification factor | `1.0` |
| `--src_audio` | `-src` | Audio source device | `pipewire` |
| `--color` | `-c` | Add color to gradient | Auto-generated |

### Examples

#### Default visualization
```bash
ffsynth
```

#### Debug mode with 20 bars
```bash
ffsynth -d -n=20
```

#### Custom color gradient (red to blue)
```bash
ffsynth -c=255,0,0 -c=0,0,255
```

#### High sensitivity with 30 bars
```bash
ffsynth -amp=1.5 -n=30
```

#### Use default audio device
```bash
ffsynth -src=default
```

#### Complex gradient (purple to cyan to yellow)
```bash
ffsynth -c=128,0,128 -c=0,255,255 -c=255,255,0 -n=15
```

#### Single color (fades to transparent)
```bash
ffsynth -c=255,100,50 -n=10
```

### Color Format

Colors are specified as RGB or RGBA values (0-255):
- **RGB**: `-c=R,G,B` (alpha defaults to 255)
- **RGBA**: `-c=R,G,B,A` (explicit alpha channel)

Examples:
- Red: `-c=255,0,0`
- Semi-transparent green: `-c=0,255,0,128`
- Blue with full opacity: `-c=0,0,255,255`

### Color Interpolation

FFsynTh automatically interpolates between colors to create smooth gradients:
- **1 color**: Fades from that color to transparent
- **2+ colors**: Creates a gradient interpolating between all specified colors
- The gradient is automatically expanded to match the number of bars

### Project Structure
```
FFsynTh/
├── src/
│   ├── main.rs      # Entry point and audio stream setup
│   ├── helper.rs    # CLI argument parsing and help
│   └── model.rs     # Color models and gradient logic
├── Cargo.toml       # Rust dependencies
└── README.md        # This file
```

### Dependencies
- `cpal`: Cross-platform audio I/O library
- `rustfft`: FFT for frequency analysis
- `ratatui`: TUI library

### Building
```bash
cargo build
```

### Running in Debug Mode
```bash
cargo run -- -d -n=10
```

## Troubleshooting

### No Audio Input
- Ensure your audio system (PipeWire/ALSA) is running
- Check available devices with `pactl list sources` (PipeWire) or `arecord -l` (ALSA)
- Try specifying a different source with `-src=default`

### Permission Denied
- Ensure your user has access to audio devices
- On Linux, add user to `audio` group: `sudo usermod -a -G audio $USER`

### Compilation Errors
- Ensure Rust is up to date: `rustup update`
- Check that edition is set to "2021" in Cargo.toml

## License

GPL-3.0-only - See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Areas of interest:
- Additional visualization modes
- TUI configuration interface
- Metrics dashboard implementation
- Preset system
- Color theme library

## Roadmap

- [ ] Live TUI configuration (change settings without restart)
- [ ] Multiple visualization modes (wave, circular, spectrum)
- [ ] Metrics dashboard (peak levels, frequency stats)
- [ ] Configuration file support
- [ ] Preset system for saving configurations
- [ ] Built-in color themes
- [ ] Interactive audio device picker
- [ ] Performance optimizations
- [ ] Windows and macOS support improvements

## Acknowledgments

Inspired by [Cava](https://github.com/karlstav/cava), the original console audio visualizer.
