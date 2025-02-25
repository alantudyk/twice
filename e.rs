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
        for s in [&s[p..i], &s[i..j]] {
            if s.len() > 1 { *h.entry(s).or_insert(0i64) += 1 }
        }
        p = j;
    }
    let mut r = Vec::with_capacity(741e6 as usize);
    let mut v = h.into_iter()
        .map(|(s, v)| (s, v, 0i64)).collect::<Vec<_>>();
    fn pv(v : &mut Vec<(&[u8], i64, i64)>, n: i64) {
        for t in v.iter_mut() {
            let z = t.0.len() as i64;
            t.2 = t.1 * (z - n) - (z + 1);
        }
        v.retain(|t| t.2 > 0);
        v.sort_by_key(|t| t.2);
    }
    pv(&mut v, 1);
    let c = {
        let mut c = [true; 255];
        for &b in &s { c[b as usize] = false; }
        (0..255).filter(|&i| c[i]).collect::<Vec<_>>()
    };
    let mut h = Map::new();
    for c in c {
        let t = v.pop().unwrap();
        h.insert(t.0, [c as u8].to_vec());
        r.push(c as u8);
        r.extend(t.0);
        r.push(0);
    }
    pv(&mut v, 2);
    let mut b = vec![255_u8; 2];
    for x in 0..256 {
        b[1] = x as u8;
        let t = v.pop().unwrap();
        h.insert(t.0, b.clone());
        r.extend(t.0);
        r.push(0);
    }
    let mut p = 0;
    while let Some((i, j)) = next_ij(p) {
        for s in [&s[p..i], &s[i..j]] {
            r.extend(if let Some(v) = h.get(s) { v } else { s })
        }
        p = j;
    }
    r.extend(&s[p..]);
    std::fs::write(a.next().unwrap(), &r).unwrap();
}
