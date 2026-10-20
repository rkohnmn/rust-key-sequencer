<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/keytape-logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="assets/keytape-logo.svg">
  <img alt="Keytape — Rust Key & Mouse Sequencer" src="assets/keytape-logo.svg" width="340">
</picture>

# ⚡ Keytape — Rust Key & Mouse Sequencer
### Ultra-Fast, Zero-Bloat Keyboard & Mouse Macro Automation Suite

[![Rust Version](https://img.shields.io/badge/rust-2021_edition-DEA584.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Windows_10%20%7C%2011-0078D6.svg?style=for-the-badge&logo=windows)](https://www.microsoft.com/windows)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg?style=for-the-badge)](LICENSE)
[![Framework](https://img.shields.io/badge/GUI-egui%20%2F%20eframe-blueviolet.svg?style=for-the-badge)](https://github.com/emilk/egui)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=for-the-badge&logo=githubactions)](https://github.com/rkohnmn/rust-key-sequencer/actions)

**A blazing-fast, ultra-lightweight desktop macro & automation suite written in 100% native Rust.**  
*Automate repetitive keystrokes, mouse clicks, hold patterns, and complex chords with millisecond accuracy.*

[Quick Start](#-quick-start-in-30-seconds) •
[Features](#-key-features) •
[Presets](#-instant-one-click-presets) •
[Branding](#-branding--assets) •
[How It Works](#-how-to-use-it-step-by-step) •
[Build from Source](#-building-from-source) •
[FAQ](#-troubleshooting--faq)

</div>

---

## 🌟 Why Rust Key Sequencer?

Most automation software today is either:
1. **Bloated Electron apps** that consume 500 MB+ of RAM and lag your games.
2. **Suspicious auto-clickers** downloaded from random ad-infested forums.
3. **Overly complicated scripting engines** that require learning proprietary programming syntax.

**Rust Key Sequencer** is the modern alternative:
- **Zero Bloat**: A single standalone native `.exe` file (~5 MB). No runtime installs, no webview, no background telemetry.
- **Ultra-Low Resource Footprint**: Uses only **~15 MB of RAM** and almost 0% CPU during execution.
- **Intuitive Visual Builder**: Clean, dark slate GUI built with immediate-mode graphics (`egui`).
- **Precision Timing**: Configurable millisecond-level hold durations, delay intervals, and safety countdowns.
- **Save & Share**: Save your macros as human-readable `.json` presets to share with friends or teammates.

---

## 🖥️ UI Overview

```text
┌────────────────────────────────────────────────────────────────────────────────────────┐
│  ⚡ RUST KEY & MOUSE SEQUENCER                                           [ —  □  ✕ ]   │
├────────────────────────────────────────────────────────────────────────────────────────┤
│  ⚡ QUICK PRESETS:                                                                     │
│  [ Ctrl + Enter ]  [ Ctrl+C / Ctrl+V ]  [ Alt + Tab ]  [ Rapid Click ]  [ WASD Loop ]   │
├─────────────────────────────────────────┬──────────────────────────────────────────────┤
│  🎯 STEP CONFIGURATOR                   │  📋 SEQUENCE TIMELINE (3 Steps • 950ms)      │
│                                         │                                              │
│  Selected Button: [ ⌨ Key: Enter    ▼ ] │  ┌────────────────────────────────────────┐  │
│                                         │  │ #1 [✓] Ctrl + Enter       (Tap / Click)│  │
│  Modifiers:                             │  │     Hold: 50ms | Delay: 300ms          │  │
│  [✓ Ctrl] [  Shift] [  Alt] [  Win]     │  │     [▲ Up] [▼ Down] [📋 Copy] [🗑 Del] │  │
│                                         │  ├────────────────────────────────────────┤  │
│  Action Mode:                           │  │ #2 [✓] Space              (Tap / Click)│  │
│  (●) Tap / Click  ( ) Hold Down  ( ) Up │  │     Hold: 50ms | Delay: 400ms          │  │
│                                         │  ├────────────────────────────────────────┤  │
│  Hold Time:   [ 50  ] ms                │  │ #3 [✓] Left Click         (Tap / Click)│  │
│  Delay After: [ 250 ] ms                │  │     Hold: 50ms | Delay: 250ms          │  │
│                                         │  └────────────────────────────────────────┘  │
│  [ ➕ ADD STEP TO SEQUENCE ]            │  [ Select All ]  [ Deselect All ]  [ Clear ] │
├─────────────────────────────────────────┴──────────────────────────────────────────────┤
│  ⚙️ EXECUTION CONTROLS:                                                                │
│  Start Delay: [ 3 ] sec    Repeat: [ 1 ] cycles    [ ] Infinite Loop                   │
│                                                                                        │
│  [  ▶  START SEQUENCE  ]       [  ⏹  STOP SEQUENCE  ]       [ 💾 Save ]  [ 📂 Load ]  │
│                                                                                        │
│  STATUS: Ready. Press START SEQUENCE to begin.                                         │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## ✨ Key Features

- **🎹 Comprehensive Input Arsenal**:
  - **Alphabet**: `A` through `Z`
  - **Numbers**: `0` through `9`
  - **Function Keys**: `F1` through `F12`
  - **Navigation & Editing**: `Space`, `Enter`, `Tab`, `Backspace`, `Escape`, Arrow Keys (`Up`, `Down`, `Left`, `Right`)
  - **Modifiers**: `Ctrl`, `Shift`, `Alt`, `Win`
  - **Mouse Controls**: `Left Click`, `Right Click`, `Middle Click`

- **🔗 Chords & Combinations**:
  Combine any key or mouse click with modifier keys (e.g., `Ctrl + Shift + S`, `Alt + Tab`, `Ctrl + Left Click`) or add custom extra companion keys.

- **⏱️ Three Flexible Action Modes**:
  - **Tap / Click**: Presses the key/button, holds it for the specified duration (e.g., 50ms), and releases it.
  - **Hold Down**: Presses and keeps the key held down (ideal for continuous walking or holding aim).
  - **Release**: Releases a previously held key.

- **🛡️ Safety-First Countdown**:
  A configurable countdown (default: 3 seconds) gives you plenty of time to click into your target game or application before inputs start firing.

- **⚡ Instant Thread Abortion**:
  The background execution engine listens on a high-frequency atomic interrupt flag. Clicking **STOP SEQUENCE** halts automation within milliseconds—no runaway macros!

- **💾 JSON Preset System**:
  Export your custom key routines to `.json` or load existing workflows with one click.

---

## ⚡ Instant One-Click Presets

The top preset bar allows you to load tested automation templates instantly:

| Preset Name | Actions Performed | Typical Use Case |
| :--- | :--- | :--- |
| **Ctrl + Enter** | Sends `Ctrl + Enter` with clean modifier release | Submitting forms, AI chat prompts, code execution |
| **Ctrl+C / Ctrl+V** | Copies selected text, pauses 150ms, then pastes | Repetitive text duplication and workflow automation |
| **Alt + Tab** | Sends `Alt + Tab` with 500ms delay | Quick application and window switching |
| **Rapid Fire Click** | Rapid 25ms mouse clicks with 40ms pauses | In-game auto-clicking, high-speed UI interaction |
| **WASD Loop** | Walks forward (`W`), right (`D`), backward (`S`), left (`A`) | In-game character patrol, anti-AFK timeout prevention |
| **Jump & Attack** | Presses `Space` (jump), pauses 100ms, then `Left Click` | Action game combat combo |
| **Hold Right Click** | Holds `Right Click` down for 2,000ms, then releases | Sustained aim-down-sights or spell-channeling |

---

## 🚀 Quick Start in 30 Seconds

### For Non-Developers (Pre-Compiled Binary)
1. Download `rust_key_sequencer.exe` from the [Releases](https://github.com/rkohnmn/rust-key-sequencer/releases) page (or from the included `dist/` directory).
2. Double-click `rust_key_sequencer.exe` to run. No installer or runtime dependencies required!

> [!NOTE]  
> If you are automating actions inside a game or an application running as Administrator (like Task Manager), make sure to **Right Click -> Run as administrator** on `rust_key_sequencer.exe`.

---

## 📖 How to Use It (Step-by-Step)

### 1. Choose Your Button
In the **Step Configurator** on the left:
- Pick any key or mouse button from the dropdown menu (e.g., `⌨ Key: E`, `🖱 Left Click`).

### 2. Add Modifiers (Optional)
Toggle any combination of:
- `[Ctrl]` • `[Shift]` • `[Alt]` • `[Win]`  
For example, check `Ctrl` and select `C` to create a `Ctrl + C` copy step.

### 3. Set Your Timing
- **Hold Time (ms)**: How long the button is physically pressed down (50ms is standard for a human-like tap).
- **Delay After (ms)**: How long to wait *after* this step before executing the next one.

### 4. Add & Organize Steps
- Click **➕ Add Step to Sequence**.
- Use **▲ Up** and **▼ Down** to reorder steps.
- Use **📋 Duplicate** to clone a step.
- Uncheck the checkbox **[✓]** on any step to temporarily disable it without deleting it.

### 5. Configure Execution & Launch
- Set **Start Delay** (e.g., `3` seconds) so you have time to switch to your target window.
- Set **Repeat Count** (e.g., `5` times) or check **Infinite Loop**.
- Click **▶ START SEQUENCE**, immediately switch to your target game or app window, and let the sequencer do the rest!
- Need to stop early? Click **⏹ STOP SEQUENCE** or press your assigned stop button.

---

## 📄 JSON Preset Format

All sequence configurations can be saved to or loaded from standard `.json` files. Here is what a simple macro file looks like:

```json
[
  {
    "button": "W",
    "modifiers": { "ctrl": false, "shift": false, "alt": false, "win": false },
    "extra_keys": [],
    "mode": "Click",
    "hold_ms": 300,
    "delay_after_ms": 100,
    "enabled": true
  },
  {
    "button": "MouseLeft",
    "modifiers": { "ctrl": false, "shift": false, "alt": false, "win": false },
    "extra_keys": [],
    "mode": "Click",
    "hold_ms": 50,
    "delay_after_ms": 250,
    "enabled": true
  }
]
```

---

## 🛠️ Building from Source

### Prerequisites
- Install the official Rust toolchain via [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Build & Run
Clone this repository and compile with Cargo:

```bash
# 1. Clone the repository
git clone https://github.com/rkohnmn/rust-key-sequencer.git
cd rust-key-sequencer

# 2. Run in development mode
cargo run

# 3. Build optimized release binary
cargo build --release
```

### One-Click Windows Batch Script
On Windows, you can simply double-click:
```bat
build_windows_exe.bat
```
This automatically compiles the release executable with Windows subsystem flags (hiding the command console window) and outputs the binary directly to `dist\rust_key_sequencer.exe`.

---

## 🎨 Branding & Assets

Keytape features a modern, bespoke vector branding suite located in [`assets/`](assets/):

| Asset | Preview | Description |
| :--- | :---: | :--- |
| **App Icon**<br>[`assets/keytape-icon.svg`](assets/keytape-icon.svg) | <img src="assets/keytape-icon.svg" width="48" height="48" alt="Keytape App Icon"> | Rounded-square application and OS taskbar icon featuring the signature copper-orange keypress indicator. |
| **Light Wordmark**<br>[`assets/keytape-logo.svg`](assets/keytape-logo.svg) | <img src="assets/keytape-logo.svg" width="200" alt="Keytape Light Logo"> | Wordmark with charcoal lettering tuned for light documentation and web themes. |
| **Dark Wordmark**<br>[`assets/keytape-logo-dark.svg`](assets/keytape-logo-dark.svg) | <img src="assets/keytape-logo-dark.svg" width="200" alt="Keytape Dark Logo"> | High-contrast cream wordmark tuned for dark mode GitHub previews and UI headers. |

---

## 🏗️ Technical Architecture

| Component | Technology | Description |
| :--- | :--- | :--- |
| **GUI Framework** | [`egui`](https://github.com/emilk/egui) & [`eframe`](https://docs.rs/eframe) | Immediate-mode GPU-rendered interface running at native 60 FPS. |
| **Asset Pipeline** | [`egui_extras`](https://docs.rs/egui_extras), [`resvg`](https://docs.rs/resvg), [`usvg`](https://docs.rs/usvg) | Native SVG rendering with theme-aware assets and dynamic window taskbar icon synthesis. |
| **Input Engine** | [`enigo`](https://github.com/enigo-rs/enigo) | Low-level OS input synthesis simulating hardware keyboard and mouse events. |
| **Serialization** | [`serde`](https://serde.rs/) & [`serde_json`](https://docs.rs/serde_json) | High-speed JSON serialization for macro export and import. |
| **File Dialogs** | [`rfd`](https://docs.rs/rfd) | Native Windows File Explorer open/save dialogs. |
| **Subsystem Linking**| `build.rs` | Custom Windows subsystem linker configuration (`/SUBSYSTEM:WINDOWS`) so the application launches cleanly without a black console window. |

---

## ❓ Troubleshooting & FAQ

### Q: Why are keystrokes not registering inside my game or elevated software?
> **A:** Windows enforces **UIPI (User Interface Privilege Isolation)**. Standard user applications are prevented from sending synthetic input to elevated (Administrator) windows.  
> **Solution**: Right-click `rust_key_sequencer.exe` and select **"Run as administrator"**.

### Q: How do I stop a macro immediately if it is looping infinitely?
> **A:** Click the prominent red **⏹ STOP SEQUENCE** button in the app. The execution worker sleeps in tiny 25ms increments, ensuring it terminates almost instantly upon user request.

### Q: Is there any risk of malware or telemetry?
> **A:** Absolutely none. Rust Key Sequencer is 100% open source under the MIT License, contains zero analytics or telemetry, makes zero network requests, and operates completely offline.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m "Add some AmazingFeature"`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📜 License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for more details.

---

<div align="center">
Built with ❤️ and 🦀 by <a href="https://github.com/rkohnmn">rkohnmn</a>
</div>
