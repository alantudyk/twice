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
            let mut h = Map::new();
            while let Some((i, j)) = next_ij(&s, p) {
                for s in [&s[p..i], &s[i..j]] {
                    if s.len() < 2 { continue }
                    if let Some(c) = h.get_mut(s) {
                        *c += 1;
                    } else {
                        h.insert(s.to_vec(), 1_i64);
                    }
                }
                p = j;
            }
            let s = &s[p..];
            if s.len() > 1 { *h.entry(s.to_vec()).or_insert(0) += 1 }
            tx.send(h).unwrap();
        });
    }
    let mut hc = nt;
    let mut h: Vec<Map<Vec<u8>, i64>> = vec![];
    for mut a in rx { 
        if let Some(mut b) = h.pop() {
            hc -= 1;
            let tx = tx.clone();
            std::thread::spawn(move || {
                if a.len() < b.len() { (a, b) = (b, a) }
                for (k, v) in b { *a.entry(k).or_insert(0) += v }
                tx.send(a).unwrap();
            });
        } else {
            h.push(a);
            if hc == 1 { break }
        }
    }

    let mut r = Vec::with_capacity(452e6 as usize);
    let mut v = h.pop().unwrap().into_iter()
        .map(|(s, v)| (s, v, 0i64)).collect::<Vec<_>>();
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
    let h = h;

    let (tx, rx) = std::sync::mpsc::channel();
    for (ti, &s) in ss.iter().enumerate() {
        let tx = tx.clone();
        let s = s.to_vec();
        let h = h.clone();
        std::thread::spawn(move || {
            let mut r = vec![];
            let mut p = 0;
            while let Some((i, j)) = next_ij(&s, p) {
                for s in [&s[p..i], &s[i..j]] {
                    r.extend(if let Some(v) = h.get(s) { v } else { s })
                }
                p = j;
            }
            let s = &s[p..];
            r.extend(if let Some(v) = h.get(s) { v } else { s });
            tx.send((ti, r)).unwrap();
        });
    }
    let mut v: Vec<_> = (0..nt).map(|_| rx.recv().unwrap()).collect();
    v.sort_by_key(|t| t.0);
    for (_, v) in v { r.extend(&v) }

    std::fs::write(a.next().unwrap(), &r).unwrap();
}
