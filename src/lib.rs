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
    fn it_works_for_japanglish() {
        let result = tragmog(&UnenglishMode::Japanglish);
        assert_eq!(result, "=J");
    }

    #[test]
    fn it_works_for_diacritical() {
        let result = tragmog(&UnenglishMode::Diacritical);
        assert_eq!(result, "=D");
    }
}
