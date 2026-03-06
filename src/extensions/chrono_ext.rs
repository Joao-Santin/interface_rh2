use chrono::{NaiveDateTime, Datelike, Weekday};

trait NaiveDateTimePtBr{
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
