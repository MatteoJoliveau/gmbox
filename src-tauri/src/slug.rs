use rand::{distributions::Alphanumeric, Rng};

pub fn id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub fn slugify<S: AsRef<str>>(s: S) -> String {
    _slugify(s.as_ref())
}

fn _slugify(s: &str) -> String {
    let mut s = s.to_lowercase().replace(" ", "-").replace("_", "-");

    if s.ends_with("-") {
        s.pop();
    }

    s
}
