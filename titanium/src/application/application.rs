use super::{
    world::World,
    window::Window
};

use crate::base::{Vec4f, color::WHITE};

///The Application struct include the basic function to create a graphic program
/// #Example
/// ```
/// use titanium::*
/// 
/// let application = Application::mew().init();
/// ```
pub struct Application {
    pub name: String,
    pub world: World,
    pub window: Window,
}

///
impl Application {
    pub fn new() -> Self {
        Application::create("Titanium White".to_string(), 1240.0, 720.0)
    }

    //
    pub fn create(name: String, width: f32, height: f32) -> Self {
        Self {        
            name,
            world: World::new(width, height),
            window: Window::new(width,height),
        }
    }
}

use crate::renderer::Renderer;
use crate::event::EventBackend;
///
pub trait ApplicationBackend<T> where T: Renderer + EventBackend {
    fn init(&self) -> T;
}