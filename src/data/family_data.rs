use sqlite;
use druid::{Data, Lens}; 

#[derive(Clone, Data, Lens)]
pub struct Family {
    pub famid: u32,
    pub head_of_family: String,

}

impl Family {
    pub fn new(famid: u32, head_of_family: impl Into<String>) -> Self {
        let head_of_family =head_of_family.into();
        Self {head_of_family, famid }
    }
}

pub fn get_family_data_from_db() -> Vec<Family> {
    let connection = sqlite::open("sqlite.db").unwrap();
    let statement = format!(r#"
    select hof.famid, hof.hof,  m.members from (select famid, group_concat(member_name, '{delimiter}') as members
    from members group by famid) as m left join
    (select famid, member_name as hof from members where rltshp = "Self") as hof
    on m.famid = hof.famid;
    "#, delimiter=", ");
    let mut result: Vec<Family> = Vec::default();
    connection.iterate(format!("{}", statement), |row| {
        let family = row.iter().map(
            |x| match x.1 {
                None => "5555".to_string(),
                Some(v) => v.to_string()
            }
        ).collect::<Vec<_>>(); 
        result.push(Family { 
            famid : family.get(0).unwrap().to_string().parse::<u32>().unwrap(), 
            head_of_family : family.get(1).unwrap().to_string(),
        });
        true
    })
    .unwrap();
    result
}
