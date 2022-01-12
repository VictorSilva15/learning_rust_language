use minigrep::Config;

#[test]
fn not_enough_arguments_config() {
    let args = vec!["whatever".to_string(), "poem.txt".to_string()];

    let config = Config::new(&args);
    assert_eq!(Err("Not enough arguments"), config)
}

#[test]
fn is_a_config_struct(){
    let args = vec!["/path".to_string(), "whatever".to_string(), "poem.txt".to_string()];

    let config = Config::new(&args);

    assert!(config.is_ok())
}

