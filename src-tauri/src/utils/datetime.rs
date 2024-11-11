use chrono::{DateTime, Local, TimeZone, Utc};
use serde::{Deserializer, Serialize, Serializer};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
pub fn serialize_dt<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    match dt {
        None => { serializer.serialize_none()}
        Some(dt) => {
            dt.format(FORMAT).to_string().serialize(serializer)
        }
    }
}

pub fn serialize_dt_local<S>(dt: &Option<DateTime<Local>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    match dt {
        None => {serializer.serialize_none()}
        Some(dt) => {
            dt.format(FORMAT).to_string().serialize(serializer)
        }
    }
}

pub fn deserializer_dt<'d,D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where D: Deserializer<'d>
{
    use serde::Deserialize;
    let str_time = String::deserialize(deserializer)?;
    if str_time.is_empty() { return Ok(None)}
    let dt = chrono::NaiveDateTime::parse_from_str(&str_time, FORMAT)
        .map_err(serde::de::Error::custom)?;
    Ok(Some(dt.and_utc()))
}
pub fn deserializer_dt_local<'s, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
where D: Deserializer<'s>
{
    use serde::Deserialize;
    let str_time = String::deserialize(deserializer)?;
    if str_time.is_empty() { return Ok(None)  }
    let dt =  chrono::NaiveDateTime::parse_from_str(&str_time, FORMAT)
        .map_err(serde::de::Error::custom)?;
    // 减去时区
    let offset = Local.offset_from_utc_datetime(&dt);
    let dt = dt.checked_sub_offset(offset)
        .unwrap()
        .and_utc()
        .with_timezone(&Local);
    Ok(Some(dt))
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, TimeZone, Utc};
    use serde::{Deserialize, Serialize};
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct TestSerializeDateUtc {
        #[serde(serialize_with = "serialize_dt", deserialize_with = "deserializer_dt")]
        pub date: Option<DateTime<Utc>>,
    }

    #[derive(Serialize, Deserialize)]
    struct TestSerializeDateLocal {
        #[serde(serialize_with = "serialize_dt_local", deserialize_with = "deserializer_dt_local")]
        pub date: Option<DateTime<Local>>,
    }
    #[test]
    fn test_serialize_dt() {
        let format = "%Y-%m-%d %H:%M:%S";
        let now = Utc::now();
        let test_dt = TestSerializeDateUtc {
            date: Some(now)
        };
        let expected = "{\n  \"date\": \"".to_string() + now.format(format).to_string().as_str() + "\"\n}";
        let result =serde_json::to_string_pretty(&test_dt);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_serialize_dt_local() {
        let now = Local::now();
        let test_dt = TestSerializeDateLocal {
            date: Some(now)
        };
        let format = "%Y-%m-%d %H:%M:%S";
        let expected = "{\n  \"date\": \"".to_string() + now.format(format).to_string().as_str() + "\"\n}";
        let result = serde_json::to_string_pretty(&test_dt);
        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_deserialize_dt() {
        let test_utc_dc = r#"
        {
            "date": "2024-10-19 12:23:44"
        }
        "#;
        let expected: DateTime<Utc> = Utc.with_ymd_and_hms(2024i32, 10, 19, 12, 23, 44)
            .unwrap();
        let actual = serde_json::from_str::<TestSerializeDateUtc>(test_utc_dc);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert!(actual.date.is_some());
        assert_eq!(expected, actual.date.unwrap())

    }

    #[test]
    fn test_deserialize_dt_local() {
        let test_utc_dc = r#"
        {
            "date": "2024-10-19 12:23:44"
        }
        "#;
        let expected: DateTime<Local> = Local.with_ymd_and_hms(2024i32, 10, 19, 12, 23, 44)
            .unwrap();
        let actual = serde_json::from_str::<TestSerializeDateLocal>(test_utc_dc);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert!(actual.date.is_some());
        assert_eq!(expected, actual.date.unwrap())

    }
}