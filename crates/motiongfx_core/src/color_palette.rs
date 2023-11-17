use bevy_render::color::Color;
use bevy_utils::HashMap;

#[derive(Clone)]
pub struct ColorPalette<Key: Sized> {
    pub palette: HashMap<Key, Color>,
}

impl Default for ColorPalette<ColorKey> {
    fn default() -> Self {
        Self {
            palette: [
                (ColorKey::Red, Color::hex("FF6188").unwrap_or_default()),
                (ColorKey::Orange, Color::hex("FC9867").unwrap_or_default()),
                (ColorKey::Yellow, Color::hex("FFD866").unwrap_or_default()),
                (ColorKey::Green, Color::hex("A9DC76").unwrap_or_default()),
                (ColorKey::Blue, Color::hex("78DCE8").unwrap_or_default()),
                (ColorKey::Purple, Color::hex("AB9DF2").unwrap_or_default()),
                (ColorKey::Base0, Color::hex("19181A").unwrap_or_default()),
                (ColorKey::Base1, Color::hex("221F22").unwrap_or_default()),
                (ColorKey::Base2, Color::hex("2D2A2E").unwrap_or_default()),
                (ColorKey::Base3, Color::hex("403E41").unwrap_or_default()),
                (ColorKey::Base4, Color::hex("5B595C").unwrap_or_default()),
                (ColorKey::Base5, Color::hex("727072").unwrap_or_default()),
                (ColorKey::Base6, Color::hex("939293").unwrap_or_default()),
                (ColorKey::Base7, Color::hex("C1C0C0").unwrap_or_default()),
                (ColorKey::Base8, Color::hex("FCFCFA").unwrap_or_default()),
            ]
            .into(),
        }
    }
}

#[derive(Default, PartialEq, Eq, Hash)]
pub enum ColorKey {
    // Primary colors
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    // Base colors, sorted from darkest to lightest
    Base0,
    Base1,
    Base2,
    Base3,
    Base4,
    Base5,
    Base6,
    Base7,
    #[default]
    Base8,
}
