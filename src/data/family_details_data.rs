use sqlite;
use druid::{Data, Lens}; 

#[derive(Clone, Data, Lens, Debug)]
pub struct FamilyDetails {
    pub famid: u32,
    pub head_of_family: String,
    pub members: String,
    pub email: String,
    pub place: String,
    pub address: String,

}

impl FamilyDetails {
    pub fn new(famid: u32, head_of_family: impl Into<String>) -> Self {
        let head_of_family =head_of_family.into();
        let members = "None".to_string();
        let email = "dummy@gmail.com".to_string();
        let place = "place".to_string();
        let address = "address here".to_string();
        Self {famid, head_of_family, members, email, place, address}
    }

    // pub fn get_details()
}

pub fn get_family_details_of_famid(famid:u32) -> FamilyDetails {
    let connection = sqlite::open("sqlite.db").unwrap();
    let statement = format!(r#"
    SELECT f.famid, hof.hof, m.mems, f.email, f."where", 
    ca.house_flat_no 
    || ', ' || ca.house_building_name 
    || ', ' || ca.street 
    || ', ' || ca.locality 
    || ', ' || ca.city
    || ', Pin: ' || ca.pin  
    as address

    FROM families as f 
    LEFT JOIN cur_addr as ca on f.famid = ca.famid
    LEFT JOIN nat_addr as na on f.famid = na.famid
    LEFT JOIN nat_parish as np on f.famid = np.famid
    LEFT JOIN (select famid, member_name as hof from members where rltshp = "Self") as hof
    on f.famid = hof.famid

    LEFT JOIN (select famid, group_concat(member_name, ', ') as mems from members where famid = {famid}) as m 
    on f.famid = m.famid
    WHERE f.famid = {famid}
    "#, famid=famid);
    let mut result: FamilyDetails = FamilyDetails {
        famid : 0, 
        head_of_family : "Empty".to_string(),
        members: "Empty".to_string(),
        email : "Empty".to_string(),
        place: "Empty".to_string(),
        address : "Empty".to_string(),
        // members: "Empty".to_string(),
    };
    connection.iterate(format!("{}", statement), |row| {
        // println!("{:?}", row);
        let family = row.iter().map(
            |x| match x.1 {
                None => "NOT FOUND".to_string(),
                Some(v) => //{println!("{:?}", v);
                    v.to_string()
            }
        ).collect::<Vec<_>>(); 
        // println!("{:?}", family);
        result = FamilyDetails { 
            famid : family.get(0).unwrap().to_string().parse::<u32>().unwrap_or(5555), 
            head_of_family : family.get(1).unwrap().to_string(),
            members: family.get(2).unwrap().to_string(),
            email: family.get(3).unwrap().to_string(),
            place: family.get(4).unwrap().to_string(),
            address: family.get(5).unwrap().to_string(),
        };
        true
    })
    .unwrap();
    result
}
