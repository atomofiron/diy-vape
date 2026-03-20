
#[macro_export]
macro_rules! icon_bytes {
    ($input:expr) => {{
        use crate::r#macro::icon_bytes::{any, count, position};
        const WHITE: u8 = 0x88; // █ E29688
        const BLACK: u8 = 0x91; // ░ E29691
        const LF: u8 = b'\n'; // ignore the starting one \n
        const INPUT: &[u8] = $input.as_bytes();
        assert!(any(INPUT, WHITE), "no visible pixels");
        const IN_LEN: usize = INPUT.len();
        const WIDTH: usize = match INPUT[0] {
            LF => position(INPUT, LF, 1, IN_LEN),
            _ => position(INPUT, LF, 0, IN_LEN),
        } / 3; // 3 bytes per character
        const HEIGHT: usize = {
            let mut count = count(INPUT, LF);
            if INPUT[0] == LF {
                count -= 1;
            }
            if INPUT[IN_LEN - 1] != LF {
                count += 1;
            }
            count
        };
        const OUT_LEN: usize = HEIGHT * (WIDTH / 8 + if (WIDTH % 8 == 0) { 0 } else { 1 });
        let mut out = [0u8; OUT_LEN];
        let mut i = if INPUT[0] == LF { 1 } else { 0 };
        let mut index = 0;
        let mut offset = 7;
        while i < IN_LEN {
            let byte = INPUT[i];
            i += 1;
            if byte == BLACK {
            } else if byte == WHITE {
                out[index] |= 1 << offset;
            } else if byte == LF {
                index += 1;
                offset = 7;
                continue
            } else {
                continue
            }
            offset = if offset == 0 {
                index += 1;
                7
            } else {
                offset - 1
            };
        }
        out
    }};
}

pub const fn position(arr: &[u8], target: u8, start: usize, default: usize) -> usize {
    let mut i = start;
    while i < arr.len() {
        if arr[i] == target {
            return i - start;
        }
        i += 1;
    }
    return default
}

pub const fn any(arr: &[u8], target: u8) -> bool {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == target {
            return true;
        }
        i += 1;
    }
    return false
}

pub const fn count(arr: &[u8], target: u8) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == target {
            count += 1;
        }
        i += 1;
    }
    return count
}
