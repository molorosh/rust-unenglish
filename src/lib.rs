
// the 'a notation helps to annotate the lifetime parameter so the 
// compiler knows which method input is being borrowed from 
pub fn tragmog<'a>(_text: &'a str, mode: &UnenglishMode) -> &'a str {
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
        let result = tragmog("hello world", &UnenglishMode::Japanglish);
        assert_eq!(result, "=J");
    }

    #[test]
    fn it_works_for_diacritical() {
        let result = tragmog("hello world", &UnenglishMode::Diacritical);
        assert_eq!(result, "=D");
    }
}
