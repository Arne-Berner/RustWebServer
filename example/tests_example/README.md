### Was kommt hier rein?

### Was könnte hier noch rein kommen?
config variables überprüfen?
```rust
...

impl Config {
   pub fn new() -> Result<Config, handle_errors::Error> {
       let config = Config::parse();
   
       if let Err(_) = env::var("BAD_WORDS_API_KEY") {
           panic!("BadWords API key not set");
       }
   
       if let Err(_) = env::var("PASETO_KEY") {
           panic!("PASETO_KEY not set");
       }
   
...
```

```rust
...


#[cfg(test)]
mod config_tests {
   use super::*;

   #[test]
   fn unset_api_key() {
       let result = std::panic::catch_unwind(|| Config::new());
       assert!(result.is_err());
   }
}
```

positive test:
```rust
...
#[cfg(test)]
mod config_tests {
    use super::*;
 
fn set_env() {
env::set_var("BAD_WORDS_API_KEY", "yes");
env::set_var("PASETO_KEY", "yes");
env::set_var("POSTGRES_USER", "user");
env::set_var("POSTGRES_PASSWORD", "pass");
env::set_var("POSTGRES_HOST", "localhost");
env::set_var("POSTGRES_PORT", "5432");
env::set_var("POSTGRES_DB", "rustwebdev");
}
 
    #[test]
    fn unset_api_key() {
        let result = std::panic::catch_unwind(|| Config::new());
        assert!(result.is_err());
    }
 
#[test]
fn set_api_key() {
set_env();
 
let expected = Config {
log_level: "warn".to_string(),
port: 8080,
db_user: "user".to_string(),
db_password: "pass".to_string(),
db_host: "localhost".to_string(),
db_port: 5432,
db_name: "rustwebdev".to_string(),
};
 
let config = Config::new().unwrap();
 
assert_eq!(config, expected);
}
}
```

Diese beiden Tests könnte man kombinieren, weil tests parallel ablaufen und das Fehler werfen würde, oder man könnte die tests nicht parallel ablaufen lassen.
