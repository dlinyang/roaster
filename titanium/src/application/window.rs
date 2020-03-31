use crate::base::Size;

#[derive(Clone,Copy)]
pub struct Window {
    pub size: Size,
    pub v_sync: bool,
    pub decoration: bool,
    pub resizable: bool,
}

impl Window {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Size::new(width, height),
            v_sync: true,
            decoration: true,
            resizable: false,
        }
    }
}