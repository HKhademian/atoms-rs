#![feature(receiver_trait)]

mod glfw_app;

extern crate glfw;

use std::sync::mpsc::{channel, Receiver};
use std::thread::Builder;
use glfw::Context;
use crate::glfw_app::GLFWApp;


fn main() {
    let mut app = GLFWApp::new(500, 500, "GLFW App");

    let render_context = window.render_context();
    let (send, recv) = channel();

    let render_task = Builder::new().name("render task".to_string());
    let render_task_done = render_task.spawn(move || {
        render(render_context, recv);
    });


    while app.cycle() {

    }

    // Tell the render task to exit.
    send.send(()).ok().expect("Failed signal to render thread.");

    // Wait for acknowledgement that the rendering was completed.
    let _ = render_task_done;
}

fn render(mut context: glfw::RenderContext, finish: Receiver<()>) {
    context.make_current();
    loop {
        // Check if the rendering should stop.
        if finish.try_recv() == Ok(()) {
            break;
        };

        // Perform rendering calls
        

        context.swap_buffers();
    }

    // required on some platforms
    glfw::make_context_current(None);
}