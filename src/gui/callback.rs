use gui::{GuiBuilder, Widget, WidgetParser};
use mecs::StaticWorld;
use std::any::Any;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct GuiCallback<T>
where
    T: Widget,
{
    pub(super) callback_id: Rc<Option<usize>>,
    phantom: PhantomData<T>,
}

impl<T> GuiCallback<T>
where
    T: Widget,
{
    pub fn new(id: usize) -> GuiCallback<T> {
        GuiCallback {
            callback_id: Rc::new(Some(id)),
            phantom: Default::default(),
        }
    }
}

impl<T> Drop for GuiCallback<T>
where
    T: Widget,
{
    fn drop(&mut self) {
        if Rc::strong_count(&self.callback_id) == 1 {
            if let Some(id) = *self.callback_id {
                WidgetParser::remove_callback(id);
            }
        }
    }
}

pub struct CallbackExecutor<'a> {
    gui_builder: &'a mut dyn Any,
    static_world: &'a mut StaticWorld,
}

impl<'a> CallbackExecutor<'a> {
    pub fn execute<S>(&mut self, cb: &GuiCallback<S>, instance: &S)
    where
        S: Widget + 'static,
    {
        WidgetParser::execute_callback(cb, self.gui_builder, instance, self.static_world);
    }
}

impl<'a, D> From<(&'a mut D, &'a mut StaticWorld)> for CallbackExecutor<'a>
where
    D: GuiBuilder + 'static,
{
    fn from(pair: (&'a mut D, &'a mut StaticWorld)) -> Self {
        CallbackExecutor {
            gui_builder: pair.0,
            static_world: pair.1,
        }
    }
}
