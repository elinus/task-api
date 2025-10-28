use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    println!("##### Bcrypt Demo! #####");
    let password = "mypassword123";

    println!("Hashing same password 5 times:\n");
    for i in 1..=5 {
        let hash = hash(password, DEFAULT_COST).unwrap();
        println!("Hash {}: {}", i, hash);
    }

    println!("\nNotice: All different! Each has random salt.");
    println!("But all verify correctly:\n");

    for i in 1..=5 {
        let hash = hash(password, DEFAULT_COST).unwrap();
        let valid = verify(password, &hash).unwrap();
        println!("Verify #{}: {}", i, valid);
    }
}
