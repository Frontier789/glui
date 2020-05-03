use gui::{WidgetParser, PostBox};
use std::any::Any;

#[derive(Default)]
pub struct GuiCallback {
    pub(super) callback_id: Option<usize>,
}

impl Drop for GuiCallback {
    fn drop(&mut self) {
        WidgetParser::remove_callback(self);
    }
}

pub struct CallbackExecutor {
    pub data: Box<dyn Any>,
    pub sender: PostBox,
}

impl CallbackExecutor {
    pub fn execute<S>(&mut self, cb: &GuiCallback, instance: &S)
    where
        S: 'static,
    {
        WidgetParser::execute_callback(cb, &mut *self.data, instance, &mut self.sender);
    }
}
