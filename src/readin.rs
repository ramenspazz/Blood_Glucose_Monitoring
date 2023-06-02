/// # Purpose
/// Defines a method to read a line up to the first space, then if possible, compute a f64 number from the read data and return it
pub fn readin(mut file: &mut std::fs::File) -> Option<f64> {
    let _char_ptr: *mut u8 = std::ptr::null_mut();
    let mut u8_buf: [u8; 1] = [0u8; 1];
    let mut num_char_before_decimal: u16 = 0u16;
    let mut num_char_after_decimal: u16 = 0u16;
    let mut reading_dec: bool = false;
    let mut num_arr: [u32; 65535] = [0; 65535]; // max size is 65534, which is the maximum

    while let Ok(read_bytes) = std::io::Read::read(&mut file, &mut u8_buf) {
        if read_bytes == 0 {
            break;
        }
        // split the string into slices at spaces, setting each part to a number
        match u8_buf.get(0) {
            Some(char_val) => {
                if *char_val == 32u8 || *char_val == 10u8 {
                    // If the character is a space or newline, stop adding digits and compute the final number
                    let mut final_number: f64 = 0.0;
                    let final_num = num_char_before_decimal + num_char_after_decimal;
                    for arr_index in (0..final_num).rev() {
                        if arr_index < num_char_before_decimal {
                            // Processing digits before the decimal point
                            final_number +=
                                *num_arr.get((num_char_before_decimal - arr_index - 1) as usize)
                                    .unwrap() as f64
                                    * 10f64.powf(arr_index as f64);
                        } else {
                            // Processing digits after the decimal point
                            final_number +=
                                *num_arr.get((arr_index - num_char_before_decimal + 1) as usize)
                                    .unwrap() as f64
                                    / 10f64.powf((arr_index - num_char_before_decimal + 1) as f64);
                        }
                    }
                    return Some(final_number);
                } else if *char_val == 46u8 {
                    // The character is a decimal point, mark that we are now processing decimals
                    reading_dec = true;
                    continue;
                } else if *char_val >= 48u8 && *char_val <= 57u8 {
                    // The character is a number and we can work with it
                    if reading_dec == true {
                        num_char_after_decimal += 1;
                    } else {
                        num_char_before_decimal += 1;
                    }
                    match num_arr.get_mut((num_char_before_decimal + num_char_after_decimal - 1) as usize) {
                        Some(value) => {
                            *value = (*char_val - 48) as u32;
                        }
                        None => panic!(),
                    }
                }
            }
            None => return None,
        }
    }
    return None;
}

// // coerce to a &str : safety
// let s: Result<&str, str::Utf8Error> = unsafe {
//     // First, we build a &[u8]...
//     let slice = std::slice::from_raw_parts(u8_buf.as_mut_ptr(), read_bytes);
//     // ... and then convert that slice into a string slice
//     std::str::from_utf8(slice)
// };
// match s {
//     Ok(value) => print!("{}", &String::from(value)),
//     Err(_) => panic!(),
// }
// std::io::stdout()
//     .flush()
//     .unwrap();