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
    const CHIP_CLOCK: u32 = 1_789_772; // full Z80 clock rate, not /4
    const SAMPLE_RATE: u32 = 48000;
    const SAMPLE_SCALE: f32 = 1.0 / 60.0;

    for sample in output.iter_mut() {
        // Compute chip cycles for this output sample
        *chip_remainder += CHIP_CLOCK;
        let cycles = *chip_remainder / SAMPLE_RATE;
        *chip_remainder %= SAMPLE_RATE;

        // Sample the current tone states for output
        let mut cur: i32 = 0;
        let vol_a = (sound_reg[6] >> 4) as i32;    // was & 0x0f
        let vol_b = (sound_reg[6] & 0x0f) as i32;  // was >> 4
        let vol_c = (sound_reg[5] & 0x0f) as i32;
        let vol_n = (sound_reg[7] >> 4)   as i32;

        cur += if *a_state != 0 { vol_a } else { -vol_a };
        cur += if *b_state != 0 { vol_b } else { -vol_b };
        cur += if *c_state != 0 { vol_c } else { -vol_c };
        if (sound_reg[5] & 0x20) != 0 {
            cur += if (*noise_state & 0x4000) != 0 { vol_n } else { -vol_n };
        }
        *sample = (cur as f32 * SAMPLE_SCALE * 32767.0) as i16;

        // Clock noise counter — may fire once per sample batch
        *noise_clock = noise_clock.wrapping_add(cycles as u8);
        if *noise_clock >= 64 {
            *noise_state = (*noise_state << 1)
                | (!(((*noise_state >> 14) ^ (*noise_state >> 13)) & 1) & 1);
            *noise_clock -= 64;
            *vibrato_clock = vibrato_clock.wrapping_add(1);
        }

        // Clock master oscillator — loop to handle multiple overflows per sample
        let mut remaining = cycles;
        while remaining > 0 {
            let to_overflow = (256u32 - *master_count as u32).min(remaining);
            remaining -= to_overflow;
            *master_count = master_count.wrapping_add(to_overflow as u8);

            if *master_count == 0 {
                *master_count = !sound_reg[0];

                if (sound_reg[5] & 0x10) == 0 {
                    if ((*vibrato_clock >> (sound_reg[4] >> 6)) & 0x0200) == 0 {
                        *master_count = master_count.wrapping_add(sound_reg[4] & 0x3f);
                    }
                } else {
                    *master_count = master_count
                        .wrapping_add(bitswap[(*noise_state >> 7) as usize & 0xff] & sound_reg[7]);
                }

                *a_count = a_count.wrapping_add(1);
                if *a_count == 0 { *a_state ^= 1; *a_count = !sound_reg[1]; }

                *b_count = b_count.wrapping_add(1);
                if *b_count == 0 { *b_state ^= 1; *b_count = !sound_reg[2]; }

                *c_count = c_count.wrapping_add(1);
                if *c_count == 0 { *c_state ^= 1; *c_count = !sound_reg[3]; }
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
