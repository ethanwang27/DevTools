#[tauri::command]
pub fn base64_encode(data: &str) -> Result<String, String> {
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;
    let result = BASE64_STANDARD.encode(data);
    Ok(result)
}

#[tauri::command]
pub fn base64_decode(data: &str) -> Result<String, String> {
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;
    let result = BASE64_STANDARD.decode(data);
    match result {
        Ok(result) => Ok(String::from_utf8_lossy(result.as_slice()).to_string()),
        Err(_) => Err("Base64解码错误".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("123456", "MTIzNDU2"; "base64编码:无需补位")]
    #[test_case("1234", "MTIzNA=="; "base64编码:需补位")]
    fn test_base64_encode(original: &str, expected: &str) {
        let actual = base64_encode(original);
        assert!(actual.is_ok(), "base64编码失败");
        assert_eq!(expected, actual.unwrap(), "base编码结果错误");
    }

    #[test_case("MTIzNDU2", "123456"; "base64编码:无需补位")]
    #[test_case("MTIzNA==", "1234"; "base64编码:需补位")]
    fn test_base64_decode(original: &str, expected: &str) {
        let actual = base64_decode(original);
        assert!(actual.is_ok(), "base64编码失败");
        assert_eq!(expected, actual.unwrap(), "base编码结果错误");
    }
}
