pub(crate) fn process_function_string(input: &str) -> Option<(String, usize)> {
    // Поиск индекса начала имени функции
    let start_idx = input.find("function")?;

    // Ищем индекс открытой скобки после имени функции
    let open_bracket_idx = input[start_idx..].find("(").unwrap_or(0) + start_idx;

    // Ищем индекс закрывающей скобки после списка аргументов
    let close_bracket_idx = input[open_bracket_idx..].find(")").unwrap_or(0) + open_bracket_idx;

    // Извлекаем имя функции и аргументы
    let function_name = input[start_idx + "function".len()..open_bracket_idx]
        .trim()
        .to_string();
    let arguments = input[open_bracket_idx + 1..close_bracket_idx]
        .split(',')
        .count();

    Some((function_name, arguments))
}
