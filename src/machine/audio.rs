pub fn generate_audio(
    sound_reg: &[u8; 8],
    master_count: &mut u8,
    vibrato_clock: &mut u16,
    noise_clock: &mut u8,
    noise_state: &mut u16,
    a_count: &mut u8,
    a_state: &mut u8,
    b_count: &mut u8,
    b_state: &mut u8,
    c_count: &mut u8,
    c_state: &mut u8,
    bitswap: &[u8; 256],
    chip_remainder: &mut u32,
    output: &mut [i16],
) {
    const CHIP_CLOCK: u32 = 1_789_772 / 4; // 447443 Hz
    const SAMPLE_RATE: u32 = 48000;
    const SAMPLE_SCALE: f32 = 1.0 / 60.0;

    for sample in output.iter_mut() {
        // Compute how many chip cycles to run for this output sample
        *chip_remainder += CHIP_CLOCK;
        let cycles = *chip_remainder / SAMPLE_RATE;
        *chip_remainder %= SAMPLE_RATE;

        let mut cur: i32 = 0;
        if *a_state != 0 { cur += (sound_reg[6] & 0x0f) as i32; }
        if *b_state != 0 { cur += (sound_reg[6] >> 4)   as i32; }
        if *c_state != 0 { cur += (sound_reg[5] & 0x0f) as i32; }
        if (sound_reg[5] & 0x20) != 0 && (*noise_state & 0x4000) != 0 {
            cur += (sound_reg[7] >> 4) as i32;
        }

        *sample = (cur as f32 * SAMPLE_SCALE * 32767.0) as i16;

        // Clock noise — advances by `cycles`, wraps at 64
        *noise_clock = noise_clock.wrapping_add(cycles as u8);
        if *noise_clock >= 64 {
            *noise_state = (*noise_state << 1)
                | (!(((*noise_state >> 14) ^ (*noise_state >> 13)) & 1) & 1);
            *noise_clock -= 64;
            *vibrato_clock = vibrato_clock.wrapping_add(1);
        }

        // Clock master oscillator — advances by `cycles`, wraps at 256
        let (new_master, wrapped) = master_count.overflowing_add(cycles as u8);
        *master_count = new_master;
        if wrapped || new_master == 0 {
            *master_count = !sound_reg[0];

            if (sound_reg[5] & 0x10) == 0 {
                // Vibrato mode
                if ((*vibrato_clock >> (sound_reg[4] >> 6)) & 0x0200) == 0 {
                    *master_count = master_count.wrapping_add(sound_reg[4] & 0x3f);
                }
            } else {
                // Noise mode
                *master_count = master_count
                    .wrapping_add(bitswap[(*noise_state >> 7) as usize & 0xff] & sound_reg[7]);
            }

            // Clock tone A
            *a_count = a_count.wrapping_add(1);
            if *a_count == 0 {
                *a_state ^= 1;
                *a_count = !sound_reg[1];
            }
            // Clock tone B
            *b_count = b_count.wrapping_add(1);
            if *b_count == 0 {
                *b_state ^= 1;
                *b_count = !sound_reg[2];
            }
            // Clock tone C
            *c_count = c_count.wrapping_add(1);
            if *c_count == 0 {
                *c_state ^= 1;
                *c_count = !sound_reg[3];
            }
        }
    }
}

pub fn build_bitswap() -> [u8; 256] {
    let mut table = [0u8; 256];
    for i in 0..256usize {
        let b = i as u8;
        table[i] = b.reverse_bits();
    }
    table
}
