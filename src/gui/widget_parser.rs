use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use gui::{GuiBuilder, GuiCallback, Widget, WidgetBuilderCache, WidgetList};
use mecs::StaticWorld;

thread_local! {
    static WIDGETPARSER_INSTANCE: RefCell<WidgetParser> = RefCell::new(WidgetParser::default());
}

pub enum StoredCallback {
    ZeroParam(Box<dyn Fn()>),
    OneParam(Box<dyn Fn(&mut dyn Any)>),
    TwoParams(Box<dyn Fn(&mut dyn Any, &dyn Any)>),
    ThreeParams(Box<dyn Fn(&mut dyn Any, &dyn Any, &mut StaticWorld)>),
}

#[derive(Default)]
pub struct WidgetParser {
    output: Option<WidgetList>,
    callbacks: HashMap<usize, StoredCallback>,
    next_callback_id: usize,
}

impl WidgetParser {
    pub fn produce_list<D>(gui_builder: &D) -> WidgetList
    where
        D: GuiBuilder,
    {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            widget_parser.borrow_mut().output = Some(WidgetList::new());
        });
        gui_builder.build();
        let mut result = None;
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            result = widget_parser.borrow_mut().output.take();
        });
        result.unwrap()
    }
    pub fn parse_push<T>(w: T)
    where
        T: Widget + 'static,
    {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.parse_push_widget(Box::new(w));
            }
        });
    }
    pub fn parse_pop() {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.parse_pop();
            }
        });
    }
    pub fn enter_builder(cache_id: u64) {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.enter_builder(WidgetBuilderCache { cache_id });
            }
        });
    }
    pub fn leave_builder() {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.leave_builder();
            }
        });
    }
    pub fn register_param<T>(_param: &T)
    where
        T: std::fmt::Debug,
    {
        // println!("Registered param {:?}", param);
    }

    pub fn add_callback<T>(cb: StoredCallback) -> GuiCallback<T>
    where
        T: Widget,
    {
        let mut id: usize = 0;
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            id = widget_parser.next_callback_id;
            widget_parser.next_callback_id += 1;

            widget_parser.callbacks.insert(id, cb);
        });
        GuiCallback::new(id)
    }
    pub(crate) fn remove_callback(id: usize) {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            widget_parser.callbacks.remove(&id);
        });
    }

    pub fn execute_callback<T>(
        cb: &GuiCallback<T>,
        data: &mut dyn Any,
        instance: &T,
        sender: &mut StaticWorld,
    ) where
        T: Widget,
    {
        if let Some(id) = *cb.callback_id {
            WIDGETPARSER_INSTANCE.with(|widget_parser| {
                let widget_parser = widget_parser.borrow();
                if let Some(callback) = widget_parser.callbacks.get(&id) {
                    match callback {
                        StoredCallback::ZeroParam(f) => f(),
                        StoredCallback::OneParam(f) => f(data),
                        StoredCallback::TwoParams(f) => f(data, instance),
                        StoredCallback::ThreeParams(f) => f(data, instance, sender),
                    }
                }
            });
        }
    }
}
