use std::process::id;

#[allow(dead_code, unused_variables)]
use super::administrative_division::*;
use crate::utils::administrative_division::get_division_code;
use crate::utils::datetime::*;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IdNo(String);

/// 性别
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    Male = 1,
    Female = 2,
}

impl IdNo {
    /// 获取身份证号
    #[allow(dead_code)]
    pub fn new(
        province: &str,
        city: &str,
        district: &str,
        birthday: &str,
        gender: Gender,
    ) -> Result<IdNo> {
        if Self::parse_to_datetime(birthday).is_err() {
            return Err(anyhow!("Birthday传入有误"));
        }

        let division_info = get_division_code(province, Some(city), Some(district))?;
        Self::generate_id_no(&division_info.code, &birthday, gender)
    }

    /// 随机生成身份证号
    #[allow(dead_code)]
    pub fn random() -> Result<Self> {
        use super::common::random::{rand_int, random_datatime};
        let gender = match rand_int(1, 3) {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::Male,
        };
        let birthday = random_datatime().format("%Y%m%d").to_string();
        let division_info = random_division_code()?;
        Self::generate_id_no(&division_info.code, &birthday, gender)
    }

    /// 生成身份证号
    fn generate_id_no(division_code: &str, birthday: &str, gender: Gender) -> Result<IdNo> {
        let sequence = Self::get_sequence(gender);
        let master_number = format!("{}{}{}", division_code, birthday, sequence);
        let id_no = format!("{}{}", master_number, Self::get_check_code(&master_number)?);
        Ok(IdNo(id_no))
    }

    /// 获取身份证中的序列号
    fn get_sequence(gender: Gender) -> i32 {
        use super::common::random::{rand_even_int, rand_odd_int};
        match gender {
            Gender::Male => rand_odd_int(1, 999),
            Gender::Female => rand_even_int(1, 999),
        }
    }

    /// 获取身份证号的最后一位(校验码)
    fn get_check_code(master_number: &str) -> Result<String> {
        if master_number.is_empty() {
            return Err(anyhow::anyhow!("Master number is empty"));
        }
        let weight = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
        let check_code = ['1', '0', 'X', '9', '8', '7', '5', '5', '4', '3', '2'];

        let all_sum: i32 = master_number
            .chars()
            .map(|item| item.to_digit(10).expect("Invalid item") as i32)
            .zip(weight.into_iter())
            .map(|(item, weight)| item * weight)
            .sum();
        let index = (all_sum % 11) as usize;
        let result = check_code.get(index).expect("获取校验码数组下标错误");
        Ok(result.to_string())
    }

    /// 字符串解析成日期
    fn parse_to_datetime(date: &str) -> Result<DateTime<Local>> {
        if date.is_empty() {
            return Err(anyhow::anyhow!("Date is empty"));
        }
        if date.len() != 8 {
            return Err(anyhow::anyhow!("Date length is invalid"));
        }

        let year = date[0..4].to_string().parse::<i32>()?;
        let month = date[4..6].to_string().parse::<u32>()?;
        let day = date[6..8].to_string().parse::<u32>()?;
        Ok(Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap())
    }
}

/// 身份信息
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Person {
    /// 籍贯：省
    pub province: Option<String>,
    /// 籍贯：市
    pub city: Option<String>,
    /// 籍贯：区县
    pub district: Option<String>,
    /// 出生日期
    #[serde(
        serialize_with = "serialize_dt_local",
        deserialize_with = "deserializer_dt_local"
    )]
    pub birthday: Option<DateTime<Local>>,
    /// 性别
    pub gender: Option<Gender>,

    pub id_no: Option<IdNo>,
}

/// 获取身份证号码
#[tauri::command]
pub fn get_id_no(person: Person) -> Result<Person, String> {
    println!("获取身份证号码：传入的信息-{:?}", person);
    use super::common::random::{rand_int, random_datatime};
    let division_info;
    if person.province.is_some() || person.city.is_some() || person.district.is_some() {
        division_info = get_division_code(
            person.province.unwrap().as_str(),
            Some(person.city.unwrap().as_str()),
            Some(person.district.unwrap().as_str()),
        )
        .expect("获取行政区划信息失败");
    } else {
        division_info = random_division_code().expect("随机获取行政区划信息失败");
    }

    let birthday = person.birthday.unwrap_or(random_datatime());
    let gender = person.gender.unwrap_or_else(|| {
        if rand_int(1, 3) == 1 {
            Gender::Male
        } else {
            Gender::Female
        }
    });
    println!(
        "获取身份证号码：行政区划-{:?}; 出生日期-{:?}；性别-{:?}",
        division_info, birthday, gender
    );
    let id = IdNo::generate_id_no(
        division_info.code.as_str(),
        birthday.format("%Y%m%d").to_string().as_str(),
        gender.clone(),
    )
    .expect("生成省份证号码错误");
    return Ok(Person {
        province: Some(division_info.province_name.clone()),
        city: Some(division_info.city_name.clone()),
        district: Some(division_info.district_name.clone()),
        gender: Some(gender),
        birthday: Some(birthday),
        id_no: Some(id),
    });
}

