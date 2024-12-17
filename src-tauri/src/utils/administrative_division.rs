use anyhow::Result;
use cached::proc_macro::cached;
use std::{clone::Clone, ops::Div};

/// 行政区划编码
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DivisionCode {
    /// 行政区划编码
    code: String,
    /// 行政区划名称
    name: String,
}

/// 省级行政区划
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProvinceDivision {
    /// 省名
    pub name: String,
    /// 编码
    pub code: String,
    /// 市级行政区划
    pub cities: Vec<CityDivision>,
}

/// 市级行政区划
#[derive(Debug, Clone, serde::Serialize)]
pub struct CityDivision {
    /// 市名
    pub name: String,
    /// 编码
    pub code: String,
    /// 区县行政区划
    pub districts: Vec<DistrictDivision>,
}

/// 区县行政区划
#[derive(Debug, Clone, serde::Serialize)]
pub struct DistrictDivision {
    /// 区县名
    pub name: String,

    /// 编码
    pub code: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DivisionInfo {
    pub province_name: String,
    pub city_name: String,
    pub district_name: String,
    pub code: String,
}

impl ProvinceDivision {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            cities: vec![],
        }
    }

    fn append_city(&mut self, cites: &mut Vec<CityDivision>) {
        self.cities.append(cites);
    }
}

impl CityDivision {
    fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            districts: vec![],
        }
    }

    fn append_district(&mut self, districts: &mut Vec<DistrictDivision>) {
        self.districts.append(districts);
    }
}

impl DistrictDivision {
    fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
        }
    }
}

impl DivisionInfo {
    pub fn new(province: &str, city: &str, district: &str, code: &str) -> Self {
        Self {
            province_name: province.to_string(),
            city_name: city.to_string(),
            district_name: district.to_string(),
            code: code.to_string(),
        }
    }
}
/// 获取所有行政区划信息
#[cached(result = true)]
pub fn get_all_division_codes() -> Result<Vec<DivisionCode>, String> {
    let csv_file = "resources/assets/administrative_division_code.csv";
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_file)
        .expect("解析行政区划编码CSV文件失败");

    let mut result = vec![];

    for division_code in reader.deserialize() {
        let code: DivisionCode = division_code.expect("解析CSV失败");
        result.push(code);
    }
    Ok(result)
}

/// 获取所有的省级行政区划信息（不包含香港、澳门、台湾）
#[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

#[tauri::command]
#[cached(result = true)]
pub fn get_all_provinces() -> Result<Vec<ProvinceDivision>, String> {
    let division_codes = get_all_division_codes().expect("获取省级行政区划信息失败");

    // 根据开始字符串、结尾字符串过滤行政区划
    let filter_division = |start: &str, end: &str| -> Vec<DivisionCode> {
        division_codes
            .iter()
            .filter(|item| -> bool {
                (start.is_empty() || item.code.starts_with(start))
                    && (end.is_empty() || item.code.ends_with(end))
            })
            .cloned()
            .collect()
    };

    // 获取市级行政区划信息
    let get_city_divisions = |province: &ProvinceDivision| -> Vec<CityDivision> {
        // 直辖市
        let municipality = vec!["北京市", "上海市", "重庆市", "天津市"];

        // 获取区县级行政区划信息： 区县级行政区划以其市级行政区划的开头四位开始
        let get_districts = |start: &str| {
            filter_division(start, "")
                .iter()
                .filter(|item| !item.code.ends_with("00"))
                .map(|item| DistrictDivision::new(item.name.as_str(), item.code.as_str()))
                .collect::<Vec<DistrictDivision>>()
        };

        // 市级行政区划编码以00结尾，以其省级行政区划的开头两位开始，中间部分为市级编码标识（直辖市默认中间两位为01）
        if municipality.contains(&province.name.as_str()) {
            // 直辖市
            let mut city = CityDivision::new(province.name.as_str(), province.code.as_str());
            let code = format!("{}01", &province.code[0..2]);
            let mut districts = get_districts(code.as_str());
            city.append_district(&mut districts);
            vec![city]
        } else {
            filter_division(&province.code[0..2], "00")
                .iter()
                .filter(|item| item.code != province.code) // 排除省
                .map(|item| {
                    let mut city = CityDivision::new(item.name.as_str(), item.code.as_str());
                    let mut districts = get_districts(&city.code[0..4]);
                    city.append_district(&mut districts);
                    city
                })
                .collect::<Vec<CityDivision>>()
        }
    };

    // 获取省级信息
    // 省级/直辖市行政区划编码以0000结尾
    let provinces = filter_division("", "0000")
        .iter_mut()
        .map(|division_code| -> ProvinceDivision {
            let mut province =
                ProvinceDivision::new(division_code.name.as_str(), division_code.code.as_str());
            province.append_city(&mut get_city_divisions(&&province));
            province
        })
        .collect::<Vec<ProvinceDivision>>();

    // 获取
    Ok(provinces)
}

