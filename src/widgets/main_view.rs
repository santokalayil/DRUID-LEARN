use druid::{Widget, WidgetExt, Env};
use druid::widget::{Label, Flex, SizedBox,};

use crate::colors::THEME;
use crate::data::appdata::AppState;
use crate::data::family_details_data::get_family_details_of_famid;

pub struct MainView {
    pub family_view: bool
}

pub fn generate_main_view(state: &MainView) -> Box<dyn Widget<AppState>> {
    if state.family_view {
        let flex = SizedBox::new(generate_family_view());
        // let flex = flex.padding(8.0).border(druid::Color::grey(0.6), 2.0).rounded(5.0);//.lens(AppState::)
        flex.boxed()
    } 
    else {
        let flex = SizedBox::new(generate_overview());
        flex.boxed()
    }
    
        
}

fn generate_family_view() -> impl Widget<AppState> {

    // let hof = TextBox::<AppState>::(|x|"hi").with_placeholder(|x| "hi")
    //     // .lens(AppState)
    // //     |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).head_of_family)
    // // )
    //     // |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).head_of_family)
    // ;
    let txt_size = 12.0;
    let famid = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).famid)
    ).with_text_size(txt_size);
    let hof = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).head_of_family)
    ).with_text_size(txt_size);
    let members = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).members)
    ).with_text_size(txt_size);
    let email = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).email)
    ).with_text_size(txt_size);
    let place = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).place)
    ).with_text_size(txt_size);
    let address = Label::new(
        |data: &AppState, _env: &Env| format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).address)
    ).with_text_size(txt_size);

    let general_text = Flex::column()
        .with_spacer(20.0)
        .with_child(famid)
        .with_child(hof)
        .with_child(members)
        .with_child(email)
        .with_child(place)
        .with_child(address)
    // .background(druid::Color::GREEN) 
    // .padding(20.)
    ;
    // .lens(AppState.families)
    use druid::{ImageBuf,};
    use druid::widget::{Image, FillStrat,};
    use druid::piet::InterpolationMode;

    let fam_photo = Flex::column()
        .with_child(
                Image::new(     ImageBuf::from_data(include_bytes!("family_images\\model.jpg")).unwrap()    )
                .fill_mode(FillStrat::Fill)
                .interpolation_mode(InterpolationMode::Bilinear)      
        )
        
        .padding(8.0).border(THEME.primary_light, 2.0).rounded(5.0)
        .background(druid::Color::grey(0.6))
        ;
    let right_side_panel = Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("FAMILY PHOTO").padding(5.))
        .with_child(fam_photo.padding(10.))
        .with_flex_child(Label::new("BELOW OF PHOTO"), 1.0)
        ;
    // horizontal row
    let hr = | thickness, c |  Flex::row().background(c).expand_width().height(thickness);
    // ribbon for text display
    let rbn = | thickness, c |  
        Flex::column().with_child(
            Label::new(
                |data: &AppState, _env: &Env| 
                format!("{}", get_family_details_of_famid(data.families.get(data.selected.unwrap_or(0)).unwrap().famid,).head_of_family)
            ).with_text_size(20.0)
        )
        .background(c).expand_width().height(thickness)
        .center()
        ;
    let head_bar = rbn(30.0, druid::Color::from_rgba32_u32(0x0099ffff));

    let central_panel = Flex::column()
        .with_child(hr(2.0, druid::Color::WHITE)) // white bar
        .with_child(hr(0.5, druid::Color::GRAY)) // bar shadow
        .with_child(head_bar)
        // .with_child(hr(30.0, druid::Color::from_rgba32_u32(0x0099ffff)))
        .with_child(general_text)

    .background(druid::Color::BLACK)
    ;

    Flex::row()
        .with_flex_child(central_panel.expand(), 1.0)
        .with_default_spacer() //.paint(ctx: &mut PaintCtx, data: &T, env: &Env)
        .with_child(right_side_panel)
        .with_default_spacer()
    .background(THEME.primary_dark)
}

fn generate_overview() -> impl Widget<AppState> {
    Label::new("OVERVIEW")
}

