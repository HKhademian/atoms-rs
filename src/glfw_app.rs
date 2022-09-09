extern crate glfw;

use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Key, Window, WindowEvent};

pub struct GLFWApp {
    glfw: glfw::Glfw,
    events: Receiver<(f64, WindowEvent)>,
    pub window: Window,
    pub handle_window_event: fn(window: &mut Window, event: WindowEvent) -> (),
}

impl GLFWApp {
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
    ) -> GLFWApp {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Cannot init GLFW");
        let (mut window, events) = glfw.create_window(
            width, height, title, glfw::WindowMode::Windowed,
        ).expect("Cannot create window");

        window.set_key_polling(true);
        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        GLFWApp {
            glfw, window, events,
            handle_window_event: Self::skeepy_handle_window_event,
        }
    }

    pub fn cycle(&mut self) -> bool {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            (self.handle_window_event)(&mut self.window, event);
        }

        !self.window.should_close()
    }

    fn skeepy_handle_window_event(window: &mut Window, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}