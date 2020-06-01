use rmu::vector::Vector2;
use titanium::event::*;

#[derive(Clone,Copy,PartialEq)]
pub enum DragState{
    NoDrag,
    DragStart,
    Dragging,
    DragEnd,
}

pub struct Drag {
    pub start: Vector2,
    pub end: Vector2,
    pub state: DragState,
    pub button: MouseButton,
}

impl Drag {
    pub fn new(button: MouseButton) -> Self {
        Self {
            start: Vector2::default(),
            end: Vector2::default(),
            state: DragState::NoDrag,
            button: button,
        }
    }

    pub fn match_event(&mut self, event: Event) {
        match event {
            Event::MouseEvent {state, button} => {
                if button == self.button {
                    match state {
                        ButtonState::Press => self.state = DragState::DragStart,
                        ButtonState::Release => self.state = DragState::DragEnd,
                    }
                }
            },
            Event::CursorEvent{x,y} => {
                if self.state == DragState::DragStart {
                    self.state = DragState::Dragging;
                    self.start = Vector2::new(x, y);
                }

                if self.state == DragState::Dragging {
                    self.end = Vector2::new(x, y);
                }
            },
            _ => (),
        }
    }

    pub fn get_move(&self) -> Vector2 {
        self.start - self.end
    }
}

pub struct KeyCombination {
    pub keys: Vec<Key>,
    pub count: usize,
    pub flag: bool,
}

impl KeyCombination {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            count: 0,
            flag: false,
        }
    }

    pub fn match_event(&mut self, event: Event) {
        match event {
            Event::KeyEvent{ state: ButtonState::Press, key} => {
                if !self.flag {
                    if key == self.keys[self.count] {
                        self.count += 1;
                        if self.count == self.keys.len() {
                            self.flag = true;
                        }
                    } else {
                        self.count = 0;
                    }
                }
            },
            _ => (),
        }
    }

    pub fn matched(&mut self) -> bool {
        if self.flag {
            self.flag = false;
            true
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! key_combination {
    ($($key:expr),+) => {
        KeyCombination {
            keys: vec![$($key),+]
            count: 0,
            flag: false,
        }
    };
}