use core::{Consumer,Source,Node,NodeType};



#[repr(C)]
pub struct XHeyBasicFilter{
}
impl XHeyBasicFilter {
    pub fn new() -> Self {
        XHeyBasicFilter{
        }
    }
}

impl Node for XHeyBasicFilter{
    fn get_type_id() -> NodeType {
        NodeType::BasicFilter
    }
}



impl Source for XHeyBasicFilter {
    fn add_target<T:Consumer>(&self, target: &T, _location: u32){
        target.set_source(self,_location);
    }

    fn remove_all_targets(){

    }
}
impl Consumer for XHeyBasicFilter {
    fn set_source<T : Source>(&self, _source: &T, _location: u32){

    }

}