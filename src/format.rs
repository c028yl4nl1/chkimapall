pub fn formatLine<T: AsRef<str>>(line: T) -> Vec<String> {
    let lines = line.as_ref();

    let mut formatline = Vec::new();

    for line in lines.lines() {
        if line.contains(":") {
            let split: Vec<&str> = line.split(":").collect();
            if split.len() == 2 {
                formatline.push(line.to_string())
            }
        } else if line.contains("|") {
            let split: Vec<&str> = line.split("|").collect();
            if split.len() == 2 {
                formatline.push(format!("{}:{}", split[0], split[1]));
            }
        }
    }

    formatline
}
