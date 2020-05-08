use gui::{GuiBuilder, PostBox, Widget, WidgetParser};
use std::any::Any;
use std::marker::PhantomData;

#[derive(Default)]
pub struct GuiCallback<T> where T: Widget {
	pub(super) callback_id: Option<usize>,
	phantom: PhantomData<T>,
}

impl<T> GuiCallback<T> where T: Widget {
	pub fn new(id: usize) -> GuiCallback<T> {
		GuiCallback {
			callback_id: Some(id),
			phantom: Default::default(),
		}
	}
}

impl<T> Drop for GuiCallback<T> where T: Widget {
	fn drop(&mut self) {
		WidgetParser::remove_callback(self);
	}
}

pub struct CallbackExecutor<'a> {
	gui_builder: &'a mut dyn Any,
	sender: &'a mut PostBox,
}

impl<'a> CallbackExecutor<'a> {
	pub(crate) fn execute<S>(&mut self, cb: &GuiCallback<S>, instance: &S)
	where S: Widget + 'static {
		WidgetParser::execute_callback(cb, self.gui_builder, instance, self.sender);
	}
}

pub struct GenericCallbackExecutor<D> where D: GuiBuilder {
	pub gui_builder: D,
	pub sender: PostBox,
}

impl<'a, D> From<&'a mut GenericCallbackExecutor<D>> for CallbackExecutor<'a> 
where 
	D: GuiBuilder + 'static {
	fn from(executor: &'a mut GenericCallbackExecutor<D>) -> Self {
		CallbackExecutor {
			gui_builder: &mut executor.gui_builder,
			sender: &mut executor.sender,
		}
	}
}
