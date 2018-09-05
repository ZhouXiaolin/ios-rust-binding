use core::{DataConsumer,DataSource};

#[repr(C)]
pub struct XHeyBasicFilter{}


impl DataSource for XHeyBasicFilter {
    fn add_target<T:DataConsumer>(&self, target: &T, _location: u32){
        target.set_source(self,_location);
    }

    fn remove_all_targets(){

    }
}
impl DataConsumer for XHeyBasicFilter {
    fn set_source<T : DataSource>(&self, _source: &T, _location: u32){

    }

}