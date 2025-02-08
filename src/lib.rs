
// the 'a notation helps to annotate the lifetime parameter so the 
// compiler knows which method input is being borrowed from 
pub fn transmogrify<'a>(text: &'a str, mode: &UnenglishMode) -> String {
    let original = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.?!";
    let choices = [
        " åƀçðèƒġħíĵķľṁñõþƣŗŝŧùʋŵχÿžÅßÇÐÈƑĠĦÍĴĶĽṀÑÕÞƢŖŜŦÙƲŴΧŸŽ,.?!",
        "　大乃匸占巳斤追井l子力レ瓜冂回尸只尺乙匕凵ソ山乂平弓大乃匸占巳斤追井l子力レ瓜冂回尸只尺乙匕凵ソ山乂平弓、。？！",
        " nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM.,!?"
    ];
    let selected_index = match mode {
        UnenglishMode::Diacritical => 0,
        UnenglishMode::Japanglish => 1,
        UnenglishMode::JuliusCaesar => 2
    };
    let choice = choices[selected_index].to_string();
    let mut s: String = String::new();
    for c in text.chars() { 
        let idx = original.find(c);
        match idx {
            Option::None => { 
                s.push(c);
            },
            Option::Some(v) => { 
                let ch = choice.chars().nth(v).unwrap();
                s.push(ch);                
            }
        };
    }
    return s;
}

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
        assert_eq!(result, "井巳レレ回、　山回尺レ占！");
    }

    #[test]
    fn it_works_for_diacritical() {
        let result = transmogrify("Hello, World!", &UnenglishMode::Diacritical);
        assert_eq!(result, "Ħèľľõ, Ŵõŗľð!");
    }

    #[test]
    fn it_works_for_julius_caesar() {
        let result = transmogrify("Hello, World!", &UnenglishMode::JuliusCaesar);
        assert_eq!(result, "Uryyb. Jbeyq?");
    }
    
    #[test]
    fn it_works_for_julius_caesar_reversibly() {
        let result = transmogrify("Uryyb. Jbeyq?", &UnenglishMode::JuliusCaesar);
        assert_eq!(result, "Hello, World!");
    }
}
