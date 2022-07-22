use chrono::{DateTime, Utc};

struct Base {
    pub title: String,
    pub amount: f32,
    pub currency: Currency,
    pub date: DateTime<Utc>,
}

impl Base {
    pub fn new(
        input_title: &str,
        input_amount: &f32,
        input_currency: &Currency,
        input_date: &DateTime<Utc>,
    ) -> Base {
        Base {
            title: input_title.to_string(),
            amount: input_amount,
            currency: input_currency,
            date: input_date,
        }
    }
}
