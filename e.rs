use std::collections::{HashMap as Map, BTreeSet as OSet};

fn main() {
    let mut a = std::env::args().skip(1);
    let s = std::fs::read(a.next().unwrap()).unwrap();

    fn test_b(b: u8) -> bool {
        [b'a', b'A'].iter().any(|&s| (s..s + 26).contains(&b))
    }
    fn next_ij(s: &[u8], mut i: usize) -> Option<(usize, usize)> {
        let z = s.len();
        while i < z && s[i] != b'&' && !test_b(s[i]) { i += 1 }
        if i == z { return None }
        let mut j = i + 1;
        if s[i] == b'&' {
            while s[j] != b';' { j += 1 }
            j += 1;
        } else {
            while test_b(s[j]) { j += 1 }
        }
        if i > 0 && s[i - 1] == b' ' { i -= 1 }
        Some((i, j))
    }

    let nt = std::thread::available_parallelism()
        .unwrap().get().clamp(2, 16);
    let ss = {
        let mut ss = vec![];
        let mut p = 0;
        let z = 1e9 as usize / nt;
        for _ in 0..nt - 1 {
            let mut i = p + z;
            while test_b(s[i]) { i += 1 }
            while s[i] != b'&' && !test_b(s[i]) { i += 1 }
            if i > 0 && s[i - 1] == b' ' { i -= 1 }
            ss.push(&s[p..i]);
            p = i
        }
        ss.push(&s[p..]);
        ss
    };

    fn s2k(s: &[u8]) -> u64 {
        let mut k = 0;
        for &b in s.iter().rev() { k = (k << 8) | b as u64 }
        k
    }
    type DiMap = (Map<u64, u32>, Map<Vec<u8>, u32>);
    fn new_bm() -> DiMap { (Map::new(), Map::new()) }
    let (tx, rx) = std::sync::mpsc::channel();
    for &s in &ss {
        let tx = tx.clone();
        let s = s.to_vec();
        std::thread::spawn(move || {
            let mut p = 0;
            let mut h = new_bm();
            let mut f = | s: &[u8] | {
                match s.len() {
                    0..=1 => return,
                    2..=8 => *h.0.entry(s2k(s)).or_insert(0_u32) += 1,
                      _   =>
                        if let Some(c) = h.1.get_mut(s) {
                            *c += 1;
                        } else {
                            h.1.insert(s.to_vec(), 1_u32);
                        }
                }
            };
            while let Some((i, j)) = next_ij(&s, p) {
                for s in [&s[p..i], &s[i..j]] { f(s) }
                p = j
            }
            f(&s[p..]);
            tx.send(h).unwrap();
        });
    }
    let mut hc = nt;
    let mut h: Vec<DiMap> = vec![];
    for a in rx {
        if let Some(b) = h.pop() {
            hc -= 1;
            let tx = tx.clone();
            std::thread::spawn(move || {
                let (mut a, mut b) = (a, b);
                if a.0.len() < b.0.len() { (a.0, b.0) = (b.0, a.0) }
                for (k, v) in b.0 { *a.0.entry(k).or_insert(0) += v }
                if a.1.len() < b.1.len() { (a.1, b.1) = (b.1, a.1) }
                for (k, v) in b.1 { *a.1.entry(k).or_insert(0) += v }
                tx.send(a).unwrap();
            });
        } else {
            h.push(a);
            if hc == 1 { break }
        }
    }

    let mut r = Vec::with_capacity(11e6 as usize);
    let (hx, hy) = h.pop().unwrap();
    let mut v = hy.into_iter()
        .map(|(s, c)| (s, c as i64, 0_i64)).collect::<Vec<_>>();
    for (mut k, c) in hx {
        let z = 8 - k.leading_zeros() / 8;
        let mut s = Vec::with_capacity(z as usize);
        for _ in 0..z { s.push(k as u8); k >>= 8 }
        v.push((s, c as i64, 0_i64));
    }
    fn pv(v : &mut Vec<(Vec<u8>, i64, i64)>, n: i64) {
        for t in v.iter_mut() {
            let z = t.0.len() as i64;
            t.2 = t.1 * (z - n) - (z + 1);
        }
        v.retain(|t| t.2 > 0);
        v.sort_by_key(|t| t.2);
    }
    pv(&mut v, 1);
    let c = {
        let mut c = [true; 253];
        for &b in &s { c[b as usize] = false; }
        (0..253).filter(|&i| c[i]).collect::<Vec<_>>()
    };
    let mut hx = Map::new();
    let mut hy = Map::new();
    let mut insert = | s: Vec<u8>, v: Vec<u8> | {
        if s.len() <= 8 {
            hx.insert(s2k(&s), v);
        } else {
            hy.insert(s, v);
        }
    };
    for c in c {
        let t = v.pop().unwrap();
        r.push(c as u8);
        r.extend(&t.0);
        r.push(0);
        insert(t.0, [c as u8].to_vec())
    }
    for (x, y) in [(256, 2), (1 << 16, 3), (877805, 4)] {
        pv(&mut v, y as i64);
        let mut b = vec![251 + y as u8; y];
        for mut x in 0..x {
            let t = v.pop().unwrap();
            for b in &mut b[1..] {
                *b = x as u8;
                x >>= 8;
            }
            r.extend(&t.0);
            r.push(0);
            insert(t.0, b.clone())
        }
    }
    let h = (hx, hy);

    let (tx, rx) = std::sync::mpsc::channel();
    for (ti, &s) in ss.iter().enumerate() {
        let tx = tx.clone();
        let s = s.to_vec();
        let (hx, hy) = h.clone();
        std::thread::spawn(move || {
            let mut r = vec![];
            let mut p = 0;
            let mut f = | s: &[u8] | {
                let v = match s.len() {
                    0..=1 => s,
                    2..=8 => if let Some(v) = hx.get(&s2k(s)) { v } else { s },
                      _   => if let Some(v) = hy.get(s) { v } else { s }
                };
                r.extend(v)
            };
            while let Some((i, j)) = next_ij(&s, p) {
                for s in [&s[p..i], &s[i..j]] { f(s) }
                p = j
            }
            f(&s[p..]);
            tx.send((ti, r)).unwrap();
        });
    }

    let mut f = std::fs::File::create(a.next().unwrap()).unwrap();
    use std::io::Write;
    f.write_all(&r).unwrap();
    let mut s = OSet::new();
    let mut t = 0;
    while t < nt {
        s.insert(rx.recv().unwrap());
        while let Some(x) = s.first() {
            if x.0 == t {
                t += 1;
                f.write_all(&s.pop_first().unwrap().1).unwrap();
            } else {
                break
            }
        }
    }
}
