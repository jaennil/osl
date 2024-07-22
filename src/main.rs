use arboard::Clipboard;
use enigo::Direction::{Click, Press, Release};
use enigo::{Enigo, Key, Keyboard, Settings};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // control works only on xorg in my case
    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('a'), Click).unwrap();
    enigo.key(Key::Control, Release).unwrap();

    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('c'), Click).unwrap();
    enigo.key(Key::Control, Release).unwrap();

    let mut clipboard = Clipboard::new().unwrap();
    let input = clipboard.get_text().unwrap();
    let translated = translate(&input);
    clipboard.set_text(translated).unwrap();

    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('v'), Click).unwrap();
    enigo.key(Key::Control, Release).unwrap();
}

fn translate(text: &str) -> String {
    text.split_whitespace()
        .map(|word| match word.chars().count() {
            1 => match word {
                "В" | "К" | "С" | "в" | "к" | "с" => format!("{}ъ", word),
                _ => word.to_string(),
            },
            3.. => translate_word(word),
            _ => word.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let mut result = word.to_string();
    let cur_char: &char;
    if word.chars().count() > 2 {
        result = result.replace("его", "аго");
        result = result.replace("ого", "аго");
        result = result.replace("ая", "ыя");
        result = result.replace("её", "ея");
        result = result.replace("ее", "ея");
        result = result.replace("ие", "iя");
        result = result.replace("иё", "iя");
        result = result.replace("ия", "iя");

        result = result.replace("ии", "iи");
        result = result.replace("ий", "iй");

        result = result.replace("иу", "iу");
        result = result.replace("иы", "iы");
        result = result.replace("иа", "iа");
        result = result.replace("ио", "iо");
        result = result.replace("иэ", "iэ");
        result = result.replace("ию", "iю");

        result = result.replace("бе", "бѣ");
        result = result.replace("ве", "вѣ");
        result = result.replace("вё", "вѣ");
        result = result.replace("де", "дѣ");
        result = result.replace("же", "жѣ");
        result = result.replace("зе", "зѣ");
        result = result.replace("ке", "кѣ");
        result = result.replace("ле", "лѣ");
        result = result.replace("не", "нѣ");
        result = result.replace("мё", "мѣ");
        result = result.replace("ме", "мѣ");
        result = result.replace("пе", "пѣ");
        result = result.replace("рё", "рѣ");
        result = result.replace("ре", "рѣ");
        result = result.replace("се", "сѣ");
        result = result.replace("те", "тѣ");
        result = result.replace("тё", "тѣ");
        result = result.replace("це", "цѣ");

        result = result.replace("Бе", "Бѣ");
        result = result.replace("Ве", "Вѣ");
        result = result.replace("вё", "Вѣ");
        result = result.replace("Де", "Дѣ");
        result = result.replace("Же", "Жѣ");
        result = result.replace("Зе", "Зѣ");
        result = result.replace("Ке", "Кѣ");
        result = result.replace("Ле", "Лѣ");
        result = result.replace("Не", "Нѣ");
        result = result.replace("Ме", "Мѣ");
        result = result.replace("Мё", "Мѣ");
        result = result.replace("Пе", "Пѣ");
        result = result.replace("Ре", "Рѣ");
        result = result.replace("Рё", "Рѣ");
        result = result.replace("Се", "Сѣ");
        result = result.replace("Те", "тѣ");
        result = result.replace("Тё", "тѣ");
        result = result.replace("Це", "Цѣ");

        result = result.replace("Ед", "ѣд");
        result = result.replace("ед", "Ѣд");

        result = result.replace("ри", "рi");
        result = result.replace("ни", "нi");
    }

    let chars = result.chars().collect::<Vec<_>>();
    let last_char = chars
        .last()
        .expect("impossible last char to be empty as i check chars amount to be > 2");
    let mut flag = false;

    if *last_char == '.'
        || *last_char == ','
        || *last_char == ';'
        || *last_char == ':'
        || *last_char == '!'
        || *last_char == '?'
        || *last_char == ')'
        || *last_char == ']'
        || *last_char == '"'
        || *last_char == '»'
    {
        result.pop();
        cur_char = chars.get(chars.len() - 2).unwrap();
    } else {
        cur_char = last_char;
        flag = true;
    }

    if *cur_char == 'б'
        || *cur_char == 'в'
        || *cur_char == 'г'
        || *cur_char == 'д'
        || *cur_char == 'ж'
        || *cur_char == 'з'
        || *cur_char == 'к'
        || *cur_char == 'л'
        || *cur_char == 'м'
        || *cur_char == 'н'
        || *cur_char == 'п'
        || *cur_char == 'р'
        || *cur_char == 'с'
        || *cur_char == 'т'
        || *cur_char == 'ф'
        || *cur_char == 'х'
        || *cur_char == 'ц'
        || *cur_char == 'ч'
        || *cur_char == 'ш'
        || *cur_char == 'щ'
    {
        result += "ъ";
    } else if *cur_char == 'ы' {
        result.pop();
        result.push('ъ');
    }

    if !flag {
        result.push(*last_char);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::translate;

    #[test]
    fn test1() {
        let input = "Оформить сегодня текст в ретро-стиле — непростая задача, поэтому мы создали этот конвертер, выполняющий автоматическое преобразование в старославянский стиль, его даже можно назвать переводчиком на старорусский";
        let expected = "Оформить сагодня тѣкстъ въ рѣтро-стилѣ — нѣпростыя задача, поэтому мы создали этотъ конвѣртѣръ, выполняющiй автоматическое прѣобразованiя въ старославянскiй стиль, аго дажѣ можно назвать пѣрѣводчикомъ на старорусскiй";
        let actual = translate(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let input = "В итоге";
        let expected = "Въ итоге";
        let actual = translate(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let input = r#"С помощью этого конвертера вы сможете оформить текст по старым правилам, точно он был написан в середине XIX — начале XX века, до революции и до так называемой "орфографической реформы", упростившей правила написания текстов. Если сегодня ретро-орфографический стиль — это просто оригинальный стилистический эффект, то в прежнее время все тексты в Царской России писались именно таким образом в соответствии с официально принятыми орфографическими правилами. Это было обязательное требование, особенно для деловых документов, писем, литературных, художественных произведений, церковнославянских текстов."#;
        let expected = r#"Съ помощью этаго конвѣртѣра вы сможѣтѣ оформить тѣкстъ по старымъ правиламъ, точно он былъ написанъ въ сѣрѣдинѣ XIX — началѣ XX вѣка, до рѣволюцiи и до такъ называемой "орфографической рѣформы", упростившей правила написанiя тѣкстовъ. Если сагодня рѣтро-орфографическiй стиль — это просто орiгинальный стилистическiй эффектъ, то въ прѣжнѣя врѣмя всѣ тѣкстъ въ Царской Россiи писались имѣнно такимъ образомъ въ соотвѣтствiи съ офицiально прiнятыми орфографическими правилами. Это было обязатѣльное трѣбованiя, особѣнно для дѣловыхъ докумѣнтовъ, писѣмъ, литѣратурныхъ, художѣствѣнныхъ произвѣдѣнiй, цѣрковнославянскихъ тѣкстовъ."#;
        let actual = translate(input);
        assert_eq!(expected, actual);
    }
}
