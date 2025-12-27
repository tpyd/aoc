pub fn run(input: &str) -> (u32, u32) {
    let trimmed_input = input.trim_end();

    let mut num: u32 = 0;
    let mut part1 = 0;
    let mut part2 = 0;

    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut d: u32;

    let k: [u32; 64] = [
         0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
         0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
         0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
         0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
         0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
         0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
         0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
         0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
         0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
         0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
         0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
         0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
         0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
         0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
         0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
         0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
    ];

    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
    ];

    let mut hash_input = [0u8; 64];
    let mut buf = [0u8; 10];
    let mut digits = [0u8; 10];

    let mut offset = trimmed_input.len();
    hash_input[0..offset].copy_from_slice(trimmed_input.as_bytes());

    loop {
        offset = trimmed_input.len();

        let mut n = num;
        let mut i = 0;
        loop {
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;

            if n == 0 {
                break;
            }
        }

        for j in 0..i {
            buf[j] = digits[i - 1 - j];
        }

        hash_input[offset..offset + i].copy_from_slice(&buf[..i]);
        offset += i;

        hash_input[offset] = 0x80;

        let input_length: u64 = (trimmed_input.len() + i) as u64 * 8;
        hash_input[56..64].copy_from_slice(&input_length.to_le_bytes());

        a = 0x67452301;
        b = 0xefcdab89;
        c = 0x98badcfe;
        d = 0x10325476;

        for chunk in hash_input.chunks(64) {
            let mut da = a;
            let mut db = b;
            let mut dc = c;
            let mut dd = d;

            let mut m = [0u32; 16];
            for (i, m_value) in m.iter_mut().enumerate() {
                let bytes = &chunk[i * 4 .. (i + 1) * 4];
                *m_value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            }

            for i in 0..64 {
                let mut f: u32;
                let g: usize;

                if i <= 15 {
                    f = (db & dc) | (!db & dd);
                    g = i;
                } else if i <= 31 {
                    f = (dd & db) | (!dd & dc);
                    g = (5 * i + 1) % 16;
                } else if i <= 47 {
                    f = db ^ dc ^ dd;
                    g = (3 * i + 5) % 16;
                } else {
                    f = dc ^ (db | !dd);
                    g = (7 * i) % 16;
                }

                f = f.wrapping_add(da).wrapping_add(k[i]).wrapping_add(m[g]);
                da = dd;
                dd = dc;
                dc = db;
                db = db.wrapping_add(f.rotate_left(s[i]));
            }

            a = a.wrapping_add(da);
            b = b.wrapping_add(db);
            c = c.wrapping_add(dc);
            d = d.wrapping_add(dd);
        }

        let digest = &a.to_le_bytes();

        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0 && part1 == 0 {
            part1 = num;
        }

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 && part2 == 0 {
            part2 = num;
        }

        if part1 != 0 && part2 != 0 {
            break;
        }

        num += 1;
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("abcdef");
        assert_eq!(part1, 609043);

        let (part1, _) = run("pqrstuv");
        assert_eq!(part1, 1048970);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2015, 4);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 282749);
        assert_eq!(part2, 9962624);
    }
}
