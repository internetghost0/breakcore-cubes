use rand::Rng;
use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const COLORS: [raylib::prelude::Color; 145] = [
        Color::INDIANRED,
        Color::LIGHTCORAL,
        Color::SALMON,
        Color::DARKSALMON,
        Color::LIGHTSALMON,
        Color::CRIMSON,
        Color::RED,
        Color::FIREBRICK,
        Color::DARKRED,
        Color::PINK,
        Color::LIGHTPINK,
        Color::HOTPINK,
        Color::DEEPPINK,
        Color::MEDIUMVIOLETRED,
        Color::PALEVIOLETRED,
        Color::CORAL,
        Color::TOMATO,
        Color::ORANGERED,
        Color::DARKORANGE,
        Color::ORANGE,
        Color::GOLD,
        Color::YELLOW,
        Color::LIGHTYELLOW,
        Color::LEMONCHIFFON,
        Color::LIGHTGOLDENRODYELLOW,
        Color::PAPAYAWHIP,
        Color::MOCCASIN,
        Color::PEACHPUFF,
        Color::PALEGOLDENROD,
        Color::KHAKI,
        Color::DARKKHAKI,
        Color::LAVENDER,
        Color::THISTLE,
        Color::PLUM,
        Color::VIOLET,
        Color::ORCHID,
        Color::FUCHSIA,
        Color::MAGENTA,
        Color::MEDIUMORCHID,
        Color::MEDIUMPURPLE,
        Color::REBECCAPURPLE,
        Color::BLUEVIOLET,
        Color::DARKVIOLET,
        Color::DARKORCHID,
        Color::DARKMAGENTA,
        Color::PURPLE,
        Color::DARKPURPLE,
        Color::INDIGO,
        Color::SLATEBLUE,
        Color::DARKSLATEBLUE,
        Color::MEDIUMSLATEBLUE,
        Color::GREENYELLOW,
        Color::CHARTREUSE,
        Color::LAWNGREEN,
        Color::LIME,
        Color::LIMEGREEN,
        Color::PALEGREEN,
        Color::LIGHTGREEN,
        Color::MEDIUMSPRINGGREEN,
        Color::SPRINGGREEN,
        Color::MEDIUMSEAGREEN,
        Color::SEAGREEN,
        Color::FORESTGREEN,
        Color::GREEN,
        Color::DARKGREEN,
        Color::YELLOWGREEN,
        Color::OLIVEDRAB,
        Color::OLIVE,
        Color::DARKOLIVEGREEN,
        Color::MEDIUMAQUAMARINE,
        Color::DARKSEAGREEN,
        Color::LIGHTSEAGREEN,
        Color::DARKCYAN,
        Color::TEAL,
        Color::AQUA,
        Color::CYAN,
        Color::LIGHTCYAN,
        Color::PALETURQUOISE,
        Color::AQUAMARINE,
        Color::TURQUOISE,
        Color::MEDIUMTURQUOISE,
        Color::DARKTURQUOISE,
        Color::CADETBLUE,
        Color::STEELBLUE,
        Color::LIGHTSTEELBLUE,
        Color::POWDERBLUE,
        Color::LIGHTBLUE,
        Color::SKYBLUE,
        Color::LIGHTSKYBLUE,
        Color::DEEPSKYBLUE,
        Color::DODGERBLUE,
        Color::CORNFLOWERBLUE,
        Color::ROYALBLUE,
        Color::BLUE,
        Color::MEDIUMBLUE,
        Color::DARKBLUE,
        Color::NAVY,
        Color::MIDNIGHTBLUE,
        Color::CORNSILK,
        Color::BLANCHEDALMOND,
        Color::BISQUE,
        Color::NAVAJOWHITE,
        Color::WHEAT,
        Color::BURLYWOOD,
        Color::TAN,
        Color::ROSYBROWN,
        Color::SANDYBROWN,
        Color::GOLDENROD,
        Color::DARKGOLDENROD,
        Color::PERU,
        Color::CHOCOLATE,
        Color::SADDLEBROWN,
        Color::SIENNA,
        Color::BROWN,
        Color::DARKBROWN,
        Color::MAROON,
        Color::WHITE,
        Color::SNOW,
        Color::HONEYDEW,
        Color::MINTCREAM,
        Color::AZURE,
        Color::ALICEBLUE,
        Color::GHOSTWHITE,
        Color::WHITESMOKE,
        Color::SEASHELL,
        Color::BEIGE,
        Color::OLDLACE,
        Color::FLORALWHITE,
        Color::IVORY,
        Color::ANTIQUEWHITE,
        Color::LINEN,
        Color::LAVENDERBLUSH,
        Color::MISTYROSE,
        Color::GAINSBORO,
        Color::LIGHTGRAY,
        Color::SILVER,
        Color::DARKGRAY,
        Color::GRAY,
        Color::DIMGRAY,
        Color::LIGHTSLATEGRAY,
        Color::SLATEGRAY,
        Color::DARKSLATEGRAY,
        Color::BLACK,
        Color::BLANK,
        Color::RAYWHITE,
    ];
