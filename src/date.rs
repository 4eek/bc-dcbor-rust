use std::rc::Rc;

use chrono::{DateTime, Utc, TimeZone, SecondsFormat, NaiveDate, NaiveDateTime};

use crate::{CBORCodable, CBOREncodable, CBORTaggedEncodable, Tag, CBOR, CBORDecodable, cbor_error::CBORError, CBORTaggedDecodable, CBORTaggedCodable, Simple, CBORTagged};

/// A CBOR-friendly representation of a date and time.
#[derive(Debug, Clone)]
pub struct Date(DateTime<Utc>);

impl Date {
    /// Creates a new `Date` from the given chrono `DateTime`.
    pub fn from_datetime(date_time: DateTime<Utc>) -> Self {
        Date(date_time)
    }

    /// Creates a new `Date` from seconds since (or before) the Unix epoch.
    pub fn from_timestamp(seconds_since_unix_epoch: i64) -> Self {
        Self::from_datetime(Utc.timestamp_opt(seconds_since_unix_epoch, 0).unwrap())
    }

    /// Creates a new `Date` from a string containing an ISO-8601 (RFC-3339) date (with or without time).
    pub fn from_str(value: &str) -> Option<Self> {
        // try parsing as DateTime
        if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
            return Some(Self::from_datetime(dt.with_timezone(&Utc)));
        }

        // try parsing as just a date (with assumed zero time)
        if let Ok(d) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            let dt = NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return Some(Self::from_datetime(DateTime::from_utc(dt, Utc)));
        }

        None
    }

    /// Creates a new `Date` containing the current date and time.
    pub fn now() -> Self {
        Self::from_datetime(Utc::now())
    }

    /// Returns the underlying chrono `DateTime` struct.
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }

    /// Returns the `Date` as the number of seconds since the Unix epoch.
    pub fn timestamp(&self) -> i64 {
        self.datetime().timestamp()
    }

    /// Returns a string with the ISO-8601 (RFC-3339) representation of the date.
    pub fn to_string(&self) -> String {
        self.datetime().to_rfc3339_opts(SecondsFormat::Secs, true)
    }
}

impl TryFrom<&str> for Date {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Self::from_str(value) {
            Some(date) => Ok(date),
            None => Err("Invalid date string".into())
        }
    }
}

impl CBOREncodable for Date {
    fn cbor(&self) -> CBOR {
        self.tagged_cbor()
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.tagged_cbor().cbor_data()
    }
}

impl CBORDecodable for Date {
    fn from_cbor(cbor: &CBOR) -> Result<Rc<Self>, CBORError> {
        Self::from_tagged_cbor(cbor)
    }
}

impl CBORCodable for Date { }

impl CBORTagged for Date {
    const CBOR_TAG: Tag = Tag::new(1);
}

impl CBORTaggedEncodable for Date {
    fn untagged_cbor(&self) -> CBOR {
        self.timestamp().cbor()
    }
}

impl CBORTaggedDecodable for Date {
    fn from_untagged_cbor(cbor: &CBOR) -> Result<Rc<Self>, CBORError> {
        match cbor {
            CBOR::Unsigned(n) => {
                let i = i64::try_from(*n).map_err(|_| CBORError::WrongType)?;
                return Ok(Rc::new(Date::from_timestamp(i)));
            },
            CBOR::Negative(n) => {
                return Ok(Rc::new(Date::from_timestamp(*n)));
            },
            CBOR::Simple(Simple::Float(n)) => {
                return Ok(Rc::new(Date::from_timestamp(*n as i64)));
            },
            _ => { return Err(CBORError::WrongType); }
        }
    }
}

impl CBORTaggedCodable for Date { }

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
