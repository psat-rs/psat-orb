pub extern crate orbtk;
extern crate psat;

use std::sync::Arc;
use orbtk::widgets::Widget;

pub struct OrbWindow {
    pub window: orbtk::Window
}

impl OrbWindow {
    fn add<T: Widget>(&mut self, widget: &Arc<T>) {
        self.window.add(widget);
    }
}

pub trait OrbComponent {
    fn add_to_window(self: Box<Self>, window: &mut OrbWindow);
    fn as_button(&self) -> Option<&orbtk::Button>;
}


impl OrbComponent for Arc<orbtk::Button> {
    fn add_to_window(self: Box<Self>, window: &mut OrbWindow) {
        window.add(&self);
    }
    fn as_button(&self) -> Option<&orbtk::Button> {
        Some(self)
    }
}

impl psat::Target for OrbWindow {
    type Component = Box<OrbComponent>;
    type Context = ();
    fn get_context(&mut self) -> &Self::Context {
        &()
    }
    fn set_root(&mut self, widget: Self::Component) {
        OrbComponent::add_to_window(widget, self);
    }
}

pub struct ButtonComponent {}
pub struct ButtonProps {
    pub text: String
}
impl psat::NativeComponent<OrbWindow> for ButtonComponent {
    type Props = ButtonProps;
    fn create(&self, _c: &()) -> <OrbWindow as psat::Target>::Component {
        Box::new(orbtk::Button::new())
    }
    fn reconcile(&self, _c: &(), component: &mut <OrbWindow as psat::Target>::Component, props: &ButtonProps, _children: &Vec<psat::VNode<OrbWindow>>) {
        match component.as_button() {
            Some(btn) => {
                btn.text.set(props.text.to_owned());
            },
            None => eprintln!("Warning: component was not a button")
        }
    }
}
pub const BUTTON: ButtonComponent = ButtonComponent {};
