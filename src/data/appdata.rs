use druid::im::{Vector, vector};
use druid::{Data, Lens};
use crate::data::family_data::FamilyData;

#[derive(Clone, Data, Lens)]
pub struct AppState { 
    pub window_title : String,
    pub vector: Vector<u32>,
    pub families: Vector<FamilyData>,
    pub checkbox_data: bool,
}
impl AppState {
    pub fn custom_function(&mut self) {
        println!("custom function");
    }
}