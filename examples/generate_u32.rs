fn main() {
    let mut base = cryptorusticon::bbs::BBS::default();

    for i in 0..10 {
        println!("{} {}", i, base.generate_u32());
    }

    println!("\n");

    let custom = cryptorusticon::bbs::BBS::new(b"30000000091", b"40000000003", b"9276869236");
    match custom {
        Ok(mut bbs) => {
            for i in 0..10 {
                println!("{} {}", i, bbs.generate_u32());
            }
        }
        Err(errors) => {
            // Handle the errors
            println!("Failed to create BBS:");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
}
