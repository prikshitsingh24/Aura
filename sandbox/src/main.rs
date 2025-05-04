use std::sync::Arc;
use std::sync::Mutex;

use aura::renderer::AuraShapes;
use aura::window::{AuraEvent,AuraWindow};
use aura::renderer::AuraRenderer;
extern crate glfw;
use glfw::Context; 

fn main() {
    let aura_window = &AuraWindow::Window::new("Aura Engine",600,400);
    let event_queue = Arc::new(Mutex::new(AuraEvent::EventQueue::new()));
    let (mut window,mut glfw) = aura_window.create(Arc::clone(&event_queue));

    AuraRenderer::Renderer::initialize(&mut window);

    while !window.should_close() {
        glfw.poll_events(); 

        if let Ok(mut queue) = event_queue.lock() {
           for event in  queue.events.drain(..) {
                match event {
                    AuraEvent::Event::KeyPressed(key, _, _, _) => {
                        println!("Key pressed: {:?}", key);
                    }
                    AuraEvent::Event::MouseButtonPressed(button, _, _) => {
                        println!("Mouse button pressed: {:?}", button);
                    }
                    AuraEvent::Event::MouseMoved(x,y ) => {
                        print!("Mouse moving {} {} \n",x,y);
                    }
                    _ => {}
                }
            }
      
        }

        // Render -----------------
        AuraRenderer::Renderer::fill(0.0, 0.0, 0.0, 0.0);
        AuraRenderer::Renderer::draw(AuraShapes::Shapes::Pyramid);

        window.swap_buffers();
    }
    
}

