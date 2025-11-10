use std::collections::{HashMap as Map, BTreeSet as OSet};
use std::mem::transmute as cast;

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
            while j < z && test_b(s[j]) { j += 1 }
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
            ss.push(unsafe { cast::<&[u8], &'static [u8]>(&s[p..i]) });
            p = i
        }
        ss.push(unsafe { cast::<&[u8], &'static [u8]>(&s[p..]) });
        ss
    };

    fn s2k(s: &[u8]) -> u64 {
        let mut k = 0;
        for &b in s.iter().rev() { k = (k << 8) | b as u64 }
        k
    }
    type TetraMap = (
        Map<u32, u32>,
        Map<u64, u32>,
        Map<(u64, u64), u32>,
        Map<Vec<u8>, u32>,
    );
    let (tx, rx) = std::sync::mpsc::channel();
    for &s in &ss {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut p = 0;
            let mut h: TetraMap = Default::default();
            let mut f = | s: &[u8] | {
                match s.len() {
                    0..=1  => return,
                    2..=4  => *h.0.entry(s2k(s) as _).or_insert(0_u32) += 1,
                    5..=8  => *h.1.entry(s2k(s)).or_insert(0_u32) += 1,
                    9..=16 => {
                        let k = (s2k(&s[..8]), s2k(&s[8..]));
                        *h.2.entry(k).or_insert(0_u32) += 1
                    },
                      _    =>
                        if let Some(c) = h.3.get_mut(s) {
                            *c += 1;
                        } else {
                            h.3.insert(s.to_vec(), 1_u32);
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
    let mut h: Vec<TetraMap> = vec![];
    for a in rx {
        if let Some(b) = h.pop() {
            hc -= 1;
            let tx = tx.clone();
            std::thread::spawn(move || {
                let (mut a, mut b) = (a, b);
                macro_rules! merge {
                    ($i:tt) => {
                        if a.$i.len() < b.$i.len() {
                            (a.$i, b.$i) = (b.$i, a.$i)
                        }
                        for (k, v) in b.$i {
                            *a.$i.entry(k).or_insert(0) += v
                        }
                    }
                }
                merge!(0);
                merge!(1);
                merge!(2);
                merge!(3);
                tx.send(a).unwrap();
            });
        } else {
            h.push(a);
            if hc == 1 { break }
        }
    }

    let mut r = Vec::with_capacity(11e6 as usize);
    let (h0, h1, h2, h) = h.pop().unwrap();
    let mut v: Vec<_> = h.into_iter()
        .map(|(s, c)| (s, c as i64, 0_i64)).collect();
    for (mut k, c) in h0 {
        let z = 4 - k.leading_zeros() / 8;
        let mut s = Vec::with_capacity(z as usize);
        for _ in 0..z { s.push(k as u8); k >>= 8 }
        v.push((s, c as i64, 0_i64));
    }
    for (mut k, c) in h1 {
        let z = 8 - k.leading_zeros() / 8;
        let mut s = Vec::with_capacity(z as usize);
        for _ in 0..z { s.push(k as u8); k >>= 8 }
        v.push((s, c as i64, 0_i64));
    }
    for ((mut k0, mut k1), c) in h2 {
        let z = 8 - k1.leading_zeros() / 8;
        let mut s = Vec::with_capacity(8 + z as usize);
        for _ in 0..8 { s.push(k0 as u8); k0 >>= 8 }
        for _ in 0..z { s.push(k1 as u8); k1 >>= 8 }
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
    let mut h: TetraMap = Default::default();
    let mut insert = | s: Vec<u8>, v: u32 | {
        match s.len() {
            0..=4  => h.0.insert(s2k(&s) as u32, v),
            5..=8  => h.1.insert(s2k(&s), v),
            9..=16 => h.2.insert((s2k(&s[..8]), s2k(&s[8..])), v),
              _    => h.3.insert(s, v),
        };
    };
    for c in c {
        let t = v.pop().unwrap();
        r.push(c as u8);
        r.extend(&t.0);
        r.push(0);
        insert(t.0, c as u32)
    }
    for (x, y) in [(256, 2), (1 << 16, 3), (877805, 4)] {
        pv(&mut v, y as i64);
        let z = (y as u32 - 1) << 24;
        for x in z..z + x {
            let t = v.pop().unwrap();
            r.extend(&t.0);
            r.push(0);
            insert(t.0, x)
        }
    }
    let h = h;

    let (tx, rx) = std::sync::mpsc::channel();
    for (ti, &s) in ss.iter().enumerate() {
        let tx = tx.clone();
        let h = unsafe { cast::<&TetraMap, &'static TetraMap>(&h) };
        std::thread::spawn(move || {
            let mut r = vec![];
            let mut p = 0;
            let mut f = | s: &[u8] | {
                macro_rules! p {
                    ($v:ident) => {{
                        let z = $v >> 24;
                        if z == 0 {
                            r.push($v as u8)
                        } else {
                            let mut v = $v;
                            r.push(252 + z as u8);
                            for _ in 0..z { r.push(v as u8); v >>= 8 }
                        }
                    }}
                }
                macro_rules! e {
                    ($i:tt, $k:ident) => {
                        if let Some(&v) = h.$i.get(&$k) {
                            p!(v)
                        } else {
                            r.extend(s)
                        }
                    }
                }
                match s.len() {
                    0..=1  => r.extend(s),
                    2..=4  => { let k = s2k(s) as u32; e!(0, k) },
                    5..=8  => { let k = s2k(s); e!(1, k) },
                    9..=16 => {
                        let k = (s2k(&s[..8]), s2k(&s[8..]));
                        e!(2, k)
                    },
                      _    =>
                        if let Some(&v) = h.3.get(s) {
                            p!(v)
                        } else {
                            r.extend(s)
                        },
                }
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
            if x.0 != t { break }
            f.write_all(&s.pop_first().unwrap().1).unwrap();
            t += 1
        }
    }
}
