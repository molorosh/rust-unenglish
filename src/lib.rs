pub fn tragmog(mode: &UnenglishMode) -> &str {
    match mode {
        UnenglishMode::Diacritical => "=D",
        UnenglishMode::Japanglish => "=J"
    }
}

pub enum UnenglishMode {
    Japanglish,
    Diacritical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = tragmog(&UnenglishMode::Diacritical);
        assert_eq!(result, "=K");
    }
}
