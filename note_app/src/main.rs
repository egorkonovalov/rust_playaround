type ResultStr = Result<i8, &'static str>;

fn error_check(check: bool) -> ResultStr {
    if check == true {
        Err("this is an error")
    } else {
        Ok(1)
    }
}

fn describe_result(result: ResultStr) {
    match result {
        Ok(x) => println!("it's a result of: {}", x),
        Err(x) => println!("{}", x),
    }
}

fn main() {
    let result: ResultStr = error_check(false);
    describe_result(result)
}
