use crate::plugin_rack::InputChannelType;

pub enum Action {
    OpenEditor(usize),
    Remove(usize),
    Bypass(usize),
    ChangeInputChannel(usize, InputChannelType),
    ChangeOutputChannel(usize, usize),
    ChangeWet(usize, f32),
    ChangeSampleRate(usize, f32)
}

pub enum ModalWindows {
    Exit,
    ExitNew,
    None
}

pub enum SaveDialogVariant {
    Yes,
    No,
    Cancel,
    None,
}