use chrono::{Duration, NaiveDate, NaiveDateTime};

use crate::domain::tally::tally::Tally;

pub struct DayResult{
    pub cpf: String,
    pub date: NaiveDate,
    pub worked: Duration,
    pub expected: Duration,
    pub balance: Duration,
}

impl DayResult{
    fn new_by_tally_info_add(registro: (NaiveDate, Vec<(NaiveDateTime, Tally)>)){
    }
}
