use crate::variant;

pub(crate) const MONT_MOD: i64 = 1 << 32;
pub(crate) const Q_INV: i32 = leaktime::modinverse(variant::Q as i64, MONT_MOD) as i32;

mod leaktime {
    const fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    const fn cmod(mut x: i64, m: i64) -> i64 {
        x %= m;
        if x > m / 2 {
            x -= m;
        } else if x < -m / 2 {
            x += m;
        }
        x
    }

    pub(super) const fn modinverse(a: i64, m: i64) -> i64 {
        let (g, x, _) = egcd(a, m);
        if g != 1 {
            panic!();
        }
        cmod(x, m)
    }
}
