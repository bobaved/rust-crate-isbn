#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        let maybe_isbn = crate::isbn::Isbn::from_string("".to_string());
        assert!(maybe_isbn.is_err(), "empty isbn results in error");
        assert!(!maybe_isbn.is_ok());
        assert!(!maybe_isbn.is_ok() && maybe_isbn.is_err());

        let maybe_isbn = crate::isbn::Isbn::from_string("123456789X".to_string());
        assert!(!maybe_isbn.is_err(), "valid isbn");
        assert!(maybe_isbn.is_ok());
        assert!(maybe_isbn.is_ok() && !maybe_isbn.is_err());

        assert!(
            crate::isbn::Isbn::from_string("1-2-3-4-5-6-7-8-9-X".to_string()).is_ok(),
            "valid isbn with dashes",
        );
        assert!(
            crate::isbn::Isbn::from_string("1-2-3-4-5-6-7-8-9-X".to_string()).is_ok(),
            "allow upper case X",
        );
        assert!(
            crate::isbn::Isbn::from_string("1-2-3-4-5-6-7-8-9-x".to_string()).is_ok(),
            "allow lower case x",
        );
        assert!(
            crate::isbn::Isbn::from_string("1-2-3-4-5-6-7-8-8-x".to_string()).is_err(),
            "ten letters but invalid isbn",
        );
    }
}

pub mod isbn {
    pub enum Isbn {
        Isbn10(String),
    }
    
    impl Isbn {
        pub fn from_string(maybe_isbn:String) -> Result<Isbn, String> {
            let maybe_isbn = maybe_isbn.replace("-", "").to_string();
            print!("DEBBUG: {}", maybe_isbn);

            if maybe_isbn.chars().count() != 10 {
                let err = format!("isbn has to be 10 characters long, you currently got {}", maybe_isbn.chars().count());
                return Err(err.to_string());
            }
    
            let maybe_last = maybe_isbn.chars().last();
            let last = match maybe_last {
                Some(last) => last,
                None => return Err("should never be reached".to_string()),
            };
    
            let last_numeric = if last.is_digit(10) {last.to_digit(10)} else {Some(10)};
            let last_numeric = match last_numeric {
                Some(i) => i,
                None => return Err(format!("last letter \"{}\" is not a digit or \"X\"", last).to_string()),
            };
    
            let sliced_isbn = (&maybe_isbn[..maybe_isbn.len()-1]).to_string();
        
            let mut result = 0;
            let mut counter = 10;
            for character in sliced_isbn.chars() {
                result = match character.to_digit(10) {
                    Some(i) => result + counter * i,
                    None => return Err(format!("letter \"{}\" is not a digit", character).to_string()),
                };
                counter = counter - 1;
            }
    
            result = result + last_numeric;
    
            if result % 11 != 0 {
                return Err(format!("{} is not a valid ISBN", maybe_isbn));
            }
    
            return Ok(Isbn::Isbn10(maybe_isbn));
        }
    }    
}