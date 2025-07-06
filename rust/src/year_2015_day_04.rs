pub fn run(input: &str) -> (u32, u32) {
    let trimmed_input = input.trim();
    let mut num = 0;
    let mut part1 = 0;
    let mut part2 = 0;

    loop {
        let hash_input = format!("{}{}", trimmed_input, num);
        let hash = md5(hash_input);

        if hash.starts_with("00000") && part1 == 0 {
            part1 = num;
        }

        if hash.starts_with("000000") && part2 == 0 {
            part2 = num;
        }

        if part1 != 0 && part2 != 0 {
            break;
        }

        num += 1;
    }

    (part1, part2)
}

fn md5(input: String) -> String {
    let mut a: u32 = 0x67452301;
    let mut b: u32 = 0xefcdab89;
    let mut c: u32 = 0x98badcfe;
    let mut d: u32 = 0x10325476;

    let k: Vec<u32> = vec![
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

    let s: Vec<u32> = vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
    ];

    let mut bytes = input.as_bytes().to_vec();
    bytes.push(0x80);

    while bytes.len() % 64 < 56 {
        bytes.push(0);
    }

    let original_length = input.len() as u64 * 8;
    bytes.extend(original_length.to_le_bytes());

    for chunk in bytes.chunks(64) {
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

    let mut digest: Vec<u8> = Vec::with_capacity(16);
    digest.extend(&a.to_le_bytes());
    digest.extend(&b.to_le_bytes());
    digest.extend(&c.to_le_bytes());
    digest.extend(&d.to_le_bytes());

    digest.iter().map(|b| format!("{:02x}", b)).collect::<String>()
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

