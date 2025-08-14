pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut _plants = Vec::new();
    let lines: Vec<&str> = diagram.lines().collect();

    let i = name_to_number(student).expect("Student doesn't exist");

    let chars1: Vec<char> = lines[0].chars().collect();
    let chars2: Vec<char> = lines[1].chars().collect();

    for c in [chars1[i], chars1[i + 1], chars2[i], chars2[i + 1]] {
        match c {
            'R' => _plants.push("radishes"),
            'C' => _plants.push("clover"),
            'V' => _plants.push("violets"),
            'G' => _plants.push("grass"),
            _ => panic!(),
        }
    }

    _plants
}

fn name_to_number(name: &str) -> Option<usize> {
    match name {
        "Alice" => Some(0),
        "Bob" => Some(2),
        "Charlie" => Some(4),
        "David" => Some(6),
        "Eve" => Some(8),
        "Fred" => Some(10),
        "Ginny" => Some(12),
        "Harriet" => Some(14),
        "Ileana" => Some(16),
        "Joseph" => Some(18),
        "Kincaid" => Some(20),
        "Larry" => Some(22),
        _ => None,
    }
}
