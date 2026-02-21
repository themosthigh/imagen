use imagen::{
    cli, image_loader,
    my_image::{DrawArgs, draw_print},
};

fn main() {
    let args = cli::parse();
    let r = image_loader::loader(&args);

    draw_print(
        r,
        DrawArgs {
            color: args.color,
            edge: args.edge,
        },
    );
}
