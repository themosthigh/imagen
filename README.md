# Imagen

A simple ascii image rendering tool

<img width="1958" height="2162" alt="Screenshot From 2026-02-20 23-22-10" src="https://github.com/user-attachments/assets/a54af0a1-76e5-4f64-8689-73cdd0b24d5b" />

## Running the project

On the main directory

```bash
cargo run --bin imagen_cli <image_path>
```

## Capabilities

- [x] Rendering image as ascii based
- [x] Rendering pixel colors
- [x] Resizing image to terminal window size (retaining image aspect ratio)
- [x] Passing image path as arg
- [x] Flags
  - [x] Grayscale vs color
  - [x] Edge detection
- [ ] TUI mode
  - [ ] Image framing
  - [ ] File picker/browser
  - [ ] Repaint on screen resize

## Usage

You may need to enable color and edge detection

```bash
cargo run --bin imagen_cli <image_path> --color --edge
```

<img width="1958" height="2162" alt="Screenshot From 2026-02-20 23-19-51" src="https://github.com/user-attachments/assets/90e64f6c-552d-458e-8216-e880e0430c80" />
