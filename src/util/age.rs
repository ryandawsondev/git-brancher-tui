use anyhow::{bail, Result};

pub fn parse_age_days(val: &str) -> Result<u64> {
    let val = val.trim();
    let (digits, unit) = val.split_at(
        val.find(|c: char| c.is_alphabetic()).unwrap_or(val.len()),
    );
    let n: u64 = digits.parse().map_err(|_| anyhow::anyhow!("invalid number in '{val}'"))?;
    match unit.to_lowercase().as_str() {
        "" | "d" => Ok(n),
        "w" => Ok(n * 7),
        "m" => Ok(n * 30),
        other => bail!("invalid unit '{other}' in '{val}'. Use d, w, or m"),
    }
}
