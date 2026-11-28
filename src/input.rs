use {miniquad as mq, mq::KeyCode};

pub fn on_frame_start(egui_input: &mut egui::RawInput, equi_ctx: &egui::Context) {
    let screen_size_in_pixels = mq::window::screen_size();
    let screen_size_in_points =
        egui::vec2(screen_size_in_pixels.0, screen_size_in_pixels.1) / equi_ctx.pixels_per_point();
    egui_input.screen_rect = Some(egui::Rect::from_min_size(
        Default::default(),
        screen_size_in_points,
    ));
    egui_input.time = Some(mq::date::now());
}

/// miniquad sends special keys (backspace, delete, F1, ...) as characters.
/// Ignore those.
/// We also ignore '\r', '\n', '\t'.
/// Newlines are handled by the `Key::Enter` event.
pub fn is_printable_char(chr: char) -> bool {
    #![allow(clippy::manual_range_contains)]

    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';

    !is_in_private_use_area && !chr.is_ascii_control()
}

pub fn egui_modifiers_from_mq_modifiers(keymods: mq::KeyMods) -> egui::Modifiers {
    egui::Modifiers {
        alt: keymods.alt,
        ctrl: keymods.ctrl,
        shift: keymods.shift,
        mac_cmd: keymods.logo && cfg!(target_os = "macos"),
        command: if cfg!(target_os = "macos") {
            keymods.logo
        } else {
            keymods.ctrl
        },
    }
}

pub fn egui_key_from_mq_key(key: KeyCode) -> Option<egui::Key> {
    Some(match key {
        KeyCode::Down => egui::Key::ArrowDown,
        KeyCode::Left => egui::Key::ArrowLeft,
        KeyCode::Right => egui::Key::ArrowRight,
        KeyCode::Up => egui::Key::ArrowUp,

        KeyCode::Escape => egui::Key::Escape,
        KeyCode::Tab => egui::Key::Tab,
        KeyCode::Backspace => egui::Key::Backspace,
        KeyCode::Enter => egui::Key::Enter,
        KeyCode::Space => egui::Key::Space,

        KeyCode::Insert => egui::Key::Insert,
        KeyCode::Delete => egui::Key::Delete,
        KeyCode::Home => egui::Key::Home,
        KeyCode::End => egui::Key::End,
        KeyCode::PageUp => egui::Key::PageUp,
        KeyCode::PageDown => egui::Key::PageDown,

        KeyCode::Key0 => egui::Key::Num0,
        KeyCode::Key1 => egui::Key::Num1,
        KeyCode::Key2 => egui::Key::Num2,
        KeyCode::Key3 => egui::Key::Num3,
        KeyCode::Key4 => egui::Key::Num4,
        KeyCode::Key5 => egui::Key::Num5,
        KeyCode::Key6 => egui::Key::Num6,
        KeyCode::Key7 => egui::Key::Num7,
        KeyCode::Key8 => egui::Key::Num8,
        KeyCode::Key9 => egui::Key::Num9,

        KeyCode::A => egui::Key::A,
        KeyCode::B => egui::Key::B,
        KeyCode::C => egui::Key::C,
        KeyCode::D => egui::Key::D,
        KeyCode::E => egui::Key::E,
        KeyCode::F => egui::Key::F,
        KeyCode::G => egui::Key::G,
        KeyCode::H => egui::Key::H,
        KeyCode::I => egui::Key::I,
        KeyCode::J => egui::Key::J,
        KeyCode::K => egui::Key::K,
        KeyCode::L => egui::Key::L,
        KeyCode::M => egui::Key::M,
        KeyCode::N => egui::Key::N,
        KeyCode::O => egui::Key::O,
        KeyCode::P => egui::Key::P,
        KeyCode::Q => egui::Key::Q,
        KeyCode::R => egui::Key::R,
        KeyCode::S => egui::Key::S,
        KeyCode::T => egui::Key::T,
        KeyCode::U => egui::Key::U,
        KeyCode::V => egui::Key::V,
        KeyCode::W => egui::Key::W,
        KeyCode::X => egui::Key::X,
        KeyCode::Y => egui::Key::Y,
        KeyCode::Z => egui::Key::Z,
        KeyCode::Comma => egui::Key::Comma,
        KeyCode::Backslash => egui::Key::Backslash,
        KeyCode::LeftBracket => egui::Key::OpenBracket,
        KeyCode::RightBracket => egui::Key::CloseBracket,
        KeyCode::GraveAccent => egui::Key::Backtick,
        KeyCode::Minus => egui::Key::Minus,
        KeyCode::Period => egui::Key::Period,
        KeyCode::Equal => egui::Key::Equals,
        KeyCode::Semicolon => egui::Key::Semicolon,
        KeyCode::Apostrophe => todo!(),
        KeyCode::Slash => egui::Key::Slash,
        KeyCode::CapsLock => egui::Key::CapsLock,
        KeyCode::ScrollLock => egui::Key::ScrollLock,
        KeyCode::NumLock => egui::Key::NumLock,
        KeyCode::PrintScreen => egui::Key::PrintScreen,
        KeyCode::Pause => egui::Key::Pause,
        KeyCode::F1 => egui::Key::F1,
        KeyCode::F2 => egui::Key::F2,
        KeyCode::F3 => egui::Key::F3,
        KeyCode::F4 => egui::Key::F4,
        KeyCode::F5 => egui::Key::F5,
        KeyCode::F6 => egui::Key::F6,
        KeyCode::F7 => egui::Key::F7,
        KeyCode::F8 => egui::Key::F8,
        KeyCode::F9 => egui::Key::F9,
        KeyCode::F10 => egui::Key::F10,
        KeyCode::F11 => egui::Key::F11,
        KeyCode::F12 => egui::Key::F12,
        KeyCode::F13 => egui::Key::F13,
        KeyCode::F14 => egui::Key::F14,
        KeyCode::F15 => egui::Key::F15,
        KeyCode::F16 => egui::Key::F16,
        KeyCode::F17 => egui::Key::F17,
        KeyCode::F18 => egui::Key::F18,
        KeyCode::F19 => egui::Key::F19,
        KeyCode::F20 => egui::Key::F20,
        KeyCode::F21 => egui::Key::F21,
        KeyCode::F22 => egui::Key::F22,
        KeyCode::F23 => egui::Key::F23,
        KeyCode::F24 => egui::Key::F24,
        KeyCode::F25 => egui::Key::F25,
        KeyCode::LeftShift => egui::Key::ShiftLeft,
        KeyCode::LeftControl => egui::Key::ControlLeft,
        KeyCode::LeftAlt => egui::Key::AltLeft,
        KeyCode::LeftSuper => egui::Key::SuperLeft,
        KeyCode::RightShift => egui::Key::ShiftRight,
        KeyCode::RightControl => egui::Key::ControlRight,
        KeyCode::RightAlt => egui::Key::AltRight,
        KeyCode::RightSuper => egui::Key::SuperRight,
        KeyCode::Menu => egui::Key::Menu,
        KeyCode::Back => egui::Key::BrowserBack,
        _ => return None,
    })
}
