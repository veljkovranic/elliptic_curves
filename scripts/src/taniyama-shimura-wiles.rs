use polynomial::Polynomial;

fn modular_form_coefficient() -> Polynomial<i32> {
    let mut product = Polynomial::new(vec![0, 1]);
    const modulo_degree: usize = 13;
    for i in 1..14 {
        let mut tmp_poly_smaller : [i32;200] = [0;200];
        tmp_poly_smaller[0] = 1;
        tmp_poly_smaller[i] = -2;
        tmp_poly_smaller[2*i] = 1;
        let poly_smaller = Polynomial::new(tmp_poly_smaller.to_vec());
        let mut tmp_poly_bigger : [i32;200] = [0;200];
        tmp_poly_bigger[0] = 1;
        if (11*i < modulo_degree) { tmp_poly_bigger[11*i] = -2; }
        if (22*i < modulo_degree) { tmp_poly_bigger[2*11*i] = 1; }
        let poly_bigger = Polynomial::new(tmp_poly_bigger.to_vec());
        product = product * poly_smaller * poly_bigger;
    }
    println!("{}", product.pretty("x"));
    product
}

fn main(){
    let generating_function = modular_form_coefficient();

}