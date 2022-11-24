use std::fs::File;

fn main() {
    let mut rom_path = None;

    for arg in std::env::args().skip(1) {
        match arg {
            _ => rom_path = Some(arg.clone()),
        }
    }

    let mut ws = ws_rs::Watara::new();

    let file = File::open(rom_path.unwrap_or("rom.sv".to_string())).unwrap();

    ws.load(file).unwrap();

    let mut window = minifb::Window::new(
        "Watara Supervision",
        160,
        160,
        minifb::WindowOptions {
            resize: true,
            scale: minifb::Scale::X4,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    ).unwrap();

    //window.set_cursor_visibility(false);

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = Vec::with_capacity(160 * 160);
    buffer.resize(160 * 160, 0);

    //let mut size = (0, 0);

    let mut noise;
    let mut carry;
    let mut seed = 0xbeefu32;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        //let new_size = (window.get_size().0, window.get_size().1);
        //if new_size != size {
        //    size = new_size;
        //    buffer.resize(size.0 * size.1, 0);
        //}

        for i in buffer.iter_mut() {
            noise = seed;
            noise >>= 3;
            noise ^= seed;
            carry = noise & 1;
            noise >>= 1;
            seed >>= 1;
            seed |= carry << 30;
            noise &= 0xFF;
            *i = (noise << 16) | (noise << 8) | noise;
        }

        window.get_keys().iter().for_each(|key| match key {
            minifb::Key::W => println!("holding w!"),
            minifb::Key::T => println!("holding t!"),
            _ => (),
        });

        window.get_keys_released().iter().for_each(|key| match key {
            minifb::Key::W => println!("released w!"),
            minifb::Key::T => println!("released t!"),
            _ => (),
        });

        window
            .update_with_buffer(&buffer, 160, 160)
            .unwrap();
    }
}
