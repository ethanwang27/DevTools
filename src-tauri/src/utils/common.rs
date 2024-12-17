pub mod random {
    use chrono::{DateTime, Local, Months};
    use rand::thread_rng;
    use rand::Rng;

    pub fn rand_int(min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..max)
    }

    ///生成随机奇数
    pub fn rand_odd_int(min: i32, max: i32) -> i32 {
        let rand = rand_int(min, max);
        if rand == min && min % 2 == 0 {
            return rand + 1;
        }
        if rand % 2 == 0 {
            rand - 1
        } else {
            rand
        }
    }

    ///生成随机偶数
    pub fn rand_even_int(min: i32, max: i32) -> i32 {
        let rand = rand_int(min, max);
        if rand == min && min % 2 == 1 {
            return rand + 1;
        }
        if rand % 2 == 1 {
            rand - 1
        } else {
            rand
        }
    }

    /// 随机生成日期时间
    pub fn random_datetime() -> DateTime<Local> {
        let datetime = Local::now();
        let months = rand_int(36, 720) as u32;
        datetime
            .checked_sub_months(Months::new(months))
            .expect("Datetime overflow")
    }

    /// 随机获取素组的元素
    pub fn random_item<T>(info: &Vec<T>) -> Option<T>
    where
        T: Clone + 'static,
    {
        if info.len() == 0 {
            return None;
        }
        let index = rand_int(0, info.len() as i32);
        info.get(index as usize).cloned()
    }
}

#[cfg(test)]
mod test {

    use crate::utils::common::random::*;
    use chrono::{Local, Months};

    #[test]
    fn test_rand() {
        let actual = rand_int(0, 20);
        assert!(actual >= 0);
        assert!(actual < 20);

        let actual = rand_int(-10, 0);
        assert!(actual >= -10);
        assert!(actual < 0);
    }

    #[test]
    fn test_rand_odd_int() {
        let actual = rand_odd_int(0, 20);
        assert!(actual >= 0);
        assert!(actual < 20);
        assert_eq!(actual % 2, 1);
    }

    #[test]
    fn test_rand_even_int() {
        let actual = rand_even_int(0, 20);
        assert!(actual >= 0);
        assert!(actual < 20);
        assert_eq!(actual % 2, 0);
    }

    #[test]
    fn test_rand_datetime() {
        let actual = random_datetime();
        let now = Local::now();
        let max_datetime = now.checked_sub_months(Months::new(36)).unwrap();
        let min_datetime = now.checked_sub_months(Months::new(720)).unwrap();
        assert!(actual <= max_datetime);
        assert!(actual > min_datetime);
    }

    #[test]
    fn test_random_item() {
        let vec = vec![1, 2, 3, 4, 5];
        let actual = random_item(&vec);
        assert!(actual.is_some(), "随机获取元素失败");

        use super::super::administrative_division::ProvinceDivision;
        let vec = vec![
            ProvinceDivision::new("a", "1"),
            ProvinceDivision::new("b", "2"),
            ProvinceDivision::new("c", "3"),
            ProvinceDivision::new("d", "4"),
            ProvinceDivision::new("e", "5"),
            ProvinceDivision::new("f", "6"),
        ];
        let actual = random_item(&vec);
        assert!(actual.is_some(), "随机获取元素失败");

        let vec: Vec<i32> = vec![];
        let actual = random_item(&vec);
        assert!(actual.is_none(), "随机获取元素失败");
    }
}
