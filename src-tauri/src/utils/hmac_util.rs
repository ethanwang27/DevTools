use crate::utils::hashes::HashType;
use base64::Engine;
use digest::{
    block_buffer::Eager,
    consts::U256,
    core_api::{BlockSizeUser, BufferKindUser, CoreProxy, FixedOutputCore, UpdateCore},
    typenum::{IsLess, Le},
    DynDigest, HashMarker,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use typenum::NonZero;

/// hmac生成结果
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HmacResult {
    /// hmac结果的hex编码
    pub hex: String,

    /// hmac结果的base64编码
    pub base64: String,
}

///  generate hmac
///  return hex and base64 encoded hmac result
#[tauri::command]
pub fn generate_hmac(key: &str, data: &str, hash_type: HashType) -> Result<HmacResult, String> {
    let result = match hash_type {
        HashType::MD2 => generate::<md2::Md2>(key, data),
        HashType::MD4 => generate::<md4::Md4>(key, data),
        HashType::MD5 => generate::<md5::Md5>(key, data),
        HashType::SHA1 => generate::<sha1::Sha1>(key, data),
        HashType::SHA224 => generate::<sha2::Sha224>(key, data),
        HashType::SHA256 => generate::<sha2::Sha256>(key, data),
        HashType::SHA384 => generate::<sha2::Sha384>(key, data),
        HashType::SHA512 => generate::<sha2::Sha512>(key, data),
    }?;
    Ok(HmacResult {
        hex: hex::encode(&result),
        base64: base64::engine::general_purpose::STANDARD.encode(&result),
    })
}

/// generate hmac from key and data, and return Vec<u8>
fn generate<D>(key: &str, data: &str) -> Result<Vec<u8>, String>
where
    D: CoreProxy,
    D::Core: HashMarker
        + UpdateCore
        + FixedOutputCore
        + BufferKindUser<BufferKind = Eager>
        + Default
        + Clone,
    <D::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<D::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    let mut mac = Hmac::<D>::new_from_slice(key.as_bytes())
        .map_err(|err| format!("Hmac生成失败: {}", err))?;
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let result = result.into_bytes().to_vec();
    Ok(result)
}

#[cfg(test)]
mod test {

    use super::*;
    use test_case::test_case;

    #[test_case(HashType::MD2, "22e920254b27d4e70d78670058be1bf9", "IukgJUsn1OcNeGcAWL4b+Q=="; "hmac_md2 test")]
    #[test_case(HashType::MD4, "2c3a5622e646594abf085b2741795328", "LDpWIuZGWUq/CFsnQXlTKA=="; "hmac_md4 test")]
    #[test_case(HashType::MD5, "9028905855a81e0f3f76a72212e6c488", "kCiQWFWoHg8/dqciEubEiA=="; "hmac_md5 test")]
    #[test_case(HashType::SHA1, "4873e80b320876fef98bd5857a7146db412007c7", "SHPoCzIIdv75i9WFenFG20EgB8c="; "hmac_sha1 test")]
    #[test_case(HashType::SHA224, "4f358a44c7bfaf92a2aa3e015d06a6913cc22dd1d684efbb3ce71755", "TzWKRMe/r5Kiqj4BXQamkTzCLdHWhO+7POcXVQ=="; "hmac_Sha224 test")]
    #[test_case(HashType::SHA256, "83b3eb2788457b46a2f17aaa048f795af0d9dabb8e5924dd2fc0ea682d929fe5", "g7PrJ4hFe0ai8XqqBI95WvDZ2ruOWSTdL8DqaC2Sn+U="; "hmac_Sha256 test")]
    #[test_case(HashType::SHA384, "1959aa7cce0c3aef75dd794d1a63bdce95fcac6ef327d8396ffc9b0f99d5a78387be7ea14c6fccaeae25c30a05e3a488", "GVmqfM4MOu913XlNGmO9zpX8rG7zJ9g5b/ybD5nVp4OHvn6hTG/Mrq4lwwoF46SI"; "hmac_Sha384 test")]
    #[test_case(HashType::SHA512, "6c9c251365f3507dc923023fd8e180925eee0dc0bb467d156edc21b9889fc1115cbd7a948090abb59b31718e83978900d7582993392d90d2835ee13c9f2fbb69", "bJwlE2XzUH3JIwI/2OGAkl7uDcC7Rn0VbtwhuYifwRFcvXqUgJCrtZsxcY6Dl4kA11gpkzktkNKDXuE8ny+7aQ=="; "hmac_Sha512 test")]
    fn test_generate_hmac(hash_type: HashType, excepted_hex: &str, expected_base64: &str) {
        let key = "123456";
        let data = "hello world";
        let result = generate_hmac(key, data, hash_type);
        assert!(result.is_ok(), "生成Hmac失败");
        let result = result.unwrap();
        assert_eq!(result.hex, excepted_hex);
        assert_eq!(result.base64, expected_base64);
    }
}
