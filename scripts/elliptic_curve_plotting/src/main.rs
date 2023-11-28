use std::collections::HashSet;

fn main() {
    let p = 17;
    let a = 0; 
    let b = 7;

    let points = find_elliptic_curve_points(p, a, b);
    print_equation_and_count(a, b, &points);
    draw_grid(p, &points);
}

fn find_elliptic_curve_points(p: i32, a: i32, b: i32) -> HashSet<(i32, i32)> {
    let mut points = HashSet::new();
    for x in 0..p {
        for y in 0..p {
            if is_point_on_curve(x, y, p, a, b) {
                points.insert((x, y));
            }
        }
    }
    points.insert((-1, -1));
    points
}

fn is_point_on_curve(x: i32, y: i32, p: i32, a: i32, b: i32) -> bool {
    let left = mod_pow(y, 2, p);
    let right = (mod_pow(x, 3, p) + a * x + b) % p;
    left == right
}

fn mod_pow(base: i32, exp: i32, modulus: i32) -> i32 {
    let mut base = base % modulus;
    let mut exp = exp;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }
    result
}

fn draw_grid(p: i32, points: &HashSet<(i32, i32)>) {
    println!("+{}+", "-".repeat((p*2) as usize));
    for y in (0..p).rev() {
        print!("|");
        for x in 0..p {
            if points.contains(&(x, y)) {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!("|");
    }
    println!("+{}+", "-".repeat((p*2) as usize));
}

fn print_equation_and_count(a: i32, b: i32, points: &HashSet<(i32, i32)>) {
    let mut str_format: String = "Elliptic Curve: y^2 = x^3 ".to_owned();
    
    if a!=0 {
        let a_part: String = format!("+ {}x ", a).to_owned();
        str_format.push_str(&a_part);
    }
    if b!=0 {
        let b_part: String = format!("+ {} ", b).to_owned();
        str_format.push_str(&b_part);
    }
    println!("{}", str_format);
    println!("Number of Points: {} (including point at ∞)", points.len() );
}
