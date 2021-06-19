use druid::{Widget, WidgetExt, Env};
use druid::widget::{Label, Flex};

use crate::colors::THEME;
use crate::data::appdata::AppState;
use crate::data::family_details_data::get_family_details_of_famid;

pub fn generate_main_view() -> impl Widget<AppState> {
    let famid = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).famid)
    );
    let hof = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).head_of_family)
    );
    let members = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).members)
    );
    let email = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).email)
    );
    let place = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).place)
    );
    let address = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).address)
    );
    Flex::column()
        .with_child(famid)
        .with_child(hof)
        .with_child(members)
        .with_child(email)
        .with_child(place)
        .with_child(address)
    .background(THEME.primary_dark)
        
}