/// 解析身份证号码
#[tauri::command]
pub fn parse_id_no(id_no: &str) -> Result<Person, String> {
    if id_no.is_empty() || id_no.len() != 18 {
        return Err("身份证号码号码位数不正确".to_string());
    }
    let division_code = &id_no[0..6];
    let division = get_division_name(division_code).expect("身份证号码不正确：行政区划解析失败");
    let offset = Local::now().offset().clone();
    let birthday = format!(
        "{}-{}-{} 00:00:00",
        &id_no[6..10],
        &id_no[10..12],
        &id_no[12..14]
    );
    let birthday = NaiveDateTime::parse_from_str(&birthday, "%Y-%m-%d %H:%M:%S")
        .expect("身份证号码不正确：出生日期解析失败")
        .checked_sub_offset(offset)
        .unwrap()
        .and_utc()
        .with_timezone(&Local);
    let sequence = id_no[14..17].to_string().parse::<i32>();
    if sequence.is_err() {
        return Err("身份证号码不正确:性别解析失败".to_string());
    }
    let gender = if sequence.unwrap() % 2 == 1 {
        Gender::Male
    } else {
        Gender::Female
    };
    Ok(Person {
        province: Some(division.0),
        city: Some(division.1),
        district: Some(division.2),
        birthday: Some(birthday),
        gender: Some(gender),
        id_no: Some(IdNo(id_no.to_string())),
    })
}

#[cfg(test)]
mod tests {
    use crate::utils::id_no::*;
    use chrono::Months;

    #[test]
    fn test_new_id_no() {
        // let ctx_rand_int = MockIdNo::get_sequence_context();
        // ctx_rand_int.expect()
        //     .with(predicate::eq(Gender::Male))
        //     .return_const(587);
        // assert_eq!(587, IdNo::get_sequence(Gender::Male));
        //
        // let ctx_rand_int = MockIdNo::get_sequence_context();
        // ctx_rand_int.expect()
        //     .with(predicate::eq(Gender::Female))
        //     .return_const(714);
        // assert_eq!(714, IdNo::get_sequence(Gender::Female));

        let mut excepted = IdNo("12010519930914587X".to_string());

        let actual = IdNo::new("天津市", "天津市", "河北区", "19930914", Gender::Male);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!(excepted, actual);

        excepted = IdNo("320412198507184645".to_string());
        let actual = IdNo::new("江苏省", "常州市", "武进区", "19850718", Gender::Female);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!(excepted, actual);
    }

    #[test]
    fn test_get_id_no() {
        let person = Person {
            province: None,
            city: None,
            district: None,
            gender: None,
            birthday: None,
        };

        let actual = get_id_no(person);
        assert!(actual.is_ok());

        let person = Person {
            province: Some("北京市".to_string()),
            city: Some("北京市".to_string()),
            district: Some("东城区".to_string()),
            gender: None,
            birthday: None,
        };
        let actual = get_id_no(person);
        assert!(actual.is_ok());

        let person = Person {
            province: Some("北京市".to_string()),
            city: Some("北京市".to_string()),
            district: Some("东城区".to_string()),
            gender: Some(Gender::Male),
            birthday: None,
        };
        let actual = get_id_no(person);
        assert!(actual.is_ok());

        let person = Person {
            province: Some("北京市".to_string()),
            city: Some("北京市".to_string()),
            district: Some("东城区".to_string()),
            gender: Some(Gender::Female),
            birthday: Some(Local::now().checked_sub_months(Months::new(300)).unwrap()),
        };
        let actual = get_id_no(person);
        assert!(actual.is_ok());
    }

    #[test]
    fn test_parse_id_no() {
        let id_no = "110101198507188289";
        let expected = Person {
            province: Some("北京市".to_string()),
            city: Some("北京市".to_string()),
            district: Some("东城区".to_string()),
            birthday: Some(Local.with_ymd_and_hms(1985i32, 07, 18, 0, 0, 0).unwrap()),
            gender: Some(Gender::Female),
        };
        let actual = parse_id_no(id_no);
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap());

        let id_no = "350521198507182292";
        let expected = Person {
            province: Some("福建省".to_string()),
            city: Some("泉州市".to_string()),
            district: Some("惠安县".to_string()),
            birthday: Some(Local.with_ymd_and_hms(1985i32, 07, 18, 0, 0, 0).unwrap()),
            gender: Some(Gender::Male),
        };
        let actual = parse_id_no(id_no);
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap());

        let id_no = "";
        let actual = parse_id_no(id_no);
        assert!(actual.is_err());

        let id_no = "99999";
        let actual = parse_id_no(id_no);
        assert!(actual.is_err());

        let id_no = "3505211985071822921";
        let actual = parse_id_no(id_no);
        assert!(actual.is_err());

        let id_no = "3505211985071822x2";
        let actual = parse_id_no(id_no);
        assert!(actual.is_err());

        let id_no = "37x5211985071822921";
        let actual = parse_id_no(id_no);
        assert!(actual.is_err());

        let id_no = "3505219985071822921";
        let actual = parse_id_no(id_no);
        assert!(actual.is_err());
    }
}
