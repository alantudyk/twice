use std::collections::HashMap as Map;

fn main() {
    let mut a = std::env::args().skip(1);
    let s = std::fs::read(a.next().unwrap()).unwrap();
    fn test_b(b: u8) -> bool {
        [b'a', b'A'].iter().any(|&s| (s..s + 26).contains(&b))
    }
    let next_ij = | mut i: usize | {
        const N: usize = 1e9 as usize;
        while i < N && s[i] != b'&' && !test_b(s[i]) { i += 1 }
        if i == N { return None }
        let mut j = i + 1;
        if s[i] == b'&' {
            while s[j] != b';' { j += 1 }
            j += 1;
        } else {
            while test_b(s[j]) { j += 1 }
        }
        if i > 0 && s[i - 1] == b' ' { i -= 1 }
        Some((i, j))
    };
    let mut p = 0;
    let mut h = Map::new();
    while let Some((i, j)) = next_ij(p) {
        if j - i > 1 { *h.entry(&s[i..j]).or_insert(0i64) += 1; }
        p = j;
    }
    let mut r = Vec::with_capacity(526e6 as usize);
    let mut v = h.into_iter()
        .map(|(s, v)| (s, v, 0i64)).collect::<Vec<_>>();
    fn pv(v : &mut Vec<(&[u8], i64, i64)>, n: i64) {
        for t in v.iter_mut() {
            let z = t.0.len() as i64;
            t.2 = t.1 * (z - n) - (z + 1);
        }
        v.sort_by_key(|t| t.2);
    }
    pv(&mut v, 1);
    let c = {
        let mut c = [true; 253];
        for &b in &s { c[b as usize] = false; }
        (0..253).filter(|&i| c[i]).collect::<Vec<_>>()
    };
    let mut h = Map::new();
    for c in c {
        let t = v.pop().unwrap();
        h.insert(t.0, [c as u8].to_vec());
        r.push(c as u8);
        r.extend(t.0);
        r.push(0);
    }
    for (x, y) in [(256, 2), (1 << 16, 3), (524447, 4)] {
        pv(&mut v, y as i64);
        let mut b = vec![251 + y as u8; y];
        for mut x in 0..x {
            let t = v.pop().unwrap();
            for b in &mut b[1..] {
                *b = x as u8;
                x >>= 8;
            }
            h.insert(t.0, b.clone());
            r.extend(t.0);
            r.push(0);
        }
    }
    let mut p = 0;
    while let Some((i, j)) = next_ij(p) {
        r.extend(&s[p..i]);
        if let Some(v) = h.get(&s[i..j]) {
            r.extend(v);
        } else {
            r.extend(&s[i..j]);
        }
        p = j;
    }
    r.extend(&s[p..]);
    let _ = std::fs::write(a.next().unwrap(), &r);
}
