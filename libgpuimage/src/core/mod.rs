
pub trait DataSource {
    fn add_target<T : DataConsumer>(&self, target: &T, _location: u32);
    fn remove_all_targets();
}

pub trait DataConsumer {
    fn set_source<T : DataSource>(&self, _source: &T, _location: u32);
}


