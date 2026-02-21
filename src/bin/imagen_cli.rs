use imagen::{
    cli, image_loader,
    my_image::{DrawArgs, draw_print},
};

fn main() {
    let args = cli::parse();
    let r = image_loader::loader(&args);

    let color = args.color;
    let edge = args.edge;

    draw_print(r, DrawArgs { color, edge });
}
