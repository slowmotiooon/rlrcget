pub enum Views {
    MusicTable,
}

pub struct ViewContext {
    pub current: Views,
}

impl ViewContext {
    pub fn new() -> ViewContext {
        ViewContext {
            current: Views::MusicTable,
        }
    }
}
