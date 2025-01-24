fn main() {    
    let result = some_crypto_lib::try_fips_mode();
    match result {
        Ok(_) => println!("FIPS mode enabled"),
        Err(e) => println!("Error enabling FIPS mode: {}", e),
    }
}
