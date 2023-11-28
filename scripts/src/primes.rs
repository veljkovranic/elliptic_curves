use const_for::const_for;

//Just Erathosten for now, might even hardcode the list, so I don't need to regenerate everytime.
pub const fn erathosten_sieve(max_primes:usize) -> [u32; 100000] {
    let mut count_primes : usize = 2;
    let mut current_set_primes : [u32; 100000] = [0; 100000];
    current_set_primes[0] = 2; current_set_primes [1] = 3;
    let mut p_candidate = 5;
    while count_primes < max_primes {
        let mut still_candidate = true;       
        while still_candidate {
            const_for!(prime_index in 0..count_primes => {
                if p_candidate % current_set_primes[prime_index] == 0 {
                    still_candidate = false;
                    break;
                }
            });
            if still_candidate {
                current_set_primes[count_primes] = p_candidate;
                count_primes += 1;
                // println!(" found! {}", p_candidate);
                break;
            } else {
                still_candidate = true;
                p_candidate += 2;
            }
        }
    }
    current_set_primes
}