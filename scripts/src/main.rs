mod primes; 

const MAX_P : usize = 100;
const PRIMES : [u32; 100000] = primes::erathosten_sieve(MAX_P);


const DEBUG_FLAG : bool = false;


fn is_curve_over_Q_non_singular(a: i32, b:i32) -> bool {
    4*a*a*a+27*b*b != 0
}

fn is_good_reduction_reduction(a: i32, b:i32, p:u32) -> bool {
    let a = a.rem_euclid(p as i32) as u32;
    let b = b.rem_euclid(p as i32) as u32;
    let discriminant = ((4 * a % p) * (a * a % p) % p)+ (27 * b % p) * b % p;
    discriminant != 0
}

fn is_point_on_curve(a: i32, b:i32, x:u32, y:u32, p:u32) -> bool {
    let a = a.rem_euclid(p as i32) as u32;
    let b = b.rem_euclid(p as i32) as u32;
    let x = x % p;
    let y = y % p;

    (y * y) % p == ((x * x) % p * x % p + a * x % p + b) % p
}

//TODO: fix for bad reductions
fn cardinality_of_e_f_p(a: i32, b:i32, p:u32) -> u32 {
    let mut count: u32 = 0;
    for x in 0..p {
        for y in 0..p {
            if is_point_on_curve(a, b, x, y, p) {
                count += 1;
                if DEBUG_FLAG {println!("{}, {}", x, y);}
            }
        }
    }
    count + 1
}

fn hasse_weil_precompute(a:i32, b:i32) -> [i32; MAX_P] {
    let mut cardinality_per_p_index : [u32; MAX_P] = [0; MAX_P];
    let mut a_p : [i32; MAX_P] = [0; MAX_P];

    for p in 0..MAX_P as usize {
        cardinality_per_p_index[p] = cardinality_of_e_f_p(a, b, PRIMES[p]);
        a_p[p] = (PRIMES[p] + 1) as i32 - cardinality_per_p_index[p] as i32;
    }
    a_p
}

fn check_hasse_weil_bound(a:i32, b: i32) -> bool {
    let a_p = hasse_weil_precompute(1, 3);
    for p in 0..50 as usize{
        if DEBUG_FLAG {println!("p={}, |a_{{F_p}}|={}", PRIMES[p], a_p[p]);}
        if a_p[p].abs() as f32 > 2.0*(PRIMES[p] as f32).sqrt() {
            return false;
        }
    }
    true
}

fn main() {
    if check_hasse_weil_bound(0, 3) {println!("Hasse-Weil Bound checks out!!!");}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_hasse_weil_bound() {
        assert_eq!(check_hasse_weil_bound(0, 3), true);
        assert_eq!(check_hasse_weil_bound(0, -2), true);
        assert_eq!(check_hasse_weil_bound(4, 0), true);
        assert_eq!(check_hasse_weil_bound(-4, 0), true);
    }

    #[test]
    fn test_is_curve_over_Q_non_singular() {
        assert_eq!(is_curve_over_Q_non_singular(0, 0), false);
        assert_eq!(is_curve_over_Q_non_singular(0, -2), true);
    }
}