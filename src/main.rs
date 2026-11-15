#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use enigo::{Button, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionButton {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    // Function Keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    // Special & Navigation Keys
    Space,
    Enter,
    Tab,
    Backspace,
    Escape,
    Shift,
    Ctrl,
    Alt,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    // Mouse Buttons
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

impl ActionButton {
    pub fn all() -> &'static [ActionButton] {
        use ActionButton::*;
        &[
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z,
            Num0,
            Num1,
            Num2,
            Num3,
            Num4,
            Num5,
            Num6,
            Num7,
            Num8,
            Num9,
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
            F7,
            F8,
            F9,
            F10,
            F11,
            F12,
            Space,
            Enter,
            Tab,
            Backspace,
            Escape,
            Shift,
            Ctrl,
            Alt,
            UpArrow,
            DownArrow,
            LeftArrow,
            RightArrow,
            MouseLeft,
            MouseRight,
            MouseMiddle,
        ]
    }

    pub fn label(&self) -> &'static str {
        use ActionButton::*;
        match self {
            A => "⌨ Key: A",
            B => "⌨ Key: B",
            C => "⌨ Key: C",
            D => "⌨ Key: D",
            E => "⌨ Key: E",
            F => "⌨ Key: F",
            G => "⌨ Key: G",
            H => "⌨ Key: H",
            I => "⌨ Key: I",
            J => "⌨ Key: J",
            K => "⌨ Key: K",
            L => "⌨ Key: L",
            M => "⌨ Key: M",
            N => "⌨ Key: N",
            O => "⌨ Key: O",
            P => "⌨ Key: P",
            Q => "⌨ Key: Q",
            R => "⌨ Key: R",
            S => "⌨ Key: S",
            T => "⌨ Key: T",
            U => "⌨ Key: U",
            V => "⌨ Key: V",
            W => "⌨ Key: W",
            X => "⌨ Key: X",
            Y => "⌨ Key: Y",
            Z => "⌨ Key: Z",
            Num0 => "🔢 Key: 0",
            Num1 => "🔢 Key: 1",
            Num2 => "🔢 Key: 2",
            Num3 => "🔢 Key: 3",
            Num4 => "🔢 Key: 4",
            Num5 => "🔢 Key: 5",
            Num6 => "🔢 Key: 6",
            Num7 => "🔢 Key: 7",
            Num8 => "🔢 Key: 8",
            Num9 => "🔢 Key: 9",
            F1 => "⚡ Key: F1",
            F2 => "⚡ Key: F2",
            F3 => "⚡ Key: F3",
            F4 => "⚡ Key: F4",
            F5 => "⚡ Key: F5",
            F6 => "⚡ Key: F6",
            F7 => "⚡ Key: F7",
            F8 => "⚡ Key: F8",
            F9 => "⚡ Key: F9",
            F10 => "⚡ Key: F10",
            F11 => "⚡ Key: F11",
            F12 => "⚡ Key: F12",
            Space => "⌨ Space",
            Enter => "⌨ Enter",
            Tab => "⌨ Tab",
            Backspace => "⌨ Backspace",
            Escape => "⌨ Escape",
            Shift => "⚙ Shift",
            Ctrl => "⚙ Ctrl",
            Alt => "⚙ Alt",
            UpArrow => "⬆ Up Arrow",
            DownArrow => "⬇ Down Arrow",
            LeftArrow => "⬅ Left Arrow",
            RightArrow => "➡ Right Arrow",
            MouseLeft => "🖱 Left Click",
            MouseRight => "🖱 Right Click",
            MouseMiddle => "🖱 Middle Click",
        }
    }

    pub fn short_name(&self) -> &'static str {
        use ActionButton::*;
        match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            H => "H",
            I => "I",
            J => "J",
            K => "K",
            L => "L",
            M => "M",
            N => "N",
            O => "O",
            P => "P",
            Q => "Q",
            R => "R",
            S => "S",
            T => "T",
            U => "U",
            V => "V",
            W => "W",
            X => "X",
            Y => "Y",
            Z => "Z",
            Num0 => "0",
            Num1 => "1",
            Num2 => "2",
            Num3 => "3",
            Num4 => "4",
            Num5 => "5",
            Num6 => "6",
            Num7 => "7",
            Num8 => "8",
            Num9 => "9",
            F1 => "F1",
            F2 => "F2",
            F3 => "F3",
            F4 => "F4",
            F5 => "F5",
            F6 => "F6",
            F7 => "F7",
            F8 => "F8",
            F9 => "F9",
            F10 => "F10",
            F11 => "F11",
            F12 => "F12",
            Space => "Space",
            Enter => "Enter",
            Tab => "Tab",
            Backspace => "Backspace",
            Escape => "Esc",
            Shift => "Shift",
            Ctrl => "Ctrl",
            Alt => "Alt",
            UpArrow => "Up",
            DownArrow => "Down",
            LeftArrow => "Left",
            RightArrow => "Right",
            MouseLeft => "Left Click",
            MouseRight => "Right Click",
            MouseMiddle => "Middle Click",
        }
    }

    pub fn to_enigo_key(&self) -> Key {
        use ActionButton::*;
        match self {
            A => Key::Unicode('a'),
            B => Key::Unicode('b'),
            C => Key::Unicode('c'),
            D => Key::Unicode('d'),
            E => Key::Unicode('e'),
            F => Key::Unicode('f'),
            G => Key::Unicode('g'),
            H => Key::Unicode('h'),
            I => Key::Unicode('i'),
            J => Key::Unicode('j'),
            K => Key::Unicode('k'),
            L => Key::Unicode('l'),
            M => Key::Unicode('m'),
            N => Key::Unicode('n'),
            O => Key::Unicode('o'),
            P => Key::Unicode('p'),
            Q => Key::Unicode('q'),
            R => Key::Unicode('r'),
            S => Key::Unicode('s'),
            T => Key::Unicode('t'),
            U => Key::Unicode('u'),
            V => Key::Unicode('v'),
            W => Key::Unicode('w'),
            X => Key::Unicode('x'),
            Y => Key::Unicode('y'),
            Z => Key::Unicode('z'),
            Num0 => Key::Unicode('0'),
            Num1 => Key::Unicode('1'),
            Num2 => Key::Unicode('2'),
            Num3 => Key::Unicode('3'),
            Num4 => Key::Unicode('4'),
            Num5 => Key::Unicode('5'),
            Num6 => Key::Unicode('6'),
            Num7 => Key::Unicode('7'),
            Num8 => Key::Unicode('8'),
            Num9 => Key::Unicode('9'),
            F1 => Key::F1,
            F2 => Key::F2,
            F3 => Key::F3,
            F4 => Key::F4,
            F5 => Key::F5,
            F6 => Key::F6,
            F7 => Key::F7,
            F8 => Key::F8,
            F9 => Key::F9,
            F10 => Key::F10,
            F11 => Key::F11,
            F12 => Key::F12,
            Space => Key::Space,
            Enter => Key::Return,
            Tab => Key::Tab,
            Backspace => Key::Backspace,
            Escape => Key::Escape,
            Shift => Key::Shift,
            Ctrl => Key::Control,
            Alt => Key::Alt,
            UpArrow => Key::UpArrow,
            DownArrow => Key::DownArrow,
            LeftArrow => Key::LeftArrow,
            RightArrow => Key::RightArrow,
            _ => Key::Space,
        }
    }

    pub fn send_direction(&self, enigo: &mut Enigo, dir: Direction) -> bool {
        match self {
            Self::MouseLeft => enigo.button(Button::Left, dir).is_ok(),
            Self::MouseRight => enigo.button(Button::Right, dir).is_ok(),
            Self::MouseMiddle => enigo.button(Button::Middle, dir).is_ok(),
            _ => enigo.key(self.to_enigo_key(), dir).is_ok(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressMode {
    Click,
    PressOnly,
    ReleaseOnly,
}

impl PressMode {
    pub fn label(&self) -> &'static str {
        match self {
            PressMode::Click => "🖱 Tap / Click",
            PressMode::PressOnly => "⬇ Hold Down",
            PressMode::ReleaseOnly => "⬆ Release",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModifiers {
    #[serde(default)]
    pub ctrl: bool,
    #[serde(default)]
    pub shift: bool,
    #[serde(default)]
    pub alt: bool,
    #[serde(default)]
    pub win: bool,
}

impl KeyModifiers {
    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.shift && !self.alt && !self.win
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequenceStep {
    pub button: ActionButton,
    #[serde(default)]
    pub modifiers: KeyModifiers,
    #[serde(default)]
    pub extra_keys: Vec<ActionButton>,
    pub mode: PressMode,
    pub hold_ms: u64,
    pub delay_after_ms: u64,
    pub enabled: bool,
}

impl SequenceStep {
    pub fn display_label(&self) -> String {
        let mut parts = Vec::new();
        if self.modifiers.ctrl {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.shift {
            parts.push("Shift".to_string());
        }
        if self.modifiers.alt {
            parts.push("Alt".to_string());
        }
        if self.modifiers.win {
            parts.push("Win".to_string());
        }
        parts.push(self.button.short_name().to_string());
        for extra in &self.extra_keys {
            parts.push(extra.short_name().to_string());
        }
        parts.join(" + ")
    }

    fn execute(&self, enigo: &mut Enigo, held: &mut HeldInputs, stop: &AtomicBool) -> bool {
        let mut chord: Vec<SyntheticInput> = Vec::new();
        if self.modifiers.ctrl { chord.push(SyntheticInput::Action(ActionButton::Ctrl)); }
        if self.modifiers.shift { chord.push(SyntheticInput::Action(ActionButton::Shift)); }
        if self.modifiers.alt { chord.push(SyntheticInput::Action(ActionButton::Alt)); }
        if self.modifiers.win { chord.push(SyntheticInput::Meta); }
        for button in &self.extra_keys { chord.push(SyntheticInput::Action(*button)); }
        let main = SyntheticInput::Action(self.button);
        match self.mode {
            PressMode::Click => {
                for &input in &chord {
                    if stop.load(Ordering::Acquire) || !held.press(enigo, input) { return false; }
                }
                if !chord.is_empty() && !sleep_cancellable(15, stop) { return false; }
                if stop.load(Ordering::Acquire) || !held.press(enigo, main) { return false; }
                if !sleep_cancellable(self.hold_ms.max(20), stop) { return false; }
                if !held.release(enigo, main) { return false; }
                if !chord.is_empty() && !sleep_cancellable(15, stop) { return false; }
                for &input in chord.iter().rev() {
                    if !held.release(enigo, input) { return false; }
                }
            }
            PressMode::PressOnly => {
                for &input in &chord {
                    if stop.load(Ordering::Acquire) || !held.press(enigo, input) { return false; }
                }
                if stop.load(Ordering::Acquire) || !held.press(enigo, main) { return false; }
            }
            PressMode::ReleaseOnly => {
                if !held.release(enigo, main) { return false; }
                for &input in chord.iter().rev() {
                    if !held.release(enigo, input) { return false; }
                }
            }
        }
        !stop.load(Ordering::Acquire)

    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SyntheticInput { Action(ActionButton), Meta }
impl SyntheticInput {
    fn send(self, enigo: &mut Enigo, direction: Direction) -> bool {
        match self {
            Self::Action(button) => button.send_direction(enigo, direction),
            Self::Meta => enigo.key(Key::Meta, direction).is_ok(),
        }
    }
}

#[derive(Default)]
struct HeldInputs { pressed: Vec<SyntheticInput> }
impl HeldInputs {
    fn press(&mut self, enigo: &mut Enigo, input: SyntheticInput) -> bool {
        if self.pressed.contains(&input) { return true; }
        if !input.send(enigo, Direction::Press) { return false; }
        self.pressed.push(input);
        true
    }
    fn release(&mut self, enigo: &mut Enigo, input: SyntheticInput) -> bool {
        if let Some(pos) = self.pressed.iter().position(|&value| value == input) {
            if !input.send(enigo, Direction::Release) { return false; }
            self.pressed.remove(pos);
        }
        true
    }
    fn release_all(&mut self, enigo: &mut Enigo) -> bool {
        let mut success = true;
        while let Some(input) = self.pressed.pop() {
            success &= input.send(enigo, Direction::Release);
        }
        success
    }
}

fn sleep_cancellable(mut millis: u64, stop: &AtomicBool) -> bool {
    while millis > 0 {
        if stop.load(Ordering::Acquire) { return false; }
        let slice = millis.min(25);
        thread::sleep(Duration::from_millis(slice));
        millis -= slice;
    }
    !stop.load(Ordering::Acquire)
}

const MAX_PRESET_BYTES: u64 = 1024 * 1024;
const MAX_PRESET_STEPS: usize = 1000;
fn validate_steps(steps: &[SequenceStep]) -> Result<(), &'static str> {
    if steps.len() > MAX_PRESET_STEPS { return Err("preset exceeds 1000 steps"); }
    for step in steps {
        if step.hold_ms > 60_000 || step.delay_after_ms > 300_000 {
            return Err("preset duration exceeds UI limits");
        }
        if step.extra_keys.len() > 8 { return Err("too many companion keys"); }
    }
    Ok(())
}
fn load_preset_file(path: &std::path::Path) -> Result<Vec<SequenceStep>, String> {
    let file = fs::File::open(path).map_err(|err| format!("Failed to open preset: {err}"))?;
    let mut data = String::new();
    file.take(MAX_PRESET_BYTES + 1).read_to_string(&mut data)
        .map_err(|err| format!("Failed to read preset: {err}"))?;
    if data.len() as u64 > MAX_PRESET_BYTES {
        return Err("Preset exceeds 1 MiB".into());
    }
    let steps: Vec<SequenceStep> = serde_json::from_str(&data)
        .map_err(|err| format!("Failed to parse JSON preset: {err}"))?;
    validate_steps(&steps).map_err(str::to_owned)?;
    Ok(steps)
}

pub struct KeySequencerApp {
    steps: Vec<SequenceStep>,
    selected_button: ActionButton,
    selected_modifiers: KeyModifiers,
    selected_extra_keys: Vec<ActionButton>,
    selected_mode: PressMode,
    new_hold_ms: u64,
    new_delay_ms: u64,

    start_countdown_sec: u64,
    repeat_count: u32,
    infinite_loop: bool,

    is_running: Arc<AtomicBool>,
    stop_requested: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
    status_text: Arc<Mutex<String>>,
    current_step_idx: Arc<Mutex<Option<usize>>>,

    theme_applied: bool,
}

impl Default for KeySequencerApp {
    fn default() -> Self {
        Self {
            steps: vec![
                SequenceStep {
                    button: ActionButton::Enter,
                    modifiers: KeyModifiers {
                        ctrl: true,
                        shift: false,
                        alt: false,
                        win: false,
                    },
                    extra_keys: Vec::new(),
                    mode: PressMode::Click,
                    hold_ms: 50,
                    delay_after_ms: 300,
                    enabled: true,
                },
                SequenceStep {
                    button: ActionButton::Space,
                    modifiers: KeyModifiers::default(),
                    extra_keys: Vec::new(),
                    mode: PressMode::Click,
                    hold_ms: 50,
                    delay_after_ms: 400,
                    enabled: true,
                },
                SequenceStep {
                    button: ActionButton::MouseLeft,
                    modifiers: KeyModifiers::default(),
                    extra_keys: Vec::new(),
                    mode: PressMode::Click,
                    hold_ms: 50,
                    delay_after_ms: 250,
                    enabled: true,
                },
            ],
            selected_button: ActionButton::Enter,
            selected_modifiers: KeyModifiers {
                ctrl: true,
                shift: false,
                alt: false,
                win: false,
            },
            selected_extra_keys: Vec::new(),
            selected_mode: PressMode::Click,
            new_hold_ms: 50,
            new_delay_ms: 250,
            start_countdown_sec: 3,
            repeat_count: 1,
            infinite_loop: false,
            is_running: Arc::new(AtomicBool::new(false)),
            stop_requested: Arc::new(AtomicBool::new(false)),
            worker: None,
            status_text: Arc::new(Mutex::new(
                "Ready. Press START SEQUENCE to begin.".to_string(),
            )),
            current_step_idx: Arc::new(Mutex::new(None)),
            theme_applied: false,
        }
    }
}

impl Drop for KeySequencerApp {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() { let _ = worker.join(); }
    }
}

impl KeySequencerApp {
    fn total_duration_ms(&self) -> u64 {
        self.steps
            .iter()
            .filter(|s| s.enabled)
            .fold(0u64, |total, s| {
                let hold = if s.mode == PressMode::Click {
                    let chord_delay = if !s.modifiers.is_empty() || !s.extra_keys.is_empty() {
                        30
                    } else {
                        0
                    };
                    s.hold_ms.max(20).saturating_add(chord_delay)
                } else {
                    0
                };
                total.saturating_add(hold).saturating_add(s.delay_after_ms)
            })
    }

    fn enabled_count(&self) -> usize {
        self.steps.iter().filter(|s| s.enabled).count()
    }

    fn load_preset(&mut self, preset: &str) {
        match preset {
            "ctrl_enter" => {
                self.steps = vec![SequenceStep {
                    button: ActionButton::Enter,
                    modifiers: KeyModifiers {
                        ctrl: true,
                        shift: false,
                        alt: false,
                        win: false,
                    },
                    extra_keys: Vec::new(),
                    mode: PressMode::Click,
                    hold_ms: 50,
                    delay_after_ms: 250,
                    enabled: true,
                }];
                *self.status_text.lock().unwrap() =
                    "Loaded 'Ctrl + Enter' shortcut preset.".to_string();
            }
            "ctrl_c_ctrl_v" => {
                self.steps = vec![
                    SequenceStep {
                        button: ActionButton::C,
                        modifiers: KeyModifiers {
                            ctrl: true,
                            shift: false,
                            alt: false,
                            win: false,
                        },
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 40,
                        delay_after_ms: 150,
                        enabled: true,
                    },
                    SequenceStep {
                        button: ActionButton::V,
                        modifiers: KeyModifiers {
                            ctrl: true,
                            shift: false,
                            alt: false,
                            win: false,
                        },
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 40,
                        delay_after_ms: 250,
                        enabled: true,
                    },
                ];
                *self.status_text.lock().unwrap() =
                    "Loaded 'Ctrl+C / Ctrl+V' Copy-Paste preset.".to_string();
            }
            "alt_tab" => {
                self.steps = vec![SequenceStep {
                    button: ActionButton::Tab,
                    modifiers: KeyModifiers {
                        ctrl: false,
                        shift: false,
                        alt: true,
                        win: false,
                    },
                    extra_keys: Vec::new(),
                    mode: PressMode::Click,
                    hold_ms: 60,
                    delay_after_ms: 500,
                    enabled: true,
                }];
                *self.status_text.lock().unwrap() =
                    "Loaded 'Alt + Tab' window switch preset.".to_string();
            }
            "rapid_fire" => {
                self.steps = vec![SequenceStep {
                    button: ActionButton::MouseLeft,
                    modifiers: KeyModifiers::default(),
                    extra_keys: Vec::new(),
                    mode: PressMode::Click,
                    hold_ms: 25,
                    delay_after_ms: 40,
                    enabled: true,
                }];
                *self.status_text.lock().unwrap() = "Loaded 'Rapid Fire Click' preset.".to_string();
            }
            "wasd_loop" => {
                self.steps = vec![
                    SequenceStep {
                        button: ActionButton::W,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 300,
                        delay_after_ms: 50,
                        enabled: true,
                    },
                    SequenceStep {
                        button: ActionButton::D,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 300,
                        delay_after_ms: 50,
                        enabled: true,
                    },
                    SequenceStep {
                        button: ActionButton::S,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 300,
                        delay_after_ms: 50,
                        enabled: true,
                    },
                    SequenceStep {
                        button: ActionButton::A,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 300,
                        delay_after_ms: 50,
                        enabled: true,
                    },
                ];
                *self.status_text.lock().unwrap() =
                    "Loaded 'WASD Movement Loop' preset.".to_string();
            }
            "jump_attack" => {
                self.steps = vec![
                    SequenceStep {
                        button: ActionButton::Space,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 50,
                        delay_after_ms: 150,
                        enabled: true,
                    },
                    SequenceStep {
                        button: ActionButton::MouseLeft,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::Click,
                        hold_ms: 50,
                        delay_after_ms: 250,
                        enabled: true,
                    },
                ];
                *self.status_text.lock().unwrap() = "Loaded 'Jump & Attack' preset.".to_string();
            }
            "hold_right_click" => {
                self.steps = vec![
                    SequenceStep {
                        button: ActionButton::MouseRight,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::PressOnly,
                        hold_ms: 0,
                        delay_after_ms: 2000,
                        enabled: true,
                    },
                    SequenceStep {
                        button: ActionButton::MouseRight,
                        modifiers: KeyModifiers::default(),
                        extra_keys: Vec::new(),
                        mode: PressMode::ReleaseOnly,
                        hold_ms: 0,
                        delay_after_ms: 500,
                        enabled: true,
                    },
                ];
                *self.status_text.lock().unwrap() =
                    "Loaded 'Hold Right Click (2s)' preset.".to_string();
            }
            _ => {}
        }
    }

    fn start_sequence(&mut self) {
        if self.is_running.load(Ordering::Acquire) { return; }
        if let Some(worker) = self.worker.take() {
            if worker.join().is_err() {
                *self.status_text.lock().unwrap() = "Failed: prior macro worker panicked".into();
                return;
            }
        }
        if let Err(reason) = validate_steps(&self.steps) {
            *self.status_text.lock().unwrap() = format!("Failed: {reason}");
            return;
        }
        let active_steps: Vec<(usize, SequenceStep)> = self.steps.iter().cloned()
            .enumerate().filter(|(_, step)| step.enabled).collect();
        if active_steps.is_empty() {
            *self.status_text.lock().unwrap() = "No enabled steps to run! Enable at least one step.".into();
            return;
        }
        self.stop_requested.store(false, Ordering::Release);
        self.is_running.store(true, Ordering::Release);
        let stop = Arc::clone(&self.stop_requested);
        let running = Arc::clone(&self.is_running);
        let status = Arc::clone(&self.status_text);
        let current = Arc::clone(&self.current_step_idx);
        let countdown = self.start_countdown_sec;
        let repeat_count = self.repeat_count;
        let infinite = self.infinite_loop;
        self.worker = Some(thread::spawn(move || {
            for sec in (1..=countdown).rev() {
                if stop.load(Ordering::Acquire) { break; }
                *status.lock().unwrap() = format!("Starting in {}s... (Switch to your target window)", sec);
                if !sleep_cancellable(1000, &stop) { break; }
            }
            let mut failed = false;
            let mut held = HeldInputs::default();
            if !stop.load(Ordering::Acquire) {
                match Enigo::new(&Settings::default()) {
                    Ok(mut enigo) => {
                        let mut cycle = 0u32;
                        'macro_loop: while !stop.load(Ordering::Acquire) && (infinite || cycle < repeat_count) {
                            cycle = cycle.saturating_add(1);
                            for (idx, step) in &active_steps {
                                if stop.load(Ordering::Acquire) { break 'macro_loop; }
                                *current.lock().unwrap() = Some(*idx);
                                let loop_name = if infinite { format!("Cycle {} (Infinite)", cycle) }
                                    else { format!("Cycle {}/{}", cycle, repeat_count) };
                                *status.lock().unwrap() = format!("{} • Step #{}: {} ({})",
                                    loop_name, idx + 1, step.display_label(), step.mode.label());
                                if !step.execute(&mut enigo, &mut held, &stop) {
                                    failed = !stop.load(Ordering::Acquire);
                                    break 'macro_loop;
                                }
                                if !sleep_cancellable(step.delay_after_ms, &stop) { break 'macro_loop; }
                            }
                        }
                        if !held.release_all(&mut enigo) {
                            failed = true;
                            *status.lock().unwrap() = "Failed: could not release all synthetic keys".into();
                        }
                    }
                    Err(err) => {
                        failed = true;
                        *status.lock().unwrap() = format!("Failed to initialize input engine: {err:?}");
                    }
                }
            }
            *current.lock().unwrap() = None;
            if !failed {
                *status.lock().unwrap() = if stop.load(Ordering::Acquire) {
                    "Sequence stopped by user.".into()
                } else {
                    "Sequence completed successfully.".into()
                };
            }
            // Disable Start until the previous worker has completed cleanup.
            running.store(false, Ordering::Release);
        }));
    }

    fn stop_sequence(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
    }

}

fn modifier_toggle(ui: &mut egui::Ui, state: &mut bool, text: &str) -> egui::Response {
    let active = *state;
    let (fill, stroke, text_color) = if active {
        (
            egui::Color32::from_rgb(37, 99, 235),
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(96, 165, 250)),
            egui::Color32::WHITE,
        )
    } else {
        (
            egui::Color32::from_rgb(30, 36, 50),
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(50, 60, 85)),
            egui::Color32::from_rgb(148, 163, 184),
        )
    };

    let label = if active {
        format!("✓ {}", text)
    } else {
        text.to_string()
    };

    let btn = egui::Button::new(
        egui::RichText::new(label)
            .strong()
            .small()
            .color(text_color),
    )
    .fill(fill)
    .stroke(stroke)
    .rounding(egui::Rounding::same(4.0))
    .min_size(egui::vec2(50.0, 22.0));

    let resp = ui.add(btn);
    if resp.clicked() {
        *state = !*state;
    }
    resp
}

fn show_button_selector(
    ui: &mut egui::Ui,
    selected: &mut ActionButton,
    id_salt: impl std::hash::Hash,
) {
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(selected.label())
        .width(165.0)
        .show_ui(ui, |ui| {
            ui.label(
                egui::RichText::new("── Mouse Buttons ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[
                ActionButton::MouseLeft,
                ActionButton::MouseRight,
                ActionButton::MouseMiddle,
            ] {
                ui.selectable_value(selected, *btn, btn.label());
            }

            ui.separator();
            ui.label(
                egui::RichText::new("── Common Keys ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[
                ActionButton::Space,
                ActionButton::Enter,
                ActionButton::Tab,
                ActionButton::Backspace,
                ActionButton::Escape,
            ] {
                ui.selectable_value(selected, *btn, btn.label());
            }

            ui.separator();
            ui.label(
                egui::RichText::new("── Modifiers ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[ActionButton::Shift, ActionButton::Ctrl, ActionButton::Alt] {
                ui.selectable_value(selected, *btn, btn.label());
            }

            ui.separator();
            ui.label(
                egui::RichText::new("── Navigation ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[
                ActionButton::UpArrow,
                ActionButton::DownArrow,
                ActionButton::LeftArrow,
                ActionButton::RightArrow,
            ] {
                ui.selectable_value(selected, *btn, btn.label());
            }

            ui.separator();
            ui.label(
                egui::RichText::new("── Numbers ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[
                ActionButton::Num0,
                ActionButton::Num1,
                ActionButton::Num2,
                ActionButton::Num3,
                ActionButton::Num4,
                ActionButton::Num5,
                ActionButton::Num6,
                ActionButton::Num7,
                ActionButton::Num8,
                ActionButton::Num9,
            ] {
                ui.selectable_value(selected, *btn, btn.label());
            }

            ui.separator();
            ui.label(
                egui::RichText::new("── Letters (A-Z) ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[
                ActionButton::A,
                ActionButton::B,
                ActionButton::C,
                ActionButton::D,
                ActionButton::E,
                ActionButton::F,
                ActionButton::G,
                ActionButton::H,
                ActionButton::I,
                ActionButton::J,
                ActionButton::K,
                ActionButton::L,
                ActionButton::M,
                ActionButton::N,
                ActionButton::O,
                ActionButton::P,
                ActionButton::Q,
                ActionButton::R,
                ActionButton::S,
                ActionButton::T,
                ActionButton::U,
                ActionButton::V,
                ActionButton::W,
                ActionButton::X,
                ActionButton::Y,
                ActionButton::Z,
            ] {
                ui.selectable_value(selected, *btn, btn.label());
            }

            ui.separator();
            ui.label(
                egui::RichText::new("── Function Keys ──")
                    .color(egui::Color32::from_rgb(96, 165, 250))
                    .small(),
            );
            for btn in &[
                ActionButton::F1,
                ActionButton::F2,
                ActionButton::F3,
                ActionButton::F4,
                ActionButton::F5,
                ActionButton::F6,
                ActionButton::F7,
                ActionButton::F8,
                ActionButton::F9,
                ActionButton::F10,
                ActionButton::F11,
                ActionButton::F12,
            ] {
                ui.selectable_value(selected, *btn, btn.label());
            }
        });
}

fn show_mode_selector(ui: &mut egui::Ui, selected: &mut PressMode, id_salt: impl std::hash::Hash) {
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(selected.label())
        .width(135.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(selected, PressMode::Click, PressMode::Click.label());
            ui.selectable_value(selected, PressMode::PressOnly, PressMode::PressOnly.label());
            ui.selectable_value(
                selected,
                PressMode::ReleaseOnly,
                PressMode::ReleaseOnly.label(),
            );
        });
}

fn setup_custom_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    let bg_dark = egui::Color32::from_rgb(14, 17, 23);
    let bg_panel = egui::Color32::from_rgb(18, 22, 31);
    let bg_card = egui::Color32::from_rgb(26, 32, 45);
    let border_subtle = egui::Color32::from_rgb(42, 51, 72);
    let accent_blue = egui::Color32::from_rgb(59, 130, 246);

    visuals.panel_fill = bg_panel;
    visuals.window_fill = bg_dark;
    visuals.extreme_bg_color = egui::Color32::from_rgb(10, 12, 17);

    visuals.widgets.noninteractive.bg_fill = bg_card;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, border_subtle);
    visuals.widgets.noninteractive.fg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(160, 174, 192));
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);

    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 37, 52);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, border_subtle);
    visuals.widgets.inactive.fg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(226, 232, 240));
    visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);

    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(42, 53, 76);
    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(96, 165, 250));
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);
    visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);

    visuals.widgets.active.bg_fill = accent_blue;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);
    visuals.widgets.active.rounding = egui::Rounding::same(6.0);

    visuals.selection.bg_fill = accent_blue;
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::WHITE);
    visuals.window_rounding = egui::Rounding::same(8.0);

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    ctx.set_style(style);
}

