struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty() && password.contains(['!', '.'])
    };
    let cred = Credentials {
        username: "admin".to_owned(),
        password: "password123".to_owned(),
        validator,
    };

    // println!("{}",cred.is_valid());

    let pas_val = get_password_validator(6, false);

    let def_cred = get_default_creds(pas_val);

    println!("{:?}", def_cred.is_valid());
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty() && password.contains("pat")
}

fn get_default_creds<T>(f: T) -> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    Credentials {
        username: "guest".to_owned(),
        password: "paggggkgkkggkgk".to_owned(),
        validator: f,
    }
}

fn get_password_validator(min_len: usize, special_chars: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_chars {
        Box::new(move |_: &str, password: &str| {
            !password.len() > min_len && password.contains(['!', '@'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !(password.len() > min_len))
    }
}
