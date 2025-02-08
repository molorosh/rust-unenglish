
// the 'a notation helps to annotate the lifetime parameter so the 
// compiler knows which method input is being borrowed from 
pub fn transmogrify<'a>(_text: &'a str, mode: &UnenglishMode) -> &'a str {
    let selected_index = match mode {
        UnenglishMode::Diacritical => 0,
        UnenglishMode::Japanglish => 1,
        UnenglishMode::JuliusCaesar => 2
    };
    match mode {
        UnenglishMode::Diacritical => "=D",
        UnenglishMode::Japanglish => "=J",
        UnenglishMode::JuliusCaesar => "13"
    }
}

// default:     abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.?!
// japanglish:  大乃匸占巳斤追井l子力レ瓜冂回尸只尺乙匕凵ソ山乂平弓大乃匸占巳斤追井l子力レ瓜冂回尸只尺乙匕凵ソ山乂平弓、。？！
// diacritical: åƀçðèƒġħíĵķľṁñõþƣŗŝŧùʋŵχÿžÅßÇÐÈƑĠĦÍĴĶĽṀÑÕÞƢŖŜŦÙƲŴΧŸŽ,.?!
// rot-13:     "nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM.,!?"


pub enum UnenglishMode {
    Japanglish,
    Diacritical,
    JuliusCaesar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_japanglish() {
        let result = transmogrify("Hello, World!", &UnenglishMode::Japanglish);
        assert_eq!(result, "井巳レレ回, 山回尺レ占!");
    } 

    #[test]
    fn it_works_for_diacritical() {
        let result = transmogrify("Hello, World!", &UnenglishMode::Diacritical);
        assert_eq!(result, "Ħèľľõ, Ŵõŗľð!");
    }

    #[test]
    fn it_works_for_julius_caesar() {
        let result = transmogrify("Hello, World!", &UnenglishMode::JuliusCaesar);
        assert_eq!(result, "Olssv, Dvysk!");
    }
}
