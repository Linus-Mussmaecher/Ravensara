use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui, widgets};

pub async fn create_skin() -> Skin {
    let font = load_ttf_font("./resources/PixelOperator.ttf")
        .await
        .unwrap();

    let label_style = root_ui()
        .style_builder()
        .with_font(&font)
        .unwrap()
        .text_color(Color::from_rgba(20, 31, 37, 255))
        .font_size(40)
        .build();

    let window_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(include_bytes!("../resources/ui/tile-ui.png"), None)
                .unwrap(),
        )
        // .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
        // .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
        .build();

    // let button_style = root_ui()
    //     .style_builder()
    //     .background(
    //         Image::from_file_with_format(include_bytes!("../resources/ui/button.png"), None)
    //             .unwrap(),
    //     )
    //     .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
    //     .background_hovered(
    //         Image::from_file_with_format(
    //             include_bytes!("../resources/ui/button_hovered.png"),
    //             None,
    //         )
    //         .unwrap(),
    //     )
    //     .background_clicked(
    //         Image::from_file_with_format(
    //             include_bytes!("../resources/ui/button_hovered.png"),
    //             None,
    //         )
    //         .unwrap(),
    //     )
    //     .with_font(&font)
    //     .unwrap()
    //     .text_color(Color::from_rgba(180, 180, 100, 255))
    //     .font_size(40)
    //     .build();

    Skin {
        window_style,
        label_style,
        ..root_ui().default_skin()
    }
}
