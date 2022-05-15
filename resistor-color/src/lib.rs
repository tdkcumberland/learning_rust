use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoEnumIterator, IntEnum)]
#[repr(usize)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {

    if let Ok(output) = ResistorColor::from_int(value) {
        format!("{:?}", output)
    } else {
        "value out of range".to_owned()
    }

}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
