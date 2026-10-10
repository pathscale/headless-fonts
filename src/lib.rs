//! A fixed set of open fonts for headless rendering, and the CSS family names each one stands in for.
//!
//! A headless browser that reads the machine's installed fonts renders differently on every machine, so a pixel
//! check passes on one and fails on another. This crate is the alternative: every face is compiled into the binary,
//! so `Georgia`, `system-ui` or `monospace` resolve to the same glyphs everywhere. A windowed browser should keep
//! using system fonts; this set is for hosts that must be reproducible.
//!
//! Faces: Gelasio (serif, metric-compatible with Georgia), Noto Sans, Noto Mono, Noto Sans Devanagari, all OFL-1.1
//! (see `assets/fonts/*LICENSE*`); the code is MIT.

/// One embedded face.
#[derive(Debug, Clone, Copy)]
pub struct Face {
    /// The family name the face is registered under.
    pub family: &'static str,
    /// CSS font-weight, 100 to 900. For a variable face, the default instance; `weight_range` is what it covers.
    pub weight: u16,
    /// The weights a variable face covers through its `wght` axis; equal to `(weight, weight)` for a static face.
    pub weight_range: (u16, u16),
    pub italic: bool,
    /// The TrueType file.
    pub data: &'static [u8],
}

/// Which bundled family a CSS generic family resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generic {
    Serif,
    SansSerif,
    Monospace,
}

macro_rules! font {
    ($file:literal) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/", $file))
    };
}

pub const SANS: &str = "Noto Sans";
/// Gelasio is metric-compatible with Georgia, so text set in Georgia keeps its line breaks.
pub const SERIF: &str = "Gelasio";
pub const MONO: &str = "Noto Mono";
pub const DEVANAGARI: &str = "Noto Sans Devanagari";

/// Every embedded face.
pub const FACES: &[Face] = &[
    Face {
        family: SANS,
        weight: 400,
        weight_range: (400, 400),
        italic: false,
        data: font!("NotoSans-Regular.ttf"),
    },
    Face {
        family: SANS,
        weight: 700,
        weight_range: (700, 700),
        italic: false,
        data: font!("NotoSans-Bold.ttf"),
    },
    Face {
        family: SANS,
        weight: 400,
        weight_range: (400, 400),
        italic: true,
        data: font!("NotoSans-Italic.ttf"),
    },
    Face {
        family: SERIF,
        weight: 400,
        weight_range: (400, 700),
        italic: false,
        data: font!("Gelasio-VariableWeight.ttf"),
    },
    Face {
        family: SERIF,
        weight: 400,
        weight_range: (400, 700),
        italic: true,
        data: font!("Gelasio-Italic-VariableWeight.ttf"),
    },
    Face {
        family: MONO,
        weight: 400,
        weight_range: (400, 400),
        italic: false,
        data: font!("NotoMono-Regular.ttf"),
    },
    Face {
        family: DEVANAGARI,
        weight: 400,
        weight_range: (400, 400),
        italic: false,
        data: font!("NotoSansDevanagari-Regular.ttf"),
    },
];

/// The bundled family for a CSS generic family.
pub const fn generic(generic: Generic) -> &'static str {
    match generic {
        Generic::Serif => SERIF,
        Generic::SansSerif => SANS,
        Generic::Monospace => MONO,
    }
}

/// CSS family names (lowercase) and the bundled family that stands in for each. Page fonts loaded with
/// `@font-face` keep their own names; this only covers names a page expects the system to have.
pub const ALIASES: &[(&str, &str)] = &[
    // Serif.
    ("serif", SERIF),
    ("georgia", SERIF),
    ("times new roman", SERIF),
    ("times", SERIF),
    ("ui-serif", SERIF),
    ("cambria", SERIF),
    // Sans.
    ("sans-serif", SANS),
    ("system-ui", SANS),
    ("-apple-system", SANS),
    ("blinkmacsystemfont", SANS),
    ("ui-sans-serif", SANS),
    ("segoe ui", SANS),
    ("roboto", SANS),
    ("helvetica", SANS),
    ("helvetica neue", SANS),
    ("arial", SANS),
    ("inter", SANS),
    ("ubuntu", SANS),
    ("cantarell", SANS),
    ("noto sans", SANS),
    // Monospace.
    ("monospace", MONO),
    ("ui-monospace", MONO),
    ("sfmono-regular", MONO),
    ("menlo", MONO),
    ("monaco", MONO),
    ("consolas", MONO),
    ("liberation mono", MONO),
    ("courier new", MONO),
    ("courier", MONO),
];

/// The bundled family a CSS family name resolves to, if it is one this set covers. Case-insensitive; quotes and
/// surrounding whitespace are ignored.
pub fn resolve(css_family: &str) -> Option<&'static str> {
    let name = css_family
        .trim()
        .trim_matches(|c| c == '"' || c == '\'')
        .trim();
    ALIASES
        .iter()
        .find(|(alias, _)| alias.eq_ignore_ascii_case(name))
        .map(|(_, family)| *family)
}

/// Families to try, in order, for text no other font covers (script fallback).
pub const FALLBACK: &[&str] = &[SANS, DEVANAGARI];
