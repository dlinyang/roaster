use std::collections::HashMap;

pub struct FontBuilder {
    data: HashMap<Str,Font>,
}

pub struct Font {
    data: Vec<u8>,
}