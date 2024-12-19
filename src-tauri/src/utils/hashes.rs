use md5::digest::DynDigest;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum HashType {
    MD2,
    MD4,
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}

fn get_hasher(hash_type: HashType) -> Box<dyn DynDigest> {
    match hash_type {
        HashType::MD2 => Box::new(md2::Md2::default()),
        HashType::MD4 => Box::new(md4::Md4::default()),
        HashType::MD5 => Box::new(md5::Md5::default()),
        HashType::SHA1 => Box::new(sha1::Sha1::default()),
        HashType::SHA224 => Box::new(sha2::Sha224::default()),
        HashType::SHA256 => Box::new(sha2::Sha256::default()),
        HashType::SHA384 => Box::new(sha2::Sha384::default()),
        HashType::SHA512 => Box::new(sha2::Sha512::default()),
    }
}

#[tauri::command]
pub fn hash_generate(data: &str, hash_type: HashType, lowercase: bool) -> Result<String, String> {
    let mut hasher = get_hasher(hash_type);
    hasher.update(data.as_bytes());
    let hash = hasher.finalize_reset();
    let result = match lowercase {
        true => base16ct::lower::encode_string(&hash),
        false => base16ct::upper::encode_string(&hash),
    };
    Ok(result)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case(HashType::MD2, "D4541250B586296FCCE5DEA4463AE17F"; "Md2生成")]
    #[test_case(HashType::MD4, "585028AA0F794AF812EE3BE8804EB14A"; "Md4生成")]
    #[test_case(HashType::MD5, "E10ADC3949BA59ABBE56E057F20F883E"; "Md5生成")]
    #[test_case(HashType::SHA1, "7C4A8D09CA3762AF61E59520943DC26494F8941B"; "Sha1生成")]
    #[test_case(
        HashType::SHA224,
        "F8CDB04495DED47615258F9DC6A3F4707FD2405434FEFC3CBF4EF4E6";
        "Sha224生成"
    )]
    #[test_case(
        HashType::SHA256,
        "8D969EEF6ECAD3C29A3A629280E686CF0C3F5D5A86AFF3CA12020C923ADC6C92";
        "Sha256生成"
    )]
    #[test_case(HashType::SHA384, "0A989EBC4A77B56A6E2BB7B19D995D185CE44090C13E2984B7ECC6D446D4B61EA9991B76A4C2F04B1B4D244841449454"; "Sha384生成")]
    #[test_case(HashType::SHA512, "BA3253876AED6BC22D4A6FF53D8406C6AD864195ED144AB5C87621B6C233B548BAEAE6956DF346EC8C17F5EA10F35EE3CBC514797ED7DDD3145464E2A0BAB413"; "Sha512生成")]
    fn test_hash_generate(hash_type: HashType, excepted: &str) {
        // test uppercase

        let actual = hash_generate("123456", hash_type, false);
        assert!(actual.is_ok());
        assert_eq!(excepted, actual.unwrap());

        // test lowercase
        let actual = hash_generate("123456", hash_type, true);
        assert!(actual.is_ok());
        assert_eq!(excepted.to_lowercase(), actual.unwrap());
    }
}
