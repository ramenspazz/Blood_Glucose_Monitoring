/// # Purpose
/// Defines a method to read a line up to the first space, then if possible, compute a f64 number from the read data and return it
pub fn readin(mut file: &mut std::fs::File) -> Option<f64> {
    let _char_ptr: *mut u8 = std::ptr::null_mut();
    let mut u8_buf: [u8; 1] = [0u8; 1];
    let mut num_char_before_decimal: u16 = 0u16;
    let mut num_char_after_decimal: u16 = 0u16;
    let mut reading_dec: bool = false;
    let mut num_arr: [u32; 65535] = [0; 65535];

    while let Ok(read_bytes) = std::io::Read::read(&mut file, &mut u8_buf) {
        if read_bytes == 0 {
            break;
        }

        match u8_buf.get(0) {
            Some(char_val) => {
                if *char_val == 32u8 || *char_val == 10u8 {
                // If the character is a space or newline, stop adding digits and compute the final number
                    let mut final_number: f64 = 0.0;
                    let final_num = num_char_before_decimal + num_char_after_decimal;

                    for arr_index in 0..final_num {
                        let current_digit = *num_arr.get(arr_index as usize).unwrap() as f64;

                        if arr_index < num_char_before_decimal {
                            final_number = final_number * 10f64 + current_digit;
                        } else {
                            let decimal_place = (arr_index - num_char_before_decimal + 1) as f64;
                            final_number += current_digit / 10f64.powf(decimal_place);
                        }
                    }

                    return Some(final_number);
                } else if *char_val == 46u8 {
                // The character is a decimal point, mark that we are now processing decimals
                    reading_dec = true;
                    continue;
                } else if *char_val >= 48u8 && *char_val <= 57u8 {
                // The character is an ascii number and we can work with it
                    let current_digit = (*char_val - 48) as u32;

                    if reading_dec {
                        num_char_after_decimal += 1;
                    } else {
                        num_char_before_decimal += 1;
                    }

                    match num_arr.get_mut((num_char_before_decimal + num_char_after_decimal - 1) as usize) {
                        Some(value) => *value = current_digit,
                        None => panic!(),
                    }
                }
            }
            None => return None,
        }
    }

    None
}