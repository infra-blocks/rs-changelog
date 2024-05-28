#[derive(Debug)]
pub struct Config {
    pub flag: bool,
    pub option: Option<String>,
    pub positional: String,
}

pub fn do_stuff(config: Config) {
    println!("received config {:?}", config);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_do_stuff() {
        // Just checking it's not throwing.
        do_stuff(Config {
            flag: true,
            option: Some(String::from("that's my option bro")),
            positional: String::from("wold is pop"),
        })
    }
}
