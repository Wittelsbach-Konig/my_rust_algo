pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        let current_bottles = start_bottles - i;
        let next_bottles = current_bottles - 1;

        let current_bottles_str = num_to_str(current_bottles);
        let next_bottles_str = num_to_str(next_bottles);

        // Формируем стих
        let verse = format!(
            "{} green bottle{} hanging on the wall,\n\
             {} green bottle{} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {} green bottle{} hanging on the wall.",
            current_bottles_str,
            plural_form(current_bottles),
            current_bottles_str,
            plural_form(current_bottles),
            next_bottles_str.to_lowercase(),
            plural_form(next_bottles),
        );

        verses.push(verse);
    }

    // Объединяем все стихи, разделяя их пустыми строками
    verses.join("\n\n")
}

fn num_to_str(num: u32) -> String {
    let num_str = match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "no",
    };
    num_str.to_string()
}

fn plural_form(num: u32) -> String {
    let plural = match num {
        1 => "",
        _ => "s",
    };
    plural.to_string()
}