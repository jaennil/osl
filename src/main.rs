fn main() {}

#[allow(dead_code)]
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
    let mut res = word.to_string();
    let cur_char: &char;
    if word.chars().count() > 2 {
        res = res.replace("его", "аго");
        res = res.replace("ого", "аго");
        res = res.replace("ая", "ыя");
        res = res.replace("её", "ея");
        res = res.replace("ее", "ея");
        res = res.replace("ие", "iя");
        res = res.replace("иё", "iя");
        res = res.replace("ия", "iя");

        res = res.replace("ии", "iи");
        res = res.replace("ий", "iй");

        res = res.replace("иу", "iу");
        res = res.replace("иы", "iы");
        res = res.replace("иа", "iа");
        res = res.replace("ио", "iо");
        res = res.replace("иэ", "iэ");
        res = res.replace("ию", "iю");

        res = res.replace("бе", "бѣ");
        res = res.replace("ве", "вѣ");
        res = res.replace("вё", "вѣ");
        res = res.replace("де", "дѣ");
        res = res.replace("же", "жѣ");
        res = res.replace("зе", "зѣ");
        res = res.replace("ке", "кѣ");
        res = res.replace("ле", "лѣ");
        res = res.replace("не", "нѣ");
        res = res.replace("мё", "мѣ");
        res = res.replace("ме", "мѣ");
        res = res.replace("пе", "пѣ");
        res = res.replace("рё", "рѣ");
        res = res.replace("ре", "рѣ");
        res = res.replace("се", "сѣ");
        res = res.replace("те", "тѣ");
        res = res.replace("тё", "тѣ");
        res = res.replace("це", "цѣ");

        res = res.replace("Бе", "Бѣ");
        res = res.replace("Ве", "Вѣ");
        res = res.replace("вё", "Вѣ");
        res = res.replace("Де", "Дѣ");
        res = res.replace("Же", "Жѣ");
        res = res.replace("Зе", "Зѣ");
        res = res.replace("Ке", "Кѣ");
        res = res.replace("Ле", "Лѣ");
        res = res.replace("Не", "Нѣ");
        res = res.replace("Ме", "Мѣ");
        res = res.replace("Мё", "Мѣ");
        res = res.replace("Пе", "Пѣ");
        res = res.replace("Ре", "Рѣ");
        res = res.replace("Рё", "Рѣ");
        res = res.replace("Се", "Сѣ");
        res = res.replace("Те", "тѣ");
        res = res.replace("Тё", "тѣ");
        res = res.replace("Це", "Цѣ");

        res = res.replace("Ед", "ѣд");
        res = res.replace("ед", "Ѣд");

        res = res.replace("ри", "рi");
        res = res.replace("ни", "нi");
    }

    let chars = res.chars().collect::<Vec<_>>();
    let last_char = chars.last().unwrap();

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
        res = res[..res.len() - 1].to_string();
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
        res += "ъ";
    } else if *cur_char == 'ы' {
        res = res[..res.len() - 1].to_string();
        res.push('ъ');
    }

    if !flag {
        res.push(*last_char);
    }

    return res;
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
}
