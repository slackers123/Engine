/// # Application
/// Here, the application base trait is implemented
/// Use this trait in another crate to create your application

use crate::tarator::{
    window::{WindowProps, Window},
    layer::*,
    core::SPtr
};
pub trait Application<TWindow> where
    TWindow: Window{
    fn new(window_props: &WindowProps) -> Self;
    fn run(&mut self);
    fn push_layer(&mut self, layer: SPtr<dyn Layer>);
    fn push_overlay(&mut self, layer: SPtr<dyn Layer>);
}

#[macro_export]
macro_rules! APPLICATION_DECLARE {
    ($label:tt) => {
        pub struct $label<TWindow> where
            TWindow: Window {
            window: UPtr<TWindow>,
            layer_stack: UPtr<LayerStack>
        }
    };
}
#[macro_export]
macro_rules! APPLICATION_DEFAULTIMPL {
    ($label:tt) => {
        fn push_layer(&mut self, layer: SPtr<dyn Layer>) {
            layer.attach();
            self.layer_stack.push_layer(layer);
        }
        fn push_overlay(&mut self, layer: SPtr<dyn Layer>) {
            layer.attach();
            self.layer_stack.push_overlay(layer);
        }
        fn run(&mut self) {
            loop {
                let event = self.window.update();
                for layer in self.layer_stack.layers.iter() {
                    layer.update();
                    layer.event(event.as_ref());
                }
                match event.get_action() {
                    EventAction::WINDOWCLOSE => return,
                    _ => {}
                }
            }
        }
    };
}
