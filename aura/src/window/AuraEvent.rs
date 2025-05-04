extern crate glfw;

pub enum Event {
    MouseMoved(f64,f64),
    MouseButtonPressed(glfw::MouseButton,glfw::Action,glfw::Modifiers),
    MouseButtonReleased(glfw::MouseButton,glfw::Action,glfw::Modifiers),
    MouseButtonRepeat(glfw::MouseButton,glfw::Action,glfw::Modifiers),
    KeyPressed(glfw::Key,glfw::Scancode,glfw::Action,glfw::Modifiers),
    KeyReleased(glfw::Key,glfw::Scancode,glfw::Action,glfw::Modifiers),
    KeyRepeat(glfw::Key,glfw::Scancode,glfw::Action,glfw::Modifiers),
    WindowResize(i32,i32)
}


pub struct EventQueue {
    pub events: Vec<Event>
}


impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: Vec::new()
        }
    }

    pub fn push_back(&mut self,event:Event) {
        self.events.push(event)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}