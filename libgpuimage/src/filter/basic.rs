use core::{Consumer,Source,Node,NodeType,RenderNode};
use std::mem;


#[repr(C)]
pub struct XHeyBasicFilter{
    node : RenderNode
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_basic_filter() -> *mut XHeyBasicFilter {
    unsafe {mem::transmute(Box::new(XHeyBasicFilter::new()))}
}

impl XHeyBasicFilter {
    pub fn new() -> Self {
        XHeyBasicFilter{
            node:RenderNode::new(NodeType::BasicFilter)
        }
    }
}

impl Node for XHeyBasicFilter{
    fn get_type_name(&self) -> NodeType {
        NodeType::BasicFilter
    }
}



impl<'a> Source<'a> for XHeyBasicFilter {
    fn add_target(&self, target: &'a dyn Consumer, _location: u32) {
        target.set_source(self,_location);
    }

    fn remove_all_targets(&self){

    }
}
impl Consumer for XHeyBasicFilter {
    fn set_source(&self, _source: &dyn Source, _location: u32) {

    }

}