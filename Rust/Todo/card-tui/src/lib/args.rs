pub struct Args {
    pub thing: String,
}

impl Args {
    pub fn new() -> Self {
        Self::parse_args().unwrap()
    }

    pub fn parse_args() -> Result<Args, lexopt::Error> {
        use lexopt::prelude::*;

        let mut thing = None;
        let _number = 0;
        let mut parser = lexopt::Parser::from_env();

        while let Some(arg) = parser.next()? {
            match arg {
                Value(val) => {
                    thing = Some(val.into_string()?);
                }
                Long("help") => {
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(Args {
            thing: thing.ok_or("missing argument THING")?,
        })
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}
