use std::any::Any;
use gui::{WidgetList, PostBox, GuiCallback, Widget, WidgetBuilderCache};
use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt::Debug;

thread_local! {
    static WIDGETPARSER_INSTANCE: RefCell<WidgetParser> = RefCell::new(WidgetParser::default());
}

enum StoredCallback {
    ZeroParam(Box<dyn Fn()>),
    OneParam(Box<dyn Fn(&mut dyn Any)>),
    TwoParams(Box<dyn Fn(&mut dyn Any, &dyn Any)>),
    ThreeParams(Box<dyn Fn(&mut dyn Any, &dyn Any, &mut PostBox)>),
}

#[derive(Default)]
pub struct WidgetParser {
    output: Option<WidgetList>,
    callbacks: HashMap<usize, StoredCallback>,
    next_callback_id: usize,
}

impl WidgetParser {
    pub fn produce_list<F>(generator: F) -> WidgetList
    where
        F: Fn(),
    {
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            widget_parser.borrow_mut().output = Some(WidgetList::new());
        });
        generator();
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
        println!("Pushing ++");
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            if let Some(widgetlist) = &mut widget_parser.output {
                widgetlist.parse_push_widget(Box::new(w));
            }
        });
    }
    pub fn parse_pop() {
        println!("Popping -");
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
    fn add_callback(cb: StoredCallback) -> GuiCallback {
        let mut id: usize = 0;
        WIDGETPARSER_INSTANCE.with(|widget_parser| {
            let mut widget_parser = widget_parser.borrow_mut();
            id = widget_parser.next_callback_id;
            widget_parser.next_callback_id += 1;

            widget_parser.callbacks.insert(id, cb);
        });
        GuiCallback {
            callback_id: Some(id),
        }
    }
    pub fn make_callback3<F, D, S>(f: F) -> GuiCallback
    where
        F: 'static + Fn(&mut D, &S, &mut PostBox),
        D: 'static,
        S: 'static,
    {
        WidgetParser::add_callback(StoredCallback::ThreeParams(Box::new(
            move |input: &mut dyn Any, instance: &dyn Any, post: &mut PostBox| {
                f(
                    input.downcast_mut().unwrap(),
                    instance.downcast_ref().unwrap(),
                    post,
                );
            },
        )))
    }

    pub fn make_callback2<F, D, S>(f: F) -> GuiCallback
    where
        F: 'static + Fn(&mut D, &S),
        D: 'static,
        S: 'static,
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
    pub fn make_callback1<F, D>(f: F) -> GuiCallback
    where
        F: 'static + Fn(&mut D),
        D: 'static,
    {
        WidgetParser::add_callback(StoredCallback::OneParam(Box::new(
            move |input: &mut dyn Any| {
                f(input.downcast_mut().unwrap());
            },
        )))
    }
    pub fn make_callback0<F>(f: F) -> GuiCallback
    where
        F: 'static + Fn(),
    {
        WidgetParser::add_callback(StoredCallback::ZeroParam(Box::new(f)))
    }
    pub fn remove_callback(cb: &GuiCallback) {
        if let Some(id) = cb.callback_id {
            WIDGETPARSER_INSTANCE.with(|widget_parser| {
                let mut widget_parser = widget_parser.borrow_mut();
                widget_parser.callbacks.remove(&id);
            });
        }
    }
    pub fn execute_callback(
        cb: &GuiCallback,
        data: &mut dyn Any,
        instance: &dyn Any,
        sender: &mut PostBox,
    ) {
        if let Some(id) = cb.callback_id {
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
