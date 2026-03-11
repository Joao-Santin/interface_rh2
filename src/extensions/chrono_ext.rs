use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday, Duration};

pub trait NaiveDateTimePtBr{
    fn monthname_ptbr(&self) -> &'static str;
    fn weekdayname_ptbr(&self) -> &'static str;
}
impl NaiveDateTimePtBr for NaiveDateTime{
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
}
pub trait NaiveDateExt{
        fn get_month_day_weekday(&self) -> Vec<(chrono::Weekday, u32)>;
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

    }
impl NaiveDateTimePtBr for NaiveDate{
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
}