impl eframe::App for KeySequencerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.theme_applied {
            setup_custom_theme(ctx);
            self.theme_applied = true;
        }

        let running = self.is_running.load(Ordering::SeqCst);
        if running {
            ctx.request_repaint_after(Duration::from_millis(50));
        }

        // Header Panel
        egui::TopBottomPanel::top("header_panel")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(16, 20, 28))
                    .inner_margin(egui::Margin::symmetric(14.0, 9.0))
                    .stroke(egui::Stroke::new(
                        1.0_f32,
                        egui::Color32::from_rgb(38, 45, 62),
                    )),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let logo_resp = ui.add(
                        egui::Image::new(egui::include_image!("../assets/keytape-icon.svg"))
                            .fit_to_exact_size(egui::vec2(28.0, 28.0))
                            .rounding(egui::Rounding::same(6.0)),
                    );
                    logo_resp.on_hover_ui(|ui| {
                        ui.add(
                            egui::Image::new(egui::include_image!(
                                "../assets/keytape-logo-dark.svg"
                            ))
                            .fit_to_exact_size(egui::vec2(173.5, 54.0)),
                        );
                        ui.label(
                            egui::RichText::new(
                                "Keytape — High-performance Rust Key & Mouse Sequencer",
                            )
                            .small()
                            .color(egui::Color32::from_rgb(148, 163, 184)),
                        );
                    });
                    ui.add_space(4.0);

                    ui.label(
                        egui::RichText::new("KEYTAPE")
                            .size(17.0)
                            .strong()
                            .color(egui::Color32::from_rgb(244, 241, 233)),
                    );
                    ui.label(
                        egui::RichText::new("⚡ KEY & MOUSE SEQUENCER")
                            .size(15.0)
                            .strong()
                            .color(egui::Color32::from_rgb(96, 165, 250)),
                    );
                    ui.label(
                        egui::RichText::new("v1.1 (Combinations)")
                            .small()
                            .color(egui::Color32::from_rgb(100, 116, 139)),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Clear button
                        if ui
                            .add_enabled(
                                !running && !self.steps.is_empty(),
                                egui::Button::new("🧹 Clear All"),
                            )
                            .on_hover_text("Clear all steps from queue")
                            .clicked()
                        {
                            self.steps.clear();
                            *self.status_text.lock().unwrap() =
                                "Cleared sequence queue.".to_string();
                        }

                        // Save button
                        if ui
                            .button("💾 Save Preset (.json)")
                            .on_hover_text("Save sequence to a JSON file")
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JSON Preset", &["json"])
                                .set_file_name("sequence.json")
                                .save_file()
                            {
                                match serde_json::to_string_pretty(&self.steps) {
                                    Ok(json) => match fs::write(&path, json) {
                                        Ok(_) => {
                                            *self.status_text.lock().unwrap() = format!(
                                                "Saved {} steps to {}",
                                                self.steps.len(),
                                                path.file_name()
                                                    .unwrap_or_default()
                                                    .to_string_lossy()
                                            );
                                        }
                                        Err(err) => {
                                            *self.status_text.lock().unwrap() =
                                                format!("Failed to save file: {}", err);
                                        }
                                    },
                                    Err(err) => {
                                        *self.status_text.lock().unwrap() =
                                            format!("Failed to serialize preset: {}", err);
                                    }
                                }
                            }
                        }

                        // Load button
                        if ui
                            .add_enabled(!running, egui::Button::new("📂 Load Preset (.json)"))
                            .on_hover_text("Load sequence from a JSON file")
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JSON Preset", &["json"])
                                .pick_file()
                            {
                                match load_preset_file(&path) {
                                    Ok(loaded) => {
                                        self.steps = loaded;
                                        *self.status_text.lock().unwrap() = format!("Loaded {} steps from {}",
                                            self.steps.len(), path.file_name().unwrap_or_default().to_string_lossy());
                                    }
                                    Err(err) => {
                                        *self.status_text.lock().unwrap() = err;
                                    }
                                }
                            }
                        }

                        // Presets menu
                        ui.add_enabled_ui(!running, |ui| {
                            egui::ComboBox::from_id_salt("quick_presets_menu")
                                .selected_text("✨ Built-in Presets")
                                .width(160.0)
                                .show_ui(ui, |ui| {
                                    if ui
                                        .selectable_label(false, "⌨ Ctrl + Enter (Send / Submit)")
                                        .clicked()
                                    {
                                        self.load_preset("ctrl_enter");
                                    }
                                    if ui
                                        .selectable_label(
                                            false,
                                            "📋 Ctrl+C / Ctrl+V (Copy & Paste)",
                                        )
                                        .clicked()
                                    {
                                        self.load_preset("ctrl_c_ctrl_v");
                                    }
                                    if ui
                                        .selectable_label(false, "🔀 Alt + Tab (Switch Window)")
                                        .clicked()
                                    {
                                        self.load_preset("alt_tab");
                                    }
                                    if ui
                                        .selectable_label(false, "⚡ Rapid Fire (Left Click Spam)")
                                        .clicked()
                                    {
                                        self.load_preset("rapid_fire");
                                    }
                                    if ui
                                        .selectable_label(false, "🏃 WASD Movement Loop")
                                        .clicked()
                                    {
                                        self.load_preset("wasd_loop");
                                    }
                                    if ui
                                        .selectable_label(false, "⚔ Jump & Attack Macro")
                                        .clicked()
                                    {
                                        self.load_preset("jump_attack");
                                    }
                                    if ui
                                        .selectable_label(false, "🖱 Hold Right Click (2s)")
                                        .clicked()
                                    {
                                        self.load_preset("hold_right_click");
                                    }
                                });
                        });
                    });
                });
            });

        // Bottom Control Panel
        egui::TopBottomPanel::bottom("control_panel")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(16, 20, 28))
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .stroke(egui::Stroke::new(
                        1.0_f32,
                        egui::Color32::from_rgb(38, 45, 62),
                    )),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⏱ Countdown:").strong());
                    ui.add_enabled(
                        !running,
                        egui::DragValue::new(&mut self.start_countdown_sec)
                            .range(0..=30)
                            .suffix(" s"),
                    );

                    ui.separator();

                    ui.checkbox(&mut self.infinite_loop, "🔁 Infinite Loop");
                    if !self.infinite_loop {
                        ui.label(egui::RichText::new("Repeat Cycles:").strong());
                        ui.add_enabled(
                            !running,
                            egui::DragValue::new(&mut self.repeat_count).range(1..=10000),
                        );
                    }

                    ui.separator();
                    let est_loop_ms = self.total_duration_ms();
                    ui.label(
                        egui::RichText::new(format!("⏱ Loop Time: {} ms", est_loop_ms))
                            .small()
                            .color(egui::Color32::from_rgb(148, 163, 184)),
                    );

                    // Right side START / STOP button
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if running {
                            let stop_btn = egui::Button::new(
                                egui::RichText::new("⏹  STOP SEQUENCE")
                                    .size(15.0)
                                    .strong()
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(egui::Color32::from_rgb(220, 38, 38))
                            .min_size(egui::vec2(170.0, 36.0))
                            .rounding(egui::Rounding::same(8.0));
                            if ui.add(stop_btn).clicked() {
                                self.stop_sequence();
                            }
                        } else {
                            let start_btn = egui::Button::new(
                                egui::RichText::new("▶  START SEQUENCE")
                                    .size(15.0)
                                    .strong()
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(egui::Color32::from_rgb(22, 163, 74))
                            .min_size(egui::vec2(170.0, 36.0))
                            .rounding(egui::Rounding::same(8.0));
                            if ui.add(start_btn).clicked() {
                                self.start_sequence();
                            }
                        }
                    });
                });

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(2.0);

                // Status row
                let status = self.status_text.lock().unwrap().clone();
                ui.horizontal(|ui| {
                    let (dot_color, status_icon) = if running {
                        if status.starts_with("Starting") {
                            (egui::Color32::from_rgb(234, 179, 8), "⏱")
                        } else {
                            (egui::Color32::from_rgb(34, 197, 94), "⚡")
                        }
                    } else if status.starts_with("Failed") || status.starts_with("Sequence stopped")
                    {
                        (egui::Color32::from_rgb(239, 68, 68), "🛑")
                    } else if status.starts_with("Sequence completed") {
                        (egui::Color32::from_rgb(34, 197, 94), "✓")
                    } else {
                        (egui::Color32::from_rgb(148, 163, 184), "●")
                    };

                    ui.label(egui::RichText::new(status_icon).color(dot_color).strong());
                    ui.strong("Status:");
                    ui.label(
                        egui::RichText::new(status).color(egui::Color32::from_rgb(226, 232, 240)),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("💡 Tip: Switch to target window during countdown")
                                .small()
                                .color(egui::Color32::from_rgb(100, 116, 139)),
                        );
                    });
                });
            });

        // Main Content Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Section 1: Add a new step
            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgb(22, 27, 38))
                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(42, 50, 71)))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("1. Add Key or Mouse Action (Single or Combination)")
                                .strong()
                                .color(egui::Color32::from_rgb(226, 232, 240)),
                        );
                        ui.label(
                            egui::RichText::new(
                                "• Combine with Ctrl, Shift, Alt, Win or extra keys",
                            )
                            .small()
                            .color(egui::Color32::from_rgb(100, 116, 139)),
                        );
                    });

                    ui.add_space(4.0);

                    // Row 1: Modifier selection + Extra keys + Result preview
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Modifiers:").strong());
                        ui.add_enabled_ui(!running, |ui| {
                            modifier_toggle(ui, &mut self.selected_modifiers.ctrl, "Ctrl");
                            modifier_toggle(ui, &mut self.selected_modifiers.shift, "Shift");
                            modifier_toggle(ui, &mut self.selected_modifiers.alt, "Alt");
                            modifier_toggle(ui, &mut self.selected_modifiers.win, "Win");

                            // Extra key chips
                            let mut remove_extra = None;
                            for (e_idx, extra) in self.selected_extra_keys.iter().enumerate() {
                                let btn = egui::Button::new(
                                    egui::RichText::new(format!("+ {} ✕", extra.short_name()))
                                        .small()
                                        .color(egui::Color32::from_rgb(147, 197, 253)),
                                )
                                .fill(egui::Color32::from_rgb(30, 58, 138))
                                .rounding(egui::Rounding::same(4.0));
                                if ui.add(btn).on_hover_text("Remove extra key").clicked() {
                                    remove_extra = Some(e_idx);
                                }
                            }
                            if let Some(e_idx) = remove_extra {
                                self.selected_extra_keys.remove(e_idx);
                            }

                            // Extra key dropdown
                            egui::ComboBox::from_id_salt("add_extra_builder")
                                .selected_text("+ Key")
                                .width(70.0)
                                .show_ui(ui, |ui| {
                                    for btn in ActionButton::all() {
                                        if ui.selectable_label(false, btn.label()).clicked() {
                                            self.selected_extra_keys.push(*btn);
                                        }
                                    }
                                });
                        });

                        ui.add_space(8.0);
                        // Result preview
                        let mut preview_parts = Vec::new();
                        if self.selected_modifiers.ctrl {
                            preview_parts.push("Ctrl");
                        }
                        if self.selected_modifiers.shift {
                            preview_parts.push("Shift");
                        }
                        if self.selected_modifiers.alt {
                            preview_parts.push("Alt");
                        }
                        if self.selected_modifiers.win {
                            preview_parts.push("Win");
                        }
                        preview_parts.push(self.selected_button.short_name());
                        for extra in &self.selected_extra_keys {
                            preview_parts.push(extra.short_name());
                        }
                        let preview_str = preview_parts.join(" + ");

                        ui.label(
                            egui::RichText::new("Combo Preview:")
                                .small()
                                .color(egui::Color32::from_rgb(148, 163, 184)),
                        );
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(30, 41, 59))
                            .stroke(egui::Stroke::new(
                                1.0_f32,
                                egui::Color32::from_rgb(59, 130, 246),
                            ))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 3.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(&preview_str)
                                        .strong()
                                        .color(egui::Color32::from_rgb(191, 219, 254)),
                                );
                            });
                    });

                    ui.add_space(4.0);

                    // Row 2: Target Button, Action, Hold, Delay, Add Button
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new("Target:").strong());
                        show_button_selector(ui, &mut self.selected_button, "new_button_selector");

                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("Action:").strong());
                        show_mode_selector(ui, &mut self.selected_mode, "new_mode_selector");

                        ui.add_space(4.0);
                        if self.selected_mode == PressMode::Click {
                            ui.label(egui::RichText::new("Hold:").strong());
                            ui.add(
                                egui::DragValue::new(&mut self.new_hold_ms)
                                    .range(0..=60000)
                                    .suffix(" ms")
                                    .speed(5),
                            );
                            if ui.small_button("25ms").clicked() {
                                self.new_hold_ms = 25;
                            }
                            if ui.small_button("50ms").clicked() {
                                self.new_hold_ms = 50;
                            }
                            if ui.small_button("100ms").clicked() {
                                self.new_hold_ms = 100;
                            }
                        }

                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("Delay After:").strong());
                        ui.add(
                            egui::DragValue::new(&mut self.new_delay_ms)
                                .range(0..=300000)
                                .suffix(" ms")
                                .speed(10),
                        );
                        if ui.small_button("100ms").clicked() {
                            self.new_delay_ms = 100;
                        }
                        if ui.small_button("250ms").clicked() {
                            self.new_delay_ms = 250;
                        }
                        if ui.small_button("500ms").clicked() {
                            self.new_delay_ms = 500;
                        }

                        ui.add_space(6.0);
                        let add_btn = egui::Button::new(
                            egui::RichText::new("➕ Add to Queue")
                                .strong()
                                .color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(37, 99, 235))
                        .min_size(egui::vec2(120.0, 26.0))
                        .rounding(egui::Rounding::same(6.0));

                        if ui.add_enabled(!running, add_btn).clicked() {
                            self.steps.push(SequenceStep {
                                button: self.selected_button,
                                modifiers: self.selected_modifiers,
                                extra_keys: self.selected_extra_keys.clone(),
                                mode: self.selected_mode,
                                hold_ms: self.new_hold_ms,
                                delay_after_ms: self.new_delay_ms,
                                enabled: true,
                            });
                        }
                    });
                });

            ui.add_space(8.0);

            // Section 2: Ordered sequence queue
            ui.horizontal(|ui| {
                let enabled_cnt = self.enabled_count();
                let total_cnt = self.steps.len();
                ui.label(
                    egui::RichText::new(format!(
                        "2. Execution Queue ({}/{} enabled)",
                        enabled_cnt, total_cnt
                    ))
                    .strong()
                    .color(egui::Color32::from_rgb(226, 232, 240)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_enabled(
                            !running && !self.steps.is_empty(),
                            egui::Button::new("Deselect All").small(),
                        )
                        .clicked()
                    {
                        for s in &mut self.steps {
                            s.enabled = false;
                        }
                    }
                    if ui
                        .add_enabled(
                            !running && !self.steps.is_empty(),
                            egui::Button::new("Select All").small(),
                        )
                        .clicked()
                    {
                        for s in &mut self.steps {
                            s.enabled = true;
                        }
                    }
                });
            });
            ui.separator();

            let active_idx = *self.current_step_idx.lock().unwrap();
            let mut move_up: Option<usize> = None;
            let mut move_down: Option<usize> = None;
            let mut remove_idx: Option<usize> = None;
            let mut duplicate_idx: Option<usize> = None;
            let total_steps = self.steps.len();

            if self.steps.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(28.0);
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(20, 24, 34))
                        .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(38, 45, 62)))
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(24.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::Image::new(egui::include_image!(
                                    "../assets/keytape-logo-dark.svg"
                                ))
                                .fit_to_exact_size(egui::vec2(208.0, 65.0)),
                            );
                            ui.add_space(12.0);
                            ui.label(
                                egui::RichText::new("📋 No steps in sequence queue")
                                    .size(16.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(148, 163, 184)),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(
                                    "Add a key combination above, load a preset JSON file, or select a built-in preset.",
                                )
                                .color(egui::Color32::from_rgb(100, 116, 139)),
                            );
                            ui.add_space(10.0);
                            if ui
                                .button("⌨ Load Ctrl + Enter Shortcut Preset")
                                .clicked()
                            {
                                self.load_preset("ctrl_enter");
                            }
                        });
                });
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for (idx, step) in self.steps.iter_mut().enumerate() {
                            let is_active = active_idx == Some(idx);
                            let is_disabled = !step.enabled;

                            let (fill_color, stroke_color) = if is_active {
                                (
                                    egui::Color32::from_rgba_unmultiplied(34, 197, 94, 35),
                                    egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(34, 197, 94)),
                                )
                            } else if is_disabled {
                                (
                                    egui::Color32::from_rgba_unmultiplied(20, 24, 34, 120),
                                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(38, 44, 60)),
                                )
                            } else {
                                (
                                    egui::Color32::from_rgb(26, 31, 44),
                                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(45, 54, 76)),
                                )
                            };

                            egui::Frame::group(ui.style())
                                .fill(fill_color)
                                .stroke(stroke_color)
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                .show(ui, |ui| {
                                    // Row 1: Checkbox, Badge, Target, Mode, Hold, Wait, Action Buttons
                                    ui.horizontal(|ui| {
                                        // Checkbox
                                        ui.add_enabled(
                                            !running,
                                            egui::Checkbox::new(&mut step.enabled, ""),
                                        );

                                        // Step index badge
                                        let badge_bg = if is_active {
                                            egui::Color32::from_rgb(22, 101, 52)
                                        } else {
                                            egui::Color32::from_rgb(38, 45, 64)
                                        };
                                        let badge_text = egui::RichText::new(format!("#{}", idx + 1))
                                            .strong()
                                            .color(if is_active {
                                                egui::Color32::from_rgb(134, 239, 172)
                                            } else {
                                                egui::Color32::from_rgb(203, 213, 225)
                                            });

                                        egui::Frame::none()
                                            .fill(badge_bg)
                                            .rounding(egui::Rounding::same(4.0))
                                            .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                            .show(ui, |ui| {
                                                ui.label(badge_text);
                                            });

                                        if is_active {
                                            egui::Frame::none()
                                                .fill(egui::Color32::from_rgb(22, 101, 52))
                                                .rounding(egui::Rounding::same(4.0))
                                                .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        egui::RichText::new("● RUNNING")
                                                            .strong()
                                                            .small()
                                                            .color(egui::Color32::WHITE),
                                                    );
                                                });
                                        }

                                        // Combo Summary Badge
                                        let combo_summary = step.display_label();
                                        let is_combo = !step.modifiers.is_empty()
                                            || !step.extra_keys.is_empty();
                                        let (combo_bg, combo_stroke, combo_txt_color) = if is_active {
                                            (
                                                egui::Color32::from_rgb(22, 101, 52),
                                                egui::Stroke::new(
                                                    1.0_f32,
                                                    egui::Color32::from_rgb(34, 197, 94),
                                                ),
                                                egui::Color32::WHITE,
                                            )
                                        } else if is_combo {
                                            (
                                                egui::Color32::from_rgb(30, 58, 138),
                                                egui::Stroke::new(
                                                    1.0_f32,
                                                    egui::Color32::from_rgb(59, 130, 246),
                                                ),
                                                egui::Color32::from_rgb(191, 219, 254),
                                            )
                                        } else {
                                            (
                                                egui::Color32::from_rgb(30, 37, 52),
                                                egui::Stroke::new(
                                                    1.0_f32,
                                                    egui::Color32::from_rgb(45, 55, 78),
                                                ),
                                                egui::Color32::from_rgb(226, 232, 240),
                                            )
                                        };

                                        egui::Frame::none()
                                            .fill(combo_bg)
                                            .stroke(combo_stroke)
                                            .rounding(egui::Rounding::same(4.0))
                                            .inner_margin(egui::Margin::symmetric(8.0, 3.0))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    egui::RichText::new(&combo_summary)
                                                        .strong()
                                                        .color(combo_txt_color),
                                                );
                                            });

                                        // Button selector
                                        ui.add_enabled_ui(!running, |ui| {
                                            show_button_selector(
                                                ui,
                                                &mut step.button,
                                                format!("step_btn_{}", idx),
                                            );
                                        });

                                        // Mode selector
                                        ui.add_enabled_ui(!running, |ui| {
                                            show_mode_selector(
                                                ui,
                                                &mut step.mode,
                                                format!("step_mode_{}", idx),
                                            );
                                        });

                                        // Timing controls
                                        if step.mode == PressMode::Click {
                                            ui.label(
                                                egui::RichText::new("Hold:")
                                                    .small()
                                                    .color(egui::Color32::from_rgb(148, 163, 184)),
                                            );
                                            ui.add_enabled(
                                                !running,
                                                egui::DragValue::new(&mut step.hold_ms)
                                                    .range(0..=60000)
                                                    .suffix(" ms")
                                                    .speed(5),
                                            );
                                        } else {
                                            ui.label(
                                                egui::RichText::new("(No hold)")
                                                    .small()
                                                    .color(egui::Color32::from_rgb(100, 116, 139)),
                                            );
                                        }

                                        ui.label(
                                            egui::RichText::new("Wait:")
                                                .small()
                                                .color(egui::Color32::from_rgb(148, 163, 184)),
                                        );
                                        ui.add_enabled(
                                            !running,
                                            egui::DragValue::new(&mut step.delay_after_ms)
                                                .range(0..=300000)
                                                .suffix(" ms")
                                                .speed(10),
                                        );

                                        // Reorder and action buttons
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                // Delete button
                                                let del_btn = egui::Button::new(
                                                    egui::RichText::new("🗑").color(
                                                        egui::Color32::from_rgb(248, 113, 113),
                                                    ),
                                                )
                                                .min_size(egui::vec2(26.0, 24.0));
                                                if ui
                                                    .add_enabled(!running, del_btn)
                                                    .on_hover_text("Delete step")
                                                    .clicked()
                                                {
                                                    remove_idx = Some(idx);
                                                }

                                                // Copy button
                                                let copy_btn = egui::Button::new(
                                                    egui::RichText::new("📋").color(
                                                        egui::Color32::from_rgb(148, 163, 184),
                                                    ),
                                                )
                                                .min_size(egui::vec2(26.0, 24.0));
                                                if ui
                                                    .add_enabled(!running, copy_btn)
                                                    .on_hover_text("Duplicate step")
                                                    .clicked()
                                                {
                                                    duplicate_idx = Some(idx);
                                                }

                                                // Down button
                                                let down_btn = egui::Button::new("▼")
                                                    .min_size(egui::vec2(26.0, 24.0));
                                                if ui
                                                    .add_enabled(
                                                        !running && idx + 1 < total_steps,
                                                        down_btn,
                                                    )
                                                    .on_hover_text("Move step down")
                                                    .clicked()
                                                {
                                                    move_down = Some(idx);
                                                }

                                                // Up button
                                                let up_btn = egui::Button::new("▲")
                                                    .min_size(egui::vec2(26.0, 24.0));
                                                if ui
                                                    .add_enabled(!running && idx > 0, up_btn)
                                                    .on_hover_text("Move step up")
                                                    .clicked()
                                                {
                                                    move_up = Some(idx);
                                                }
                                            },
                                        );
                                    });

                                    ui.add_space(2.0);

                                    // Row 2: Modifier toggles and Extra Keys for this specific step
                                    ui.horizontal(|ui| {
                                        ui.add_space(26.0);
                                        ui.label(
                                            egui::RichText::new("Combine with:")
                                                .small()
                                                .color(egui::Color32::from_rgb(148, 163, 184)),
                                        );

                                        ui.add_enabled_ui(!running, |ui| {
                                            modifier_toggle(ui, &mut step.modifiers.ctrl, "Ctrl");
                                            modifier_toggle(ui, &mut step.modifiers.shift, "Shift");
                                            modifier_toggle(ui, &mut step.modifiers.alt, "Alt");
                                            modifier_toggle(ui, &mut step.modifiers.win, "Win");

                                            // Extra keys
                                            let mut remove_extra = None;
                                            for (e_idx, extra) in step.extra_keys.iter().enumerate()
                                            {
                                                let btn = egui::Button::new(
                                                    egui::RichText::new(format!(
                                                        "+ {} ✕",
                                                        extra.short_name()
                                                    ))
                                                    .small()
                                                    .color(egui::Color32::from_rgb(147, 197, 253)),
                                                )
                                                .fill(egui::Color32::from_rgb(30, 58, 138))
                                                .rounding(egui::Rounding::same(4.0));
                                                if ui
                                                    .add(btn)
                                                    .on_hover_text("Remove extra key")
                                                    .clicked()
                                                {
                                                    remove_extra = Some(e_idx);
                                                }
                                            }
                                            if let Some(e_idx) = remove_extra {
                                                step.extra_keys.remove(e_idx);
                                            }

                                            // Dropdown to add extra keys to this step
                                            egui::ComboBox::from_id_salt(format!(
                                                "add_extra_{}",
                                                idx
                                            ))
                                            .selected_text("+ Key")
                                            .width(70.0)
                                            .show_ui(ui, |ui| {
                                                for btn in ActionButton::all() {
                                                    if ui
                                                        .selectable_label(false, btn.label())
                                                        .clicked()
                                                    {
                                                        step.extra_keys.push(*btn);
                                                    }
                                                }
                                            });
                                        });
                                    });
                                });
                            ui.add_space(2.0);
                        }
                    });
            }

            if let Some(i) = move_up {
                self.steps.swap(i, i - 1);
            }
            if let Some(i) = move_down {
                self.steps.swap(i, i + 1);
            }
            if let Some(i) = duplicate_idx {
                let copy = self.steps[i].clone();
                self.steps.insert(i + 1, copy);
            }
            if let Some(i) = remove_idx {
                self.steps.remove(i);
            }
        });
    }
}

