fn main() {
    let mut a = std::env::args().skip(1);
    let s = std::fs::read(a.next().unwrap()).unwrap();
    let mut c = [true; 255];
    let mut d = [0; 1_256];
    let mut si = 0;
    for _ in 0..49 {
        let i = s[si] as usize;
        si += 1;
        (c[i], d[i]) = (false, si);
        si = (si + 2..).find(|&i| s[i] == 0).unwrap() + 1;
    }
    for di in 0..256 {
        d[di + 1_000] = si;
        si = (si + 2..).find(|&i| s[i] == 0).unwrap() + 1;
    }
    let (c, d) = (c, d);
    let mut r = Vec::with_capacity(1e9 as usize);
    let sz = s.len();
    while si < sz {
        let i = s[si] as usize;
        si += 1;
        if i != 255 {
            if c[i] { r.push(i as u8); } else {
                let mut j = d[i];
                while s[j] != 0 { r.push(s[j]); j += 1 }
            }
        } else {
            let mut j = d[1_000 + s[si] as usize];
            si += 1;
            while s[j] != 0 { r.push(s[j]); j += 1 }
        }
    }
    std::fs::write(a.next().unwrap(), &r).unwrap();
}
