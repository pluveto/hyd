fn parse_next(args: &Vec<String>, i_: &mut usize) -> String {
    let location: Option<String> = next(args, i_);
    *i_ += 1;
    if location == None {
        eprintln!("unexpected end of save statement");
        std::process::exit(1)
    }
    location.unwrap()
}

// increase i then read a token
// args     the token list. 
// i        the index of current token
fn next(args: &Vec<String>, i_: &mut usize) -> Option<String> {
    *i_ += 1;
    if args.len() <= *i_ {
        return None;
    }
    Some(args[*i_].clone())
}

pub struct AppArgument {
    pub source_location: Option<String>,
    pub target_location: Option<String>,
}

pub fn parse_all(args: &Vec<String>) -> AppArgument {
    let mut i: usize = 1;

    let mut source_location: Option<String> = None;
    let mut target_location: Option<String> = None;

    while i < args.len() {
        match args[i].as_str() {
            "save" => source_location = Some(parse_next(&args, &mut i)),
            "to" => target_location = Some(parse_next(&args, &mut i)),
            _ => {
                eprintln!("unexpected token: {}", args[i]);
                i += 1;
            }
        }
    }

    AppArgument {
        source_location,
        target_location,
    }
}
