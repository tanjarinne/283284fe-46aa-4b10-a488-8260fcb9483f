use ring::digest::{digest, SHA256};

pub fn generate_shortened_hash(input: &str) -> String {
  let hash = digest(&SHA256, input.as_bytes());
  let mut hex_hash = String::new();
  for byte in hash.as_ref() {
    hex_hash.push_str(&format!("{:02x}", byte));
  }

  hex_hash.chars().take(9).collect::<String>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use regex::Regex;

  #[test]
  fn test_shortened_url_format() {
    let url = "https://foo.com/";
    let result = generate_shortened_hash(url);
    let validator = Regex::new(r"^[a-zA-Z0-9]{9}$").unwrap();

    assert_eq!(result.len(), 9);
    assert_eq!(validator.is_match(&result), true);
    assert_eq!("0d43b1693", result);
  }

  #[test]
  fn test_collission() {
    let url = "https://example.here/foo/bar";
    let result1 = generate_shortened_hash(url);
    let result2 = generate_shortened_hash(url);
    let result3 = generate_shortened_hash(url);
    let result4 = generate_shortened_hash(url);

    assert_eq!(result1, result2);
    assert_eq!(result1, result3);
    assert_eq!(result2, result3);
    assert_eq!(result1, result4);
    assert_eq!(result2, result4);
    assert_eq!("9f161af1f", result1);
    assert_eq!("9f161af1f", result2);
    assert_eq!("9f161af1f", result3);
    assert_eq!("9f161af1f", result4);
  }

  #[test]
  fn test_non_empty() {
    let url = "https://fizz.com/buzz?foo=bar";
    let result = generate_shortened_hash(url);

    assert_ne!(result, "");
    assert_eq!("26dd9ca46", result);
  }
}
