use crate::address::Addresses;

#[derive(Clone, Copy)]
pub struct Input([u8; 0x40]);

impl Input {
    pub fn get(addresses: &Addresses) -> Self {
        let inputs = unsafe { (addresses.keyboard_values as *const [u8; 0x40]).read() };
        Self(inputs)
    }

    pub fn held(&self, key: Key) -> bool {
        let (idx, mask) = key.0;
        self.0[idx] & mask != 0
    }

    pub fn pressed(&self, key: Key) -> bool {
        let (idx, mask) = key.0;
        self.0[idx + 0x20] & mask != 0
    }
}

// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
macro_rules! def_enum {
    (
        $(#[$attr:meta])*
        $vis:vis $name:ident => $ty:ty {
            $($variant:ident => $val:expr),+
            $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis struct $name($ty);

        impl $name {
            $(
                pub const $variant: Self = Self($val);
            )+
        }
    };
}

def_enum!(
    #[derive(Clone, Copy, PartialEq)]
    pub Key => (usize, u8) {
        W => (0x2, 0x2),
        A => (0x3, 0x40),
        S => (0x3, 0x80),
        D => (0x4, 0x1),
        I => (0x2, 0x80),
        J => (0x4, 0x10),
        K => (0x4, 0x20),
        L => (0x4, 0x40),
        Z => (0x5, 0x10),
        C => (0x5, 0x40),
        F => (0x4, 0x2),
        SHIFT => (0x5, 0x4),
        CTRL => (0x3, 0x20),
        SPACE => (0x7, 0x2),
        F11 => (0xA, 0x80),
        F12 => (0xB, 0x1),
        N1 => (0x0, 0x4),
        N2 => (0x0, 0x8),
        N3 => (0x0, 0x10),
        N4 => (0x0, 0x20),
        N5 => (0x0, 0x40),
        Q => (0x2, 0x1),
        E => (0x2, 0x4),
        R => (0x2, 0x8),
        T => (0x2, 0x10),
        U => (0x2, 0x40),
        O => (0x3, 0x1),
    }
);

// #[derive(Clone, Copy, PartialEq)]
// pub enum Key {
//     W,
//     A,
//     S,
//     D,
//     I,
//     J,
//     K,
//     L,
//     Z,
//     C,
//     F,
//     Shift,
//     Ctrl,
//     Space,
//     F11,
//     F12,
//     N1,
//     N2,
//     N3,
//     N4,
//     N5,
//     Q,
//     E,
//     R,
//     T,
//     U,
//     O,
// }

// impl Key {
//     const fn idx_mask(&self) -> (usize, u8) {
//         match self {
//             Key::W => (0x2, 0x2),
//             Key::A => (0x3, 0x40),
//             Key::S => (0x3, 0x80),
//             Key::D => (0x4, 0x1),
//             Key::I => (0x2, 0x80),
//             Key::J => (0x4, 0x10),
//             Key::K => (0x4, 0x20),
//             Key::L => (0x4, 0x40),
//             Key::Z => (0x5, 0x10),
//             Key::C => (0x5, 0x40),
//             Key::F => (0x4, 0x2),
//             Key::Shift => (0x5, 0x4),
//             Key::Ctrl => (0x3, 0x20),
//             Key::Space => (0x7, 0x2),
//             Key::F11 => (0xA, 0x80),
//             Key::F12 => (0xB, 0x1),
//             Key::N1 => (0x0, 0x4),
//             Key::N2 => (0x0, 0x8),
//             Key::N3 => (0x0, 0x10),
//             Key::N4 => (0x0, 0x20),
//             Key::N5 => (0x0, 0x40),
//             Key::Q => (0x2, 0x1),
//             Key::E => (0x2, 0x4),
//             Key::R => (0x2, 0x8),
//             Key::T => (0x2, 0x10),
//             Key::U => (0x2, 0x40),
//             Key::O => (0x3, 0x1),
//         }
//     }
// }
