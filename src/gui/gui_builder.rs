use gui::widget_parser::StoredCallback;
use gui::GuiCallback;
use gui::Widget;
use gui::WidgetParser;
use mecs::{Message, StaticWorld};
use std::any::Any;
use std::fmt::Debug;
use std::ops::Shl;
use std::time::Duration;

#[macro_export]
macro_rules! impl_widget_building_for {
    ($t:ty) => {
        impl Neg for $t {
            type Output = WidgetAdder;

            fn neg(self) -> Self::Output {
                WidgetParser::parse_push(self);
                WidgetAdder::new()
            }
        }

        impl Shl<()> for $t {
            type Output = ();

            fn shl(self, _rhs: ()) -> Self::Output {}
        }
    };
}

pub struct WidgetAdder {
    depth: u32,
}

impl WidgetAdder {
    pub fn new() -> Self {
        WidgetAdder { depth: 1 }
    }
}

impl Drop for WidgetAdder {
    fn drop(&mut self) {
        // println!("< {} --", self.depth);
        for _ in 0..self.depth {
            WidgetParser::parse_pop();
        }
    }
}

impl Shl<()> for WidgetAdder {
    type Output = ();

    fn shl(self, _rhs: ()) -> Self::Output {
        ()
    }
}

impl Shl<WidgetAdder> for WidgetAdder {
    type Output = WidgetAdder;

    fn shl(mut self, mut rhs: WidgetAdder) -> Self::Output {
        rhs.depth = 0;
        self.depth += 1;
        self
    }
}

pub trait GuiBuilder: Clone + PartialEq + Debug {
    fn make_callback3<F, S>(&self, f: F) -> GuiCallback<S>
    where
        F: 'static + Fn(&mut Self, &S, &mut StaticWorld),
        S: Widget + 'static,
        Self: Sized + 'static,
    {
        WidgetParser::add_callback(StoredCallback::ThreeParams(Box::new(
            move |input: &mut dyn Any, instance: &dyn Any, world: &mut StaticWorld| {
                f(
                    input.downcast_mut().unwrap(),
                    instance.downcast_ref().unwrap(),
                    world,
                );
            },
        )))
    }

    fn make_callback2<F, S>(&self, f: F) -> GuiCallback<S>
    where
        F: 'static + Fn(&mut Self, &S),
        S: Widget + 'static,
        Self: Sized + 'static,
    {
        WidgetParser::add_callback(StoredCallback::TwoParams(Box::new(
            move |input: &mut dyn Any, instance: &dyn Any| {
                f(
                    input.downcast_mut().unwrap(),
                    instance.downcast_ref().unwrap(),
                );
            },
        )))
    }
    fn make_callback1<F, S>(&self, f: F) -> GuiCallback<S>
    where
        F: 'static + Fn(&mut Self),
        S: Widget + 'static,
        Self: Sized + 'static,
    {
        WidgetParser::add_callback(StoredCallback::OneParam(Box::new(
            move |input: &mut dyn Any| {
                f(input.downcast_mut().unwrap());
            },
        )))
    }

    fn make_callback0<F, S>(&self, f: F) -> GuiCallback<S>
    where
        F: 'static + Fn(),
        S: Widget + 'static,
    {
        WidgetParser::add_callback(StoredCallback::ZeroParam(Box::new(f)))
    }

    fn build(&self);

    fn receive(&mut self, _msg: &Box<dyn Message>, _world: &mut StaticWorld) {}
    fn update(&mut self, _delta_time: Duration, _world: &mut StaticWorld) {}

    fn persist(&self, _world: &mut StaticWorld) {}
    fn restore(&mut self, _world: &mut StaticWorld) {}
}
