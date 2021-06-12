use sqlite;
use druid::{im::Vector, Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct FamilyData {
    pub head_of_family: String,
    pub members: String,
}



pub fn get_family_data_from_db() -> Vector<FamilyData> {
    let connection = sqlite::open("sqlite.db").unwrap();
    let statement = format!(r#"
    select hof.hof,  m.members from (select famid, group_concat(member_name, '{delimiter}') as members
    from members group by famid) as m left join
    (select famid, member_name as hof from members where rltshp = "Self") as hof
    on m.famid = hof.famid;
    "#, delimiter=", ");
    let mut result: Vector<FamilyData> = Vector::default();
    connection.iterate(format!("{}", statement), |row| {
        let family = row.iter().map(
            |x| match x.1 {
                None => "NOt FOUND".to_string(),
                Some(v) => v.to_string()
            }
        ).collect::<Vec<_>>(); // x.0 for selectinn first item in tuple inside list   | ).collect::<Vec<_>>();
        println!("{:?}", family);
        result.push_back(FamilyData { 
            head_of_family : family.get(0).unwrap().to_string(),
            members: family.get(1).unwrap().to_string(),
        });
        true
    })
    .unwrap();
    result
}

// fn get_family_data_from_db() -> Vec<(String, String)>{
//     let connection = sqlite::open("sqlite.db").unwrap();
//     let mut result = Vec::new();
//     let statement = format!(r#"
//     select hof.hof,  m.members from (select famid, group_concat(member_name, '{delimiter}') as members
//     from members group by famid) as m left join
//     (select famid, member_name as hof from members where rltshp = "Self") as hof
//     on m.famid = hof.famid;
//     "#, delimiter=", ");
//     connection.iterate(format!("{}", statement), |row| {
//         let family = row.iter().map(
//             |x| match x.1 {
//                 None => "NOt FOUND".to_string(),
//                 Some(v) => v.to_string()
//             }
//         ).collect::<Vec<_>>(); // x.0 for selectinn first item in tuple inside list   | ).collect::<Vec<_>>();
//         println!("{:?}", family);
//         result.push(
//             (
//             family.get(0).unwrap().to_string(), // hof
//             family.get(1).unwrap().to_string(), // members string comma separated
//             )
//         );
//         true
//     })
//     .unwrap();
//     Ok(result)
// }