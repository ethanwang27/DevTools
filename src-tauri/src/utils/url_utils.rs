#[tauri::command]
pub fn url_encode(data: &str) -> Result<String, String> {
    use urlencoding::encode;
    let result = encode(data);
    Ok(result.to_string())
}

#[tauri::command]
pub fn url_decode(data: &str) -> Result<String, String> {
    use urlencoding::decode;
    let result = decode(data);
    match result {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(e.to_string()),
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn test_url_encode() {
        let data = "https://www.google.com/search?q=hello+world";
        let result = super::url_encode(data).unwrap();
        assert_eq!(result, "https%3A%2F%2Fwww.google.com%2Fsearch%3Fq%3Dhello%2Bworld");
    }

    #[test]
    fn test_url_decode() {
        let data = "https%3A%2F%2Fwww.google.com%2Fsearch%3Fq%3Dhello%2Bworld";
        let result = super::url_decode(data).unwrap();
        assert_eq!(result, "https://www.google.com/search?q=hello+world");
    }   
}
