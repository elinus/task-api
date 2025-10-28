use base64::Engine;
use base64::engine::general_purpose;

fn main() {
    println!("##### Tamper Token! #####");
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 3 {
        println!("Invalid token format");
        return;
    }

    println!("Original token parts:");
    println!("Header:    {}", parts[0]);
    println!("Payload:   {}", parts[1]);
    println!("Signature: {}", parts[2]);
    println!();

    // Decode payload
    let payload_bytes = general_purpose::STANDARD_NO_PAD.decode(parts[1]).unwrap();
    let payload = String::from_utf8(payload_bytes).unwrap();

    println!("Decoded payload:");
    println!("{}", payload);
    println!();

    let tampered_payload = r#"{"sub":"999","email":"hacker@evil.com","role":"admin"}"#;
    let tampered_payload_b64 = general_purpose::STANDARD_NO_PAD.encode(tampered_payload.as_bytes());

    let tampered_token = format!("{}.{}.{}", parts[0], tampered_payload_b64, parts[2]);

    println!("Tampered token:");
    println!("{}", tampered_token);
    println!();
    println!("If you try to use this tampered token, server will reject it");
    println!("because signature doesn't match the tampered payload!");
}