struct Cube {
    pos: Vector2,
    direction: Vector2,
    size: Vector2,
    speed: f32,
    color: Color,
}

impl Cube {
    pub const fn new(
        pos_x: f32,
        pos_y: f32,
        direction: Vector2,
        size: Vector2,
        speed: f32,
        color: Color,
    ) -> Self {
        Self {
            pos: Vector2::new(pos_x, pos_y),
            direction,
            size,
            speed,
            color,
        }
    }
    pub const fn zero() -> Self {
        Self {
            pos: Vector2::zero(),
            direction: Vector2::zero(),
            size: Vector2::zero(),
            speed: 0.0,
            color: Color::WHITE,
        }
    }
}

fn render_cube(d: &mut RaylibDrawHandle, cube: &mut Cube) {
    cube.pos.x += cube.speed * cube.direction.x;
    cube.pos.y += cube.speed * cube.direction.y;
    if cube.pos.x < 0.0 {
        cube.direction.x = 1.0;
    }
    if cube.pos.y < 0.0 {
        cube.direction.y = 1.0;
    }
    if cube.pos.x + cube.size.x > WIDTH as f32 {
        cube.direction.x = -1.0;
    }
    if cube.pos.y + cube.size.y > HEIGHT as f32 {
        cube.direction.y = -1.0;
    }
    d.draw_rectangle_v(cube.pos, cube.size, cube.color);
}

fn main() {

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Cubes").build();
    rl.set_target_fps(120);

    let mut cubes: Vec<Cube> = vec![Cube::new(
        0.0,
        0.0,
        Vector2::one(),
        Vector2::new(30.0, 30.0),
        15.0,
        Color::PINK,
    )];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // render screen border
        let border_color = Color::GRAY;
        d.draw_line(0, 0, WIDTH, 0, border_color);
        d.draw_line(0, 0, 1, HEIGHT, border_color);
        d.draw_line(WIDTH, HEIGHT, WIDTH, 0, border_color);
        d.draw_line(WIDTH, HEIGHT, 0, HEIGHT, border_color);

        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) || d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mut cube = Cube::zero();
            cube.pos.x = rand::thread_rng().gen_range(0..WIDTH) as f32;
            cube.pos.y = rand::thread_rng().gen_range(0..HEIGHT) as f32;
            cube.direction.x = if rand::random() { 1.0 } else { -1.0 };
            cube.direction.y = if rand::random() { 1.0 } else { -1.0 };
            cube.size = Vector2::new(30.0, 30.0);
            cube.speed = (rand::thread_rng().gen_range(100..400) as f32 / 10.0) as f32; // 10.0..40.0
            cube.color = *COLORS
                .get(rand::thread_rng().gen_range(0..COLORS.len()))
                .unwrap_or(&Color::WHITE);
            cubes.push(cube);
        }

        if d.is_key_pressed(KeyboardKey::KEY_C) {
            println!("Cubes count: {}", cubes.len());
        }
        if d.is_key_pressed(KeyboardKey::KEY_R) {
            cubes.clear();
        }

        for cube in &mut cubes {
            render_cube(&mut d, cube);
        }
        d.draw_fps(10, 10);
    }
}
