extern crate glfw;

use std::sync::{Arc, Mutex};

use glfw::{Action, Context};

use super::AuraEvent;

pub struct Window{
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(title: &str, width: u32, height:u32) -> Self {
        Window {
            title: String::from(title),
            width,
            height,
        }
    }

    pub fn create(&self,  event_queue: Arc<Mutex<AuraEvent::EventQueue>>) -> (glfw::PWindow, glfw::Glfw) {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, _) = glfw.create_window(self.width, self.height, &self.title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);


        let queue_for_key = Arc::clone(&event_queue);
        window.set_key_callback(move | _, key, scancode, action: Action, mods | {
            if let Ok(mut q) = queue_for_key.lock() {
                match action {
                    glfw::Action::Press => q.push_back(AuraEvent::Event::KeyPressed(key, scancode, action, mods)),
                    glfw::Action::Release => q.push_back(AuraEvent::Event::KeyReleased(key, scancode, action, mods)),
                    glfw::Action::Repeat => q.push_back(AuraEvent::Event::KeyRepeat(key, scancode, action, mods)),
                }
            }
        });

        let queue_for_mouse_button = Arc::clone(&event_queue);
        window.set_mouse_button_callback(move | _, button, action, mods | {
            if let Ok(mut q) = queue_for_mouse_button.lock() {
                match action {
                    glfw::Action::Press => q.push_back(AuraEvent::Event::MouseButtonPressed(button, action, mods)),
                    glfw::Action::Release => q.push_back(AuraEvent::Event::MouseButtonReleased(button, action, mods)),
                    glfw::Action::Repeat => q.push_back(AuraEvent::Event::MouseButtonRepeat(button, action, mods)),
                }
            }
        });

        let queue_for_mouse_move = Arc::clone(&event_queue);
        window.set_cursor_pos_callback(move | _, x, y  | {
            if let Ok(mut q) = queue_for_mouse_move.lock() {
                q.push_back(AuraEvent::Event::MouseMoved(x, y))
            }
        });


       
        window.make_current();
        (window, glfw)
    }
}