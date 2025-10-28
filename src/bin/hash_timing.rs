use bcrypt::verify;
use std::time::Instant;

fn main() {
    println!("##### Hashing Timing! #####");
    let password = "mypassword123";
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
    println!("{}", hash);
    println!("Testing bcrypt timing...\n");

    // Correct password
    let start = Instant::now();
    let result = verify(password, &hash).unwrap();
    let duration = start.elapsed();
    println!("Correct password: {:?} ({})", duration, result);

    // Wrong password (same length)
    let start = Instant::now();
    let result = verify("wrongpassword", &hash).unwrap();
    let duration = start.elapsed();
    println!("Wrong password:   {:?} ({})", duration, result);

    // Wrong password (different length)
    let start = Instant::now();
    let result = verify("wrong", &hash).unwrap();
    let duration = start.elapsed();
    println!("Short password:   {:?} ({})\n", duration, result);

    println!("Notice: All take similar time (~100ms)");
    println!("This prevents timing attacks!");
}
