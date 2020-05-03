use std::ops::Shr;
use gui::WidgetParser;

#[macro_export]
macro_rules! impl_widget_building_for {
    ($t:ty) => {
        impl Neg for $t {
            type Output = WidgetAdder;
        
            fn neg(self) -> Self::Output {
                WidgetParser::parse_push(self);
                WidgetAdder{}
            }
        }
        
        impl Shr<()> for $t {
            type Output = ();
            
            fn shr(self, _rhs: ()) -> Self::Output {
                
            }
        }
    }
}

pub struct WidgetAdder {}

impl Drop for WidgetAdder {
    fn drop(&mut self) {
        WidgetParser::parse_pop();
    }
}

impl Shr<()> for WidgetAdder {
    type Output = ();

    fn shr(self, _rhs: ()) -> Self::Output {
        ()
    }
}

pub struct WidgetAdderLeaf {}

impl Drop for WidgetAdderLeaf {
    fn drop(&mut self) {
        WidgetParser::parse_pop();
    }
}

impl Shr<()> for WidgetAdderLeaf {
    type Output = WidgetAdderLeaf;

    fn shr(self, _rhs: ()) -> Self::Output {
        self
    }
}

impl Shr<WidgetAdder> for WidgetAdder {
    type Output = WidgetAdderLeaf;

    fn shr(self, _rhs: WidgetAdder) -> Self::Output {
        WidgetAdderLeaf{}
    }
}
