pub(crate) fn process_function_string(input: &str) -> Option<(String, usize)> {
    // Поиск индекса начала имени функции
    let start_idx = input.find("function")?;

    // Ищем индекс открытой скобки после имени функции
    let open_bracket_idx = input[start_idx..].find("(").unwrap_or(0) + start_idx;

    // Ищем индекс закрывающей скобки после списка аргументов
    let close_bracket_idx = input[open_bracket_idx..].find(")").unwrap_or(0) + open_bracket_idx;

    // Ищем начало блока кода функции
    let block_start_idx = input[close_bracket_idx..].find("{").unwrap_or(0) + close_bracket_idx;

    // Ищем конец блока кода функции
    let mut block_end_idx = 0;
    let mut open_brackets = 0;

    for (idx, c) in input[block_start_idx..].char_indices() {
        if c == '{' {
            open_brackets += 1;
        } else if c == '}' {
            open_brackets -= 1;
            if open_brackets == 0 {
                block_end_idx = idx + block_start_idx;
                break;
            }
        }
    }

    // Извлекаем имя функции и аргументы
    let function_name = input[start_idx + "function".len()..open_bracket_idx].trim().to_string();
    let arguments = input[open_bracket_idx + 1..close_bracket_idx].split(',').count();

    Some((function_name, arguments))
}