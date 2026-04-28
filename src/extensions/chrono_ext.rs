use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday, Duration, Timelike};

pub trait NaiveDateTimePtBr{
    fn monthname_abb_ptbr(&self) -> &'static str;
    fn monthname_ptbr(&self) -> &'static str;
    fn weekdayname_ptbr(&self) -> &'static str;
    fn round_with_tolerance(&self) -> Self;
}
impl NaiveDateTimePtBr for NaiveDateTime{
    fn monthname_abb_ptbr(&self) -> &'static str{
        const MESES: [&str; 12] = [
            "Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"
        ];
        MESES[(self.month()-1) as usize]
    }
    fn monthname_ptbr(&self) -> &'static str{
        const MESES: [&str; 12] = [
            "Janeiro", "Fevereiro", "Marco", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"
        ];
        MESES[(self.month()-1) as usize]
    }
    fn weekdayname_ptbr(&self) -> &'static str{
        match self.weekday() {
            Weekday::Mon => "Segunda-feira",
            Weekday::Tue => "Terça-feira",
            Weekday::Wed => "Quarta-feira",
            Weekday::Thu => "Quinta-feira",
            Weekday::Fri => "Sexta-feira",
            Weekday::Sat => "Sábado",
            Weekday::Sun => "Domingo",
        }
    }
    fn round_with_tolerance(&self) -> Self {
        let minute = self.minute();

        if minute <= 12 {
            self.with_minute(0).unwrap().with_second(0).unwrap()
        } else if minute >= 48 {
            self
                .with_minute(0).unwrap()
                .with_second(0).unwrap()
                + Duration::hours(1)
        } else {
            self.with_second(0).unwrap()
        }
    }
}
pub trait NaiveDateExt{
    fn get_month_day_weekday(&self) -> Vec<(chrono::Weekday, u32)>;
    fn get_workday_duration(&self) -> Duration;

}
impl NaiveDateExt for NaiveDate{
    fn get_month_day_weekday(&self) -> Vec<(chrono::Weekday, u32)>{
        let month = self.month();
        let mut date = NaiveDate::from_ymd_opt(self.year(), month, 1).expect("data invalida");
        let mut days = Vec::new();
        while date.month() == month{
            days.push((date.weekday(), date.day()));
            date += Duration::days(1);
        }
        days
    }
    fn get_workday_duration(&self) -> Duration{
        match self.weekday(){
            Weekday::Mon |
            Weekday::Tue |
            Weekday::Wed |
            Weekday::Thu => Duration::hours(9),
            Weekday::Fri => Duration::hours(8),
            Weekday::Sat |
            Weekday::Sun => Duration::zero(),
        }
    }

    }
impl NaiveDateTimePtBr for NaiveDate{
    fn monthname_abb_ptbr(&self) -> &'static str{
        const MESES: [&str; 12] = [
            "Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"
        ];
        MESES[(self.month()-1) as usize]
    }
    fn monthname_ptbr(&self) -> &'static str{
        const MESES: [&str; 12] = [
            "Janeiro", "Fevereiro", "Marco", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"
        ];
        MESES[(self.month()-1) as usize]
    }
    fn weekdayname_ptbr(&self) -> &'static str{
        match self.weekday() {
            Weekday::Mon => "Segunda-feira",
            Weekday::Tue => "Terça-feira",
            Weekday::Wed => "Quarta-feira",
            Weekday::Thu => "Quinta-feira",
            Weekday::Fri => "Sexta-feira",
            Weekday::Sat => "Sábado",
            Weekday::Sun => "Domingo",
        }
    }
    fn round_with_tolerance(&self) -> Self {
        *self
    }
}
pub fn is_valid_naivedatetime(date_string:&String) -> bool{
    NaiveDateTime::parse_from_str(date_string.trim(), "%d-%m-%Y %H:%M").is_ok()
}
