# Kinetic Trading Terminal

Professional-grade cryptocurrency trading terminal built with **Tauri (Rust) + React/TypeScript**.

## Features

- **L3 Order Book Aggregation** across Big Four exchanges (Binance, OKX, Bybit, Coinbase)
- **Cap'n Proto Zero-Copy Serialization** for sub-second latency
- **Multi-Window Architecture** with SharedWorker data synchronization
- **AI-Powered News Sentiment** using OpenAI gpt-4o-mini ($50-100/month budget)
- **Advanced Risk Management** with Greeks calculator and auto-killswitch
- **Global Command Bar (Alt+K)** with fuzzy search
- **Real-time Analytics**: Correlation matrices, liquidation maps, flow metrics

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Backend** | Rust + Tauri | Native performance, security, WebSocket handling |
| **Serialization** | Cap'n Proto | Zero-copy RPC, superior network performance |
| **Frontend** | React 18 + TypeScript | UI components and state management |
| **State** | Zustand | Lightweight, performant state management |
| **Charts** | Lightweight Charts | High-performance canvas-rendered charts |
| **AI** | OpenAI gpt-4o-mini | Sentiment analysis ($0.15/1M input tokens) |
| **Fonts** | Inter + JetBrains Mono | Professional typography with tabular figures |

## Project Structure

```
terminal/
├── schemas/
│   └── market_data.capnp          # Cap'n Proto schema definitions
├── src-tauri/                     # Rust backend
│   ├── src/
│   │   ├── main.rs                # Entry point
│   │   ├── commands.rs            # Tauri IPC commands
│   │   ├── ipc_bus.rs             # Cap'n Proto streaming
│   │   ├── websocket/             # Exchange WebSocket clients
│   │   ├── orderbook/             # L3 aggregation
│   │   ├── analytics/             # Correlation, liquidations, flow
│   │   ├── news/                  # News aggregation + sentiment
│   │   ├── risk/                  # Greeks, killswitch
│   │   └── api/                   # Secure storage
│   ├── Cargo.toml                 # Rust dependencies
│   └── build.rs                   # Cap'n Proto build script
└── src/                           # React frontend
    ├── components/
    │   ├── GlobalHeader.tsx       # Ticker tape header
    │   ├── Footer.tsx             # Connection status
    │   └── Canvas/
    │       └── WorkspaceCanvas.tsx # 6-module grid layout
    ├── styles/
    │   ├── globals.css            # Design system variables
    │   ├── theme.css              # Trading-specific colors
    │   └── typography.css         # Inter + JetBrains Mono
    ├── App.tsx
    └── main.tsx
```

## Getting Started

### Prerequisites

- **Rust** (1.70+): https://rustup.rs/
- **Node.js** (18+): https://nodejs.org/
- **Cap'n Proto** compiler: https://capnproto.org/install.html

### Installation

```bash
# Install frontend dependencies
npm install

# Install Rust dependencies (automatically via cargo)
cd src-tauri
cargo build

# Return to root
cd ..
```

### Development

```bash
# Run in development mode
npm run tauri dev
```

### Build

```bash
# Build for production
npm run tauri build
```

## Design System

### Colors
- Background: `#0A0B0D`
- Surface: `#14171A`
- Bullish: `#00FFA3` (Electric Neon Green)
- Bearish: `#FF3B69` (Vivid Crimson)

### Typography
- **Headings**: Inter (tight tracking `-0.02em`)
- **Data/Numbers**: JetBrains Mono (tabular figures prevent "jumping" text)

### Transitions
- All hover states: `100ms ease` (no fading/bouncing - speed is priority)

## Configuration

### API Keys (Secure Storage)

API keys are encrypted using OS-native secure storage:
- **Windows**: Credential Manager
- **macOS**: Keychain
- **Linux**: Secret Service

Keys never touch the frontend.

### Exchange Configuration

Priority exchanges (Phase 1):
1. **Binance** - Global liquidity
2. **Coinbase Advanced** - US institutional compliance
3. **OKX** - Advanced derivatives
4. **Bybit** - Secondary liquidity

## Architecture

See [`architecture_diagram.md`](./architecture_diagram.md) for detailed system architecture including:
- Multi-window + SharedWorker data flow
- Cap'n Proto serialization pipeline
- L3 order book aggregation
- WebSocket connection management

## Development Status

✅ **Complete**:
- Project structure and configuration
- Rust backend module stubs
- React frontend with design system
- Cap'n Proto schema definitions

🚧 **In Progress**:
- WebSocket client implementations
- Cap'n Proto serialization/deserialization
- L3 aggregation logic
- UI component library

📋 **Planned**:
- Command bar with fuzzy search
- Market overview (heatmap, correlation, liquidation map)
- News feed with AI sentiment
- Risk monitor with Greeks

## License

Proprietary - Kinetic Team

## Contact

For questions or support, contact the Kinetic development team.
