pub fn generate_audio(
    sound_reg: &[u8; 8],
    master_count: &mut u8,
    vibrato_clock: &mut u16,
    noise_clock: &mut u8,
    noise_state: &mut u16,
    a_count: &mut u8, a_state: &mut u8,
    b_count: &mut u8, b_state: &mut u8,
    c_count: &mut u8, c_state: &mut u8,
    bitswap: &[u8; 256],
    output: &mut [i16],
) {
    let sample_count = output.len();
    let mut idx = 0;

    while idx < sample_count {
        // Compute current sample
        let mut cur: i32 = 0;
        if *a_state != 0 { cur += (sound_reg[6] & 0x0f) as i32; }
        if *b_state != 0 { cur += (sound_reg[6] >> 4) as i32; }
        if *c_state != 0 { cur += (sound_reg[5] & 0x0f) as i32; }

        // Noise AM
        if (sound_reg[5] & 0x20) != 0 && (*noise_state & 0x4000) != 0 {
            cur += (sound_reg[7] >> 4) as i32;
        }

        // Scale to i16 range — max cur is 15+15+15+15=60, scale to ~32767
        let sample = ((cur * 32767) / 60) as i16;

        // Fill output (stereo: left and right)
        output[idx] = sample;

        // Clock noise
        *noise_clock = noise_clock.wrapping_add(1);
        if *noise_clock >= 64 {
            *noise_state = (*noise_state << 1) 
                | (!(((*noise_state >> 14) ^ (*noise_state >> 13)) & 1) & 1);
            *noise_clock -= 64;
            *vibrato_clock = vibrato_clock.wrapping_add(1);
        }

        // Clock master oscillator
        *master_count = master_count.wrapping_add(1);
        if *master_count == 0 {
            *master_count = !sound_reg[0];

            if (sound_reg[5] & 0x10) == 0 {
                // Vibrato mode
                if ((*vibrato_clock >> (sound_reg[4] >> 6)) & 0x0200) == 0 {
                    *master_count = master_count.wrapping_add(sound_reg[4] & 0x3f);
                }
            } else {
                // Noise mode
                *master_count = master_count.wrapping_add(
                    bitswap[(*noise_state >> 7) as usize & 0xff] & sound_reg[7]
                );
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

        idx += 1;
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