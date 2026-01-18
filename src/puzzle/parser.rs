use std::fs;
use crate::puzzle::state::State;

pub fn parse_file(path: &str) -> Result<State, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut numbers = Vec::new();
    let mut size = None;

    for line in content.lines() {
        let clean_line = line.split('#').next().unwrap_or("").trim();
        if clean_line.is_empty() { 
            continue;
        }
        if size.is_none() {
            size = Some(clean_line.parse::<usize>().map_err(|_| "Invalid size")?);
        } else {
            for num in clean_line.split_whitespace() {
                numbers.push(num.parse::<u16>().map_err(|_| "Invalid number")?);
            }
        }
    }

    let n = size.ok_or("Size not found")?;
    if numbers.len() != n * n { 
        return Err("Board size mismatch".into());
    }
    
    Ok(State::new(numbers, n))
}