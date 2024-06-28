


pub fn load_environment_var_file() {
    match dotenvy::dotenv() {
        Ok(_) => println!("found .env file, loading contents..."),
        Err(_) => println!("no .env file found, assuming cloud environment.")
    }
}

