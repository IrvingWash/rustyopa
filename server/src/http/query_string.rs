use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, QueryValue<'buff>>,
}

impl<'buff> QueryString<'buff> {
    pub fn get(&self, key: &str) -> Option<&QueryValue> {
        return self.data.get(key);
    }
}

impl<'buff> From<&'buff str> for QueryString<'buff> {
    fn from(s: &'buff str) -> Self {
        let mut data = HashMap::new();

        for substring in s.split('&') {
            let mut key = substring;
            let mut val = "";

            if let Some(i) = substring.find('=') {
                key = &substring[..i];
                val = &substring[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut QueryValue| match existing {
                    QueryValue::Single(prev_val) => {
                        *existing = QueryValue::Multiple(vec![prev_val, val]);
                    }
                    QueryValue::Multiple(vec) => vec.push(val),
                })
                .or_insert(QueryValue::Single(val));
        }

        QueryString { data }
    }
}

#[derive(Debug)]
pub enum QueryValue<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}