fn load_app_icon() -> Option<egui::IconData> {
    let svg_bytes = include_bytes!("../assets/keytape-icon.svg");
    let opt = usvg::Options::default();
    use usvg::TreeParsing;
    let tree = usvg::Tree::from_data(svg_bytes, &opt).ok()?;
    let size = tree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())?;
    let resvg_tree = resvg::Tree::from_usvg(&tree);
    resvg_tree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
    Some(egui::IconData {
        rgba: pixmap.take(),
        width: size.width(),
        height: size.height(),
    })
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([920.0, 680.0])
        .with_min_inner_size([780.0, 520.0])
        .with_title("Keytape // Rust Key & Mouse Sequencer");

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Keytape",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(KeySequencerApp::default()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_sequences_load_without_optional_chord_fields() {
        let steps: Vec<SequenceStep> = serde_json::from_str(
            r#"[{"button":"Enter","mode":"Click","hold_ms":1,"delay_after_ms":25,"enabled":true}]"#,
        )
        .unwrap();
        assert!(steps[0].modifiers.is_empty());
        assert!(steps[0].extra_keys.is_empty());
        let app = KeySequencerApp {
            steps,
            ..Default::default()
        };
        assert_eq!(app.total_duration_ms(), 45);
    }

    #[test]
    fn chord_roundtrip_preserves_order_and_duration() {
        let mut app = KeySequencerApp::default();
        app.load_preset("ctrl_enter");
        app.steps[0].extra_keys = vec![ActionButton::Shift, ActionButton::A];
        let serialized = serde_json::to_string(&app.steps).unwrap();
        app.steps = serde_json::from_str(&serialized).unwrap();
        assert_eq!(app.steps[0].display_label(), "Ctrl + Enter + Shift + A");
        assert_eq!(app.total_duration_ms(), 330); // hold + both 15ms chord waits + delay
        app.steps[0].enabled = false;
        assert_eq!(app.enabled_count(), 0);
        assert_eq!(app.total_duration_ms(), 0);
        app.start_sequence(); // Disabled steps must never initialize an input device.
        assert!(!app.is_running.load(Ordering::SeqCst));
        assert!(app.status_text.lock().unwrap().contains("No enabled steps"));
    }

    #[test]
    fn hold_preset_has_matching_release_and_ignores_click_hold_time() {
        let mut app = KeySequencerApp::default();
        app.load_preset("hold_right_click");
        assert_eq!(app.steps[0].button, app.steps[1].button);
        assert_eq!(app.steps[0].mode, PressMode::PressOnly);
        assert_eq!(app.steps[1].mode, PressMode::ReleaseOnly);
        app.steps[0].hold_ms = u64::MAX;
        assert_eq!(app.total_duration_ms(), 2500);
    }

    #[test]
    fn oversized_imported_durations_do_not_wrap_or_panic() {
        let mut app = KeySequencerApp::default();
        app.load_preset("ctrl_c_ctrl_v");
        app.steps[0].delay_after_ms = u64::MAX;
        assert_eq!(app.total_duration_ms(), u64::MAX);
        assert!(serde_json::from_str::<Vec<SequenceStep>>(r#"[{"button":"Unknown"}]"#).is_err());
    }

    #[test]
    fn imported_duration_limits_are_enforced_without_os_input() {
        let mut app = KeySequencerApp::default();
        assert!(validate_steps(&app.steps).is_ok());
        app.steps[0].hold_ms = 60_001;
        assert!(validate_steps(&app.steps).is_err());
        app.steps[0].hold_ms = 50;
        app.steps[0].delay_after_ms = 300_001;
        assert!(validate_steps(&app.steps).is_err());
    }

    #[test]
    fn cancelling_long_sleep_returns_without_waiting() {
        let stop = AtomicBool::new(true);
        assert!(!sleep_cancellable(60_000, &stop));
    }

    #[test]
    fn stop_does_not_prematurely_clear_running_state() {
        let mut app = KeySequencerApp::default();
        app.is_running.store(true, Ordering::Release);
        app.stop_sequence();
        assert!(app.stop_requested.load(Ordering::Acquire));
        assert!(app.is_running.load(Ordering::Acquire));
    }

    #[test]
    fn test_load_app_icon() {
        let icon = load_app_icon().expect("App icon SVG should load and render properly");
        assert_eq!(icon.width, 96);
        assert_eq!(icon.height, 96);
        assert_eq!(icon.rgba.len(), (96 * 96 * 4) as usize);
    }

    #[test]
    fn test_assets_exist() {
        let icon_svg = include_str!("../assets/keytape-icon.svg");
        let logo_svg = include_str!("../assets/keytape-logo.svg");
        let logo_dark_svg = include_str!("../assets/keytape-logo-dark.svg");

        assert!(icon_svg.contains("<svg"));
        assert!(logo_svg.contains("<svg"));
        assert!(logo_dark_svg.contains("<svg"));
    }
}
