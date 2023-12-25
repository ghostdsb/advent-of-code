extern crate dotenv;

use day_22::part1::{create_tower};
use dotenv::dotenv;
use nannou::{prelude::*, rand};

fn main() {
    dotenv().ok();

    nannou::sketch(view).run();

}

fn view(app: &App, frame: Frame) {
    let colors: Vec<Srgb<u8>> = vec![
    ALICEBLUE,	
    ANTIQUEWHITE,	
    AQUA,	
    AQUAMARINE,	
    AZURE,	
    BEIGE,	
    BISQUE,	
    BLACK,	
    BLANCHEDALMOND,	
    BLUE,	
    BLUEVIOLET,	
    BROWN,	
    BURLYWOOD,	
    CADETBLUE,	
    CHARTREUSE,	
    CHOCOLATE,	
    CORAL,	
    CORNFLOWERBLUE,	
    CORNSILK,	
    CRIMSON,	
    CYAN,	
    DARKBLUE,	
    DARKCYAN,	
    DARKGOLDENROD,	
    DARKGRAY,	
    DARKGREEN,	
    DARKGREY,	
    DARKKHAKI,	
    DARKMAGENTA,	
    DARKOLIVEGREEN,	
    DARKORANGE,	
    DARKORCHID,	
    DARKRED,	
    DARKSALMON,	
    DARKSEAGREEN,	
    DARKSLATEBLUE,	
    DARKSLATEGRAY,	
    DARKSLATEGREY,	
    DARKTURQUOISE,	
    DARKVIOLET,	
    DEEPPINK,	
    DEEPSKYBLUE,	
    DIMGRAY,	
    DIMGREY,	
    DODGERBLUE,	
    FIREBRICK,	
    FLORALWHITE,	
    FORESTGREEN,	
    FUCHSIA,	
    GAINSBORO,	
    GHOSTWHITE,	
    GOLD,	
    GOLDENROD,	
    GRAY,	
    GREEN,	
    GREENYELLOW,	
    GREY,	
    HONEYDEW,	
    HOTPINK,	
    INDIANRED,	
    INDIGO,	
    IVORY,	
    KHAKI,	
    LAVENDER,	
    LAVENDERBLUSH,	
    LAWNGREEN,	
    LEMONCHIFFON,	
    LIGHTBLUE,	
    LIGHTCORAL,	
    LIGHTCYAN,	
    LIGHTGOLDENRODYELLOW,	
    LIGHTGRAY,	
    LIGHTGREEN,	
    LIGHTGREY,	
    LIGHTPINK,	
    LIGHTSALMON,	
    LIGHTSEAGREEN,	
    LIGHTSKYBLUE,	
    LIGHTSLATEGRAY,	
    LIGHTSLATEGREY,	
    LIGHTSTEELBLUE,	
    LIGHTYELLOW,	
    LIME,	
    LIMEGREEN,	
    LINEN,	
    MAGENTA,	
    MAROON,	
    MEDIUMAQUAMARINE,	
    MEDIUMBLUE,	
    MEDIUMORCHID,	
    MEDIUMPURPLE,	
    MEDIUMSEAGREEN,	
    MEDIUMSLATEBLUE,	
    MEDIUMSPRINGGREEN,	
    MEDIUMTURQUOISE,	
    MEDIUMVIOLETRED,	
    MIDNIGHTBLUE,	
    MINTCREAM,	
    MISTYROSE,	
    MOCCASIN,	
    NAVAJOWHITE,	
    NAVY,	
    OLDLACE,	
    OLIVE,	
    OLIVEDRAB,	
    ORANGE,	
    ORANGERED,	
    ORCHID,	
    PALEGOLDENROD,	
    PALEGREEN,	
    PALETURQUOISE,	
    PALEVIOLETRED,	
    PAPAYAWHIP,	
    PEACHPUFF,	
    PERU,	
    PINK,	
    PLUM,	
    POWDERBLUE,	
    PURPLE,	
    REBECCAPURPLE,	
    RED,	
    ROSYBROWN,	
    ROYALBLUE,	
    SADDLEBROWN,	
    SALMON,	
    SANDYBROWN,	
    SEAGREEN,	
    SEASHELL,	
    SIENNA,	
    SILVER,	
    SKYBLUE,	
    SLATEBLUE,	
    SLATEGRAY,	
    SLATEGREY,	
    SNOW,	
    SPRINGGREEN,	
    STEELBLUE,	
    TAN,	
    TEAL,	
    THISTLE,	
    TOMATO,	
    TURQUOISE,	
    VIOLET,	
    WHEAT,	
    WHITE,	
    WHITESMOKE,	
    YELLOW,	
    YELLOWGREEN,
    ];
    let input = include_str!("../../input/test/input-1.txt");
    // let draw = app.draw();
    
    create_tower(&input).iter().for_each(|block|{
        let w = block.end2.0 - block.end1.0 + 1;
        let h = block.end2.2 - block.end1.2 + 1;
        let x_y = (
            block.end1.0,
            block.end1.2,
        );
        let draw = app.draw();
        
        let index = (rand::random::<f32>() * colors.len() as f32).floor() as usize;
        let color = colors[index];
        draw
        .translate(Vec3::new(-500., -500., 0.))
        .scale(100.)
        .rect()
        .color(color)
        .w(w as f32)
        .h(h as f32)
        .x_y(x_y.0 as f32 , x_y.1 as f32 );
        draw.to_frame(app, &frame).unwrap();
    });
    // draw.to_frame(app, &frame).unwrap();
}
