pub fn build_palette() -> Box<[u32; 512]> {
    let mut palette = Box::new([0u32; 512]);

    for color in 0..32usize {
        let angle = (color as f64 / 32.0) * (2.0 * std::f64::consts::PI);
        let ry = if color != 0 { 0.75 * angle.sin() } else { 0.0 };
        let by = if color != 0 { 1.15 * angle.cos() } else { 0.0 };

        for luma in 0..16usize {
            let y = luma as f64 / 15.0;

            let r = ((ry + y) * 255.0) as i32;
            let g = (((y - 0.299 * (ry + y) - 0.114 * (by + y)) / 0.587) * 255.0) as i32;
            let b = ((by + y) * 255.0) as i32;

            let r = r.clamp(0, 255) as u32;
            let g = g.clamp(0, 255) as u32;
            let b = b.clamp(0, 255) as u32;

            // XRGB8888 format
            palette[color * 16 + luma] = (r << 16) | (g << 8) | b;
        }
    }

    palette
}

pub fn render_frame(
    vram: &[u8; 0x10000],
    initial_colors: &[u8; 8],
    color_events: &[(u32, usize, u8)],
    horcb: u8,
    verbl: u8,
    cycles_per_frame: u32,
    palette: &[u32; 512],
    output: &mut [u32],
) {
    let screen_lines = (verbl / 2) as usize;
    let screen_lines = screen_lines.min(102);
    let boundary_pixel = (horcb as usize & 0x3f) * 4;

    let mut colors = *initial_colors;

    let cycles_per_scanline = cycles_per_frame / screen_lines.max(1) as u32;
    let mut event_idx = 0;

    for y in 0..102usize {
        let line_start = y * 160;

        if y >= screen_lines {
            for x in 0..160usize {
                output[line_start + x] = 0;
            }
            continue;
        }

        let scanline_start_cycle = y as u32 * cycles_per_scanline;
        while event_idx < color_events.len() && color_events[event_idx].0 <= scanline_start_cycle {
            let (_, reg, val) = color_events[event_idx];
            colors[reg] = val;
            event_idx += 1;
        }

        let fb_offset = 0x4000 + y * 40;

        for byte_col in 0..40usize {
            let byte = vram[fb_offset + byte_col];
            let base_pixel = byte_col * 4;

            for bit_pair in 0..4usize {
                let pixel_x = base_pixel + bit_pair;
                let shift = 6 - (bit_pair * 2);
                let pix = ((byte >> shift) & 0x03) as usize;

                let color_base = if pixel_x < boundary_pixel { 4 } else { 0 };
                let color_reg = colors[color_base + pix] as usize;

                let colordata = color_reg << 1;
                let hue = (colordata >> 4) & 0x1f;
                let luma = colordata & 0x0f;

                let palette_index = hue * 16 + luma;
                output[line_start + pixel_x] = palette[palette_index];
            }
        }
    }
}
