use crate::http::start_server;

mod http;

fn main() {
    match start_server() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use crate::http::start_server;

    #[test]
    fn test_server() {
        assert!(true);
    }
}