/// # 获取行政区划信息
/// - province: 省名（必传)
/// - city: 市名
/// - district: 区县名
/// > 若city、district为空，则返回省的行政编码
/// > 若只有district为空，则返回市的行政编码
/// > 若三个参数都不为空，则返回区县的行政编码
pub fn get_division_code(
    province: &str,
    city: Option<&str>,
    district: Option<&str>,
) -> Result<DivisionInfo> {
    let all_province = get_all_provinces().expect("获取省级行政区划信息失败");
    let province = all_province
        .iter()
        .find(|item| item.name == province)
        .expect(format!("未找到{}", province).as_str());
    let mut result = DivisionInfo::new(
        &province.code,
        city.unwrap_or_default(),
        district.unwrap_or_default(),
        "",
    );

    if city.is_none() {
        result.code = province.code.clone();
        return Ok(result);
    }
    let city = province
        .cities
        .iter()
        .find(|item| item.name == city.unwrap())
        .expect(format!("未找到{}", city.unwrap()).as_str());

    if district.is_none() {
        result.code = city.code.clone();
        return Ok(result);
    }
    let district = city
        .districts
        .iter()
        .find(|item| item.name == district.unwrap())
        .expect(format!("未找到{}", district.unwrap()).as_str());
    result.code = district.code.clone();
    Ok(result)
}

/// 随机获取行政区划编码
pub fn random_division_code() -> Result<DivisionInfo> {
    use super::common::random::random_item;

    let all_province = get_all_provinces().expect("获取省级行政区划信息失败");
    let province = random_item(&all_province).expect("随机获取省份失败");
    let city = random_item(&province.cities).expect("随机获取市失败");
    let mut result = DivisionInfo::new(&province.name, &city.name, "", "");
    if city.districts.len() == 0 {
        result.code = city.code.clone();
        return Ok(result);
    }
    let district = random_item(&city.districts).expect("随机获取区县失败");
    result.district_name = district.name.clone();
    result.code = district.code.clone();
    return Ok(result);
}

