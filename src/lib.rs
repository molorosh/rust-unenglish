use std::usize;


// the 'a notation helps to annotate the lifetime parameter so the 
// compiler knows which method input is being borrowed from 
pub fn transmogrify<'a>(text: &'a str, mode: &UnenglishMode) -> String {
    let original = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.?!";
    let choices = [
        "åƀçðèƒġħíĵķľṁñõþƣŗŝŧùʋŵχÿžÅßÇÐÈƑĠĦÍĴĶĽṀÑÕÞƢŖŜŦÙƲŴΧŸŽ,.?!",
        "大乃匸占巳斤追井l子力レ瓜冂回尸只尺乙匕凵ソ山乂平弓大乃匸占巳斤追井l子力レ瓜冂回尸只尺乙匕凵ソ山乂平弓、。？！",
        "nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM.,!?"
    ];
    let selected_index = match mode {
        UnenglishMode::Diacritical => 0,
        UnenglishMode::Japanglish => 1,
        UnenglishMode::JuliusCaesar => 2
    };
    let choice = choices[selected_index].to_string();
    print!(" >> selected_index {} << \n", selected_index);
    print!(" >> choices.len() {} << \n", choices.len());

    // we define a new String with nothing in it
    let mut s: String = String::new();

    for c in text.chars() { 
        // what index is it in the `original` String?
        let idx = original.find(c);
        print!("  >> c {} << \n", c.to_string());
        print!("  >> idx.is_some() {} << \n", idx.is_some().to_string());
        match idx {
            Option::None => { 
                print!("    >> idx None \n");
                // no match so just add the current character
                s.push(c);
            },
            Option::Some(v) => { 
                print!("    >> idx {} << \n", v.to_string());
                // there is a match so fetch the appropriate character
                let ch = choice.chars().nth(v).unwrap();
                s.push(ch);                
            }
        };
    }
    // finally return the String
    return s;
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
    fn demo() {
        let basic_string = "Hello, World.";
        print!(" >> basic_string {} << \n", basic_string);
        let result = transmogrify("Hello, World!", &UnenglishMode::Japanglish);
        print!(" >> result {} << \n", result);
        assert_eq!(result, "井巳レレ回、　山回尺レ占!");
    }

    #[test]
    fn it_works_for_japanglish() {
        let result = transmogrify("Hello, World!", &UnenglishMode::Japanglish);
        assert_eq!(result, "井巳レレ回、　山回尺レ占!");
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
