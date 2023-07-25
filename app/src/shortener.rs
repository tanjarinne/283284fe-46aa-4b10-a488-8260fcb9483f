pub fn generate_shortened_hash(input: &str) -> String {
  input.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;
  use regex::Regex;

  #[test]
  fn test_shortened_url_format() {
    let url = "https://foo.com/";
    let result = generate_shortened_hash(url);
    let pattern = r"^[a-zA-Z0-9]{9}$";
    let validator = Regex::new(pattern).unwrap();
    let is_valid = validator.is_match(&result);

    assert_eq!(result.len(), 9);
    assert_eq!(is_valid, true);
  }

  #[test]
  fn test_collission() {
    let url = "https://example.here/foo/bar";
    let result1 = generate_shortened_hash(url);
    let result2 = generate_shortened_hash(url);

    assert_eq!(result1, result2);
  }

  #[test]
  fn test_non_empty() {
    let url = "https://fizz.com/buzz?foo=bar";
    let result = generate_shortened_hash(url);

    assert_ne!(result, "");
  }
}
