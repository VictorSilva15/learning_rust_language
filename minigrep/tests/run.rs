use minigrep::*;

#[test]
fn returns_an_error() {
    let config = Config {
        query: String::from("something"),
        filename: String::from("hehehehehee"),
        case_sensitive: true,
    }; 

    let result = run(config);

    assert!(result.is_err());
}

#[test]
fn returns_an_ok(){
    let config = Config {
        query: String::from("whatever"),
        filename: String::from("poem.txt"),
        case_sensitive: true,
    };

    let result =  run(config);

    assert_eq!((), result.unwrap());
}
