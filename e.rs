use std::collections::HashMap as Map;

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

    let (tx, rx) = std::sync::mpsc::channel();
    for &s in &ss {
        let tx = tx.clone();
        let s = s.to_vec();
        std::thread::spawn(move || {
            let mut p = 0;
            let mut hx = Map::new();
            let mut hy = Map::new();
            let mut f = | s: &[u8] | {
                let z = s.len() as i64;
                if z < 2 { return }
                if z <= 7 {
                    let mut k = 0i64;
                    for &b in s.iter().rev() { k = (k << 8) | b as i64 }
                    k = (k << 8) | z;
                    *hx.entry(k).or_insert(0i64) += 1;
                } else if let Some(c) = hy.get_mut(s) {
                    *c += 1;
                } else {
                    hy.insert(s.to_vec(), 1_i64);
                }
            };
            while let Some((i, j)) = next_ij(&s, p) {
                for s in [&s[p..i], &s[i..j]] { f(s) }
                p = j;
            }
            f(&s[p..]);
            tx.send((hx, hy)).unwrap();
        });
    }
    let mut hc = nt;
    let mut h: Vec<(Map<i64, i64>, Map<Vec<u8>, i64>)> = vec![];
    for (mut ax, mut ay) in rx {
        if let Some((mut bx, mut by)) = h.pop() {
            hc -= 1;
            let tx = tx.clone();
            std::thread::spawn(move || {
                if ax.len() < bx.len() { (ax, bx) = (bx, ax) }
                for (k, v) in bx { *ax.entry(k).or_insert(0) += v }
                if ay.len() < by.len() { (ay, by) = (by, ay) }
                for (k, v) in by { *ay.entry(k).or_insert(0) += v }
                tx.send((ax, ay)).unwrap();
            });
        } else {
            h.push((ax, ay));
            if hc == 1 { break }
        }
    }

    let mut r = Vec::with_capacity(452e6 as usize);
    let (hx, hy) = h.pop().unwrap();
    let mut v = hy.into_iter()
        .map(|(s, v)| (s, v, 0i64)).collect::<Vec<_>>();
    for (mut k, c) in hx {
        let z = k as u8; k >>= 8;
        let mut s = Vec::with_capacity(z as usize);
        for _ in 0..z { s.push(k as u8); k >>= 8 }
        v.push((s, c, 0i64));
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
    let mut h = Map::new();
    for c in c {
        let t = v.pop().unwrap();
        r.push(c as u8);
        r.extend(&t.0);
        r.push(0);
        h.insert(t.0, [c as u8].to_vec());
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
            h.insert(t.0, b.clone());
        }
    }

    let mut hx = Map::new();
    let mut hy = Map::new();
    for (s, v) in h {
        if s.len() <= 8 {
            let mut k = 0_u64;
            for b in s.into_iter().rev() { k = (k << 8) | b as u64 }
            hx.insert(k, v);
        } else {
            hy.insert(s, v);
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
                let z = s.len();
                let v = if z < 2 {
                    s
                } else if z <= 8 {
                    let mut k = 0_u64;
                    for &b in s.iter().rev() { k = (k << 8) | b as u64 }
                    if let Some(v) = hx.get(&k) { v } else { s }
                } else if let Some(v) = hy.get(s) {
                    v
                } else {
                    s
                };
                r.extend(v)
            };
            while let Some((i, j)) = next_ij(&s, p) {
                for s in [&s[p..i], &s[i..j]] { f(s) }
                p = j;
            }
            f(&s[p..]);
            tx.send((ti, r)).unwrap();
        });
    }
    let mut v: Vec<_> = (0..nt).map(|_| rx.recv().unwrap()).collect();
    v.sort_by_key(|t| t.0);
    for (_, v) in v { r.extend(&v) }

    std::fs::write(a.next().unwrap(), &r).unwrap();
}
