use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::Australia::{Adelaide, Brisbane, Darwin, Hobart, Melbourne, Perth, Sydney};
use chrono_tz::Europe::{Berlin, Budapest, London, Moscow, Vienna};
use chrono_tz::Pacific::Auckland;
use chrono_tz::Tz;
use chrono_tz::US::{Central, Eastern, Pacific};

#[derive(Debug)]
pub enum CityList {
    Moscow,
    Budapest,
    Vienna,
    Berlin,
    London,
    Sydney,
    Brisbane,
    Melbourne,
    Hobart,
    Adelaide,
    Darwin,
    Perth,
    LosAngeles,
    Chicago,
    NewYork,
    Auckland,
}

#[derive(Debug)]
pub struct City {
    city: CityList,
    year: i32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl City {
    pub fn new(city: CityList) -> Self {
        match city {
            CityList::Moscow => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Moscow);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Budapest => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Budapest);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Vienna => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Vienna);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Berlin => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Berlin);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::London => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&London);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Sydney => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Sydney);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Brisbane => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Brisbane);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Melbourne => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Melbourne);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Hobart => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Hobart);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Adelaide => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Adelaide);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Darwin => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Darwin);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Perth => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Perth);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::LosAngeles => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Chicago => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Central);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::NewYork => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Eastern);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
            CityList::Auckland => {
                let dt_city: DateTime<Tz> = Utc::now().with_timezone(&Auckland);
                Self {
                    city: city,
                    year: dt_city.year(),
                    month: dt_city.month(),
                    day: dt_city.day(),
                    hours: dt_city.hour(),
                    minutes: dt_city.minute(),
                    seconds: dt_city.second(),
                }
            }
        }
    }

    pub fn city(&self) -> &CityList {
        &self.city
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn hours(&self) -> u32 {
        self.hours
    }

    pub fn minutes(&self) -> u32 {
        self.minutes
    }

    pub fn seconds(&self) -> u32 {
        self.seconds
    }

    pub fn current_time(&self) -> String {
        format!(
            "{:?}: {}/{}/{} {}:{}:{}",
            self.city(),
            self.day(),
            self.month(),
            self.year(),
            self.hours(),
            self.minutes(),
            self.seconds()
        )
    }

    pub fn timestamp(&self) -> i64 {
        let dt: DateTime<Utc> = Utc
            .with_ymd_and_hms(
                self.year(),
                self.month(),
                self.day(),
                self.hours(),
                self.minutes(),
                self.seconds(),
            )
            .unwrap();
        dt.timestamp()
    }

    pub fn time_difference(&self, city: &City) -> f32 {
        let diff = city.timestamp() - self.timestamp();
        (diff as f32) / (3600.0 as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_difference_01() {
        let sydney = City::new(CityList::Sydney);
        let auckland = City::new(CityList::Auckland);
        assert_eq!(sydney.time_difference(&auckland), 2.0);
    }
}