/// # 获取行政区划名称信息
/// ## params
/// - division_code: 行政区划编码
/// ## return
/// 返回省名、市名、区县名的字符串元组 (province_name, city_name, district_name)
pub fn get_division_name(division_code: &str) -> Option<(String, String, String)> {
    let province_prefix = division_code[0..2].to_owned();
    let all_provinces = get_all_provinces().expect("获取省份信息失败");
    let province = all_provinces
        .iter()
        .find(|item| item.code.starts_with(province_prefix.as_str()));
    if province.is_none() {
        return None;
    }
    let province = province.unwrap().clone();

    // 直辖市没有市级行政区划，get_all_provinces()返回的信息中直辖市的市级行政区划还是直辖市
    let municipality = vec![
        "110000", //北京
        "120000", //天津
        "310000", //上海
        "500000", //重庆
    ];
    // 判断是否是直辖市，若是则city_prefix为直辖市前缀再加00，否则为传入的division_code前四位
    let city_prefix = if municipality
        .iter()
        .any(|item| item.starts_with(province_prefix.as_str()))
    {
        format!("{}00", province_prefix)
    } else {
        division_code[0..4].to_string()
    };
    let city = province
        .cities
        .iter()
        .find(|item| item.code.starts_with(city_prefix.as_str()));

    if city.is_none() {
        return Some((province.name, "".to_string(), "".to_string()));
    }
    let city = city.unwrap().clone();
    let district = city
        .districts
        .iter()
        .find(|item| item.code == division_code);
    if district.is_none() {
        Some((province.name, city.name, "".to_string()))
    } else {
        Some((province.name, city.name, district.unwrap().clone().name))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::administrative_division::*;

    #[test]
    fn test_get_administrative_division_code() {
        let code_counts = 3205;
        let result = get_all_division_codes();
        assert!(result.is_ok(), "获取行政区划编码失败");
        let codes = result.unwrap();
        assert_eq!(code_counts, codes.len());
        assert_eq!(codes[0].code, "110000");
        assert_eq!(codes[0].name, "北京市");
    }

    #[test]
    fn test_get_all_provinces() {
        let excepted_provinces_count = 31;
        let result = get_all_provinces();
        assert!(result.is_ok(), "获取省份信息失败");
        let actual_provinces = result.unwrap();
        // 获取到的行政区划数量相同
        assert_eq!(excepted_provinces_count, actual_provinces.len());
        // 直辖市
        let beijing = actual_provinces.iter().find(|item| item.name == "北京市");
        assert!(beijing.is_some(), "未获取到直辖市：北京");
        let beijing = beijing.unwrap();
        assert_eq!(1, beijing.cities.len(), "直辖市的下属市级行政区划数量有误");
        assert_eq!(
            "110000", beijing.cities[0].code,
            "直辖市的下属市级行政区划编码有误"
        );
        assert_eq!(
            16,
            beijing.cities[0].districts.len(),
            "直辖市的下属区县级行政区划编码有误"
        );

        // 省
        let shaanxi = actual_provinces.iter().find(|item| item.name == "陕西省");
        assert!(shaanxi.is_some(), "未获取到陕西省");
        let shanxi = shaanxi.unwrap();
        assert_eq!(10, shanxi.cities.len(), "陕西省的市级行政区数量错误");
        assert_eq!(
            13,
            shanxi.cities[0].districts.len(),
            "西安市的区县级行政区数量错误"
        )
    }

    #[test]
    fn test_get_division_code() {
        let excepted_code = "110000";
        let actual = get_division_code("北京市", None, None);
        assert!(actual.is_ok(), "未找到北京市行政编码");
        assert_eq!(
            excepted_code,
            actual.unwrap().code.as_str(),
            "北京市行政编码错误"
        );

        let excepted_code = "110000";
        let actual = get_division_code("北京市", Some("北京市"), None);
        assert!(actual.is_ok(), "未找到北京市行政编码");
        assert_eq!(
            excepted_code,
            actual.unwrap().code.as_str(),
            "北京市行政编码错误"
        );

        let excepted_code = "110113";
        let actual = get_division_code("北京市", Some("北京市"), Some("顺义区"));
        assert!(actual.is_ok(), "未找到北京市行政编码");
        assert_eq!(
            excepted_code,
            actual.unwrap().code.as_str(),
            "北京市行政编码错误"
        );

        // 上海、浙江都有普陀区
        let excepted_code = "330903";
        let actual = get_division_code("浙江省", Some("舟山市"), Some("普陀区"));
        assert!(actual.is_ok(), "未找到浙江省舟山市普陀区行政编码");
        assert_eq!(
            excepted_code,
            actual.unwrap().code.as_str(),
            "未找到浙江省舟山市普陀区行政编码错误"
        );
    }

    #[test]
    fn test_get_division_name() {
        // 无效的行政区划编码
        let actual = get_division_name("999999");
        assert!(actual.is_none());

        let expected = (
            "江苏省".to_string(),
            "常州市".to_string(),
            "武进区".to_string(),
        );
        let actual = get_division_name("320412");
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());

        let expected = (
            "北京市".to_string(),
            "北京市".to_string(),
            "东城区".to_string(),
        );
        let actual = get_division_name("110101");
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }
}
