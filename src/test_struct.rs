
#[derive(Default, Debug)]
pub struct Father {
    debug: bool,
    retries: u32,
    timeout: f64,
    child: Child,
}

#[derive(Debug)]
pub struct Child {
    pub(crate) debug: bool,
    pub(crate) retries: u32,
    pub(crate) timeout: f64,
}

impl Default for Child {
    fn default() -> Self {
        Self {
            debug: false,
            retries: 8,
            timeout: 3.0,
        }
    }
}

impl Child {
    fn new(self: Self) -> Self {
        Self { ..self }
    }
    fn get_debug(&self) -> bool {
        self.debug
    }
}


fn return_result(value: i32) -> Result<String, String> {
    let a: Result<String, String> = Ok(value.to_string());
    let b: Result<String, String> = Err(value.to_string());
    Err(value.to_string())
}

#[derive(Debug, Copy, Clone)]
struct sample_struct {
    i: i32,
}