fn main() {
    let mut a = std::env::args().skip(1);
    let s = std::fs::read(a.next().unwrap()).unwrap();
    let mut c = [true; 253];
    let mut d = vec![0; 624447];
    let mut si = 0;
    for _ in 0..47 {
        let i = s[si] as usize;
        si += 1;
        (c[i], d[i]) = (false, si);
        si = (si + 2..).find(|&i| s[i] == 0).unwrap() + 1;
    }
    let o = [1_000, 10_000, 100_000];
    for (di, &o) in std::iter::zip([256, 1 << 16, 524447], &o) {
        for di in 0..di {
            d[di + o] = si;
            si = (si + 2..).find(|&i| s[i] == 0).unwrap() + 1;
        }
    }
    let (c, d) = (c, d);
    let mut r = Vec::with_capacity(1e9 as usize);
    let sz = s.len();
    while si < sz {
        let i = s[si] as usize;
        si += 1;
        if i < 253 {
            if c[i] { r.push(i as u8); } else {
                let mut j = d[i];
                while s[j] != 0 { r.push(s[j]); j += 1 }
            }
        } else {
            let i = i - 253;
            let mut j = 0;
            for i in 0..=i {
                j |= (s[si] as usize) << i * 8;
                si += 1;
            }
            j = d[j + o[i]];
            while s[j] != 0 { r.push(s[j]); j += 1 }
        }
    }
    std::fs::write(a.next().unwrap(), &r).unwrap();
}
