use druid::im::{Vector, }; // vector
use druid::{Data, Lens};
use std::sync::Arc;
use crate::data::family_data::Family;

#[derive(Clone, Data, Lens, Debug)]
pub struct AppState { 
    pub window_title : String,
    pub vector: Vector<u32>,
    pub families: Arc<Vec<Family>>,
    pub selected: Option<usize>,
    pub checkbox_data: bool,
}
impl AppState {
    pub fn custom_function(&mut self) {
        println!("custom function");
    }
    // pub fn get_selected_family_view(&mut self) {
    //     self.families = 
    // }
}

