

pub fn get_family_view(famid: u32) {
    // println!("Selected family id is {}", famid);
    // println!("Switching to Family View of ID {}", famid);
    let result = crate::data::family_details_data::get_family_details_of_famid(famid);
    println!("{:?}", result)
}