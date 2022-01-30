enum InertiaError {
    InputErro,
}

struct InputError {
    input: Vec<&str>, 
}

impl InputError {
    pub fn display(&self) -> String {
    }
}
