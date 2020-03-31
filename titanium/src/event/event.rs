#[derive(PartialEq)]
pub enum Event {
    MouseEvent(Mouse),
    CursorEvent{x: f32, y: f32},
    KeyEvent{
        state: ButtonState,
        key: Key,
    },
    Exit,
    Other,
}

pub trait EventBackend {
    fn get_event(&mut self) -> Event;
}

#[derive(PartialEq)]
pub enum ButtonState {
    Press,
    Release,
}

#[derive(PartialEq)]
pub enum Key {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
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
    Escape,
    Up,
    Down,
    Left,
    Right,
    MouseLeftButton,
    MouseRightButton,
    OTHER,
}

#[derive(PartialEq)]
pub enum Mouse {
    
}