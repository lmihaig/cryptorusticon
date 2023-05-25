fn main() {
    println!("Default:");
    let mut base = cryptorusticon::bbs::BBS::default();
    for i in 0..10 {
        println!("{} {}", i, base.generate_u32());
    }
    println!("\n");

    println!("Custom p q:");
    let custom = cryptorusticon::bbs::BBS::new(b"30000000091", b"40000000003", b"9276869236");
    match custom {
        Ok(mut bbs) => {
            for i in 0..10 {
                println!("{} {}", i, bbs.generate_u16());
            }
        }
        Err(errors) => {
            println!("Failed to create BBS:");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    println!("\n");

    println!("Generate p and q from seed:");
    let w_rand_primes = cryptorusticon::bbs::BBS::with_random_primes(256, 2101);
    match w_rand_primes {
        Ok(mut bbs) => {
            for i in 0..10 {
                println!("{} {}", i, bbs.generate_u8());
            }
        }
        Err(err) => {
            println!("Failed to create BBS:");
            println!("  - {}", err);
        }
    }

    println!("Generate fully random:");
    let mut full_random = cryptorusticon::bbs::BBS::with_full_random(256).unwrap();
    println!("{}", full_random.generate_u32());
}
