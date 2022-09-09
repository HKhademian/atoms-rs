use std::time::Duration;
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub(crate) struct SDLApp {
    context: Sdl,
    events: EventPump,
    pub canvas: WindowCanvas,
    pub event_handler: fn(&Event) -> bool,
    pub update: fn(&mut Self) -> (),
}

impl SDLApp {
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
    ) -> SDLApp {
        let context = sdl2::init().expect("Cannot init SDL2");
        let events = context.event_pump().expect("Cannot get events pump");
        let window = context.video()
            .expect("Cannot get video system")
            .window(title, width, height)
            .position_centered()
            // .resizable()
            .build()
            .expect("Cannot create window");
        let canvas = window.into_canvas()
            .accelerated()
            .build()
            .expect("Cannot get canvas");

        SDLApp {
            context,
            events,
            canvas,
            update: |_| (),
            event_handler: |event| {
                let (close, _) = Self::default_event_handler(event);
                close
            },
        }
    }

    pub fn cycle(&mut self) -> bool {
        self.canvas.present();

        for event in self.events.poll_iter() {
            if (self.event_handler)(&event) {
                return false;
            }
        }

        (self.update)(self);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        true
    }

    pub fn default_event_handler(event: &Event) -> (bool, bool) {
        match event {
            Event::Quit { .. } |
            Event::KeyUp { keycode: Some(Keycode::Escape), .. } |
            Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                (/*close*/ true, /*handled*/ true)
            }
            _ => { (/*close*/ false, /*handled*/ false) }
        }
    }
}