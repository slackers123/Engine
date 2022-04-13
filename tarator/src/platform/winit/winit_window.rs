use crate::tarator::{
    window::{WindowProps, Window, self},
    event::{
        Event,
        application_event::*,
        key_event::*
    },
    core::{UPtr, SPtr}
};
use winit::{
    dpi::LogicalSize,
    window::WindowBuilder,
    platform::run_return::EventLoopExtRunReturn,
    event_loop::ControlFlow
};

struct WinitWindowData/*<'a>*/ {
    #[allow(unused)]
    title: String,
    #[allow(unused)]
    width: u32,
    #[allow(unused)]
    height: u32,
    #[allow(unused)]
    vsync: bool
}
/// ## WGPU Implementation of window trait
/// tarator/window.rs
pub struct WinitWindow/*<'a>*/ {
    #[allow(unused)]
    event_loop: UPtr<winit::event_loop::EventLoop<()>>,
    #[allow(unused)]
    window: UPtr<winit::window::Window>,
    #[allow(unused)]
    data: WinitWindowData/*<'a>*/
}
impl/*<'a>*/ Window for WinitWindow/*<'a>*/ {
    fn update(&mut self) -> SPtr<dyn Event> {
        let mut return_event: SPtr<dyn Event> = SPtr::new(ApplicationUpdateEvent::default());
        self.event_loop.run_return(|event, _target, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                winit::event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                        match event {
                            winit::event::WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                                return_event = SPtr::new(WindowCloseEvent::default());
                            },
                            winit::event::WindowEvent::KeyboardInput {
                                input: winit::event::KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    ..
                                },
                                ..
                            } => {
                                return_event = SPtr::new(KeyPressedEvent::default());
                            },
                            _ => {}
                        }
                },
                _ => ()
            };
        });
        return return_event;
    }
    fn get_width(&self) {}
    fn get_height(&self) {}

    #[allow(unused)]
    fn set_vsync(&mut self, enabled: bool) { self.data.vsync = enabled; }
    fn get_vsync_enabled(&self) -> bool { return self.data.vsync; }

    #[allow(unused)]
    fn new(window_props: &WindowProps) -> WinitWindow/*<'a>*/ {
        TR_INFO!("Executing WinitWindow::new();\n");
        let event_loop= winit::event_loop::EventLoop::new();
        let data: WinitWindowData = WinitWindowData {
            title: window_props.title.clone(),
            width: window_props.width,
            height: window_props.height,
            vsync: true                  // VSYNC IS HARDCODED HERE
        };
        let window = match WindowBuilder::new()
                .with_inner_size(LogicalSize {
                    width: data.width,
                    height: data.height
                })
                .build(&event_loop) {
            Ok(window) => window,
            Err(os_error) => panic!("Failed To Create Window!")
        };
        return WinitWindow {
            event_loop: UPtr::new(event_loop),
            window: UPtr::new(window),
            data: data
        };
    }
}
