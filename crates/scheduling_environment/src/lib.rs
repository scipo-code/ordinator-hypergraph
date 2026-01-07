use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

pub mod technician;
pub mod work_order;

#[derive(Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct Period(NaiveDate);

impl Period
{
    pub fn from_start_date(start_date: NaiveDate) -> Self
    {
        Self(start_date)
    }

    pub fn start_date(&self) -> NaiveDate
    {
        self.0
    }
}
