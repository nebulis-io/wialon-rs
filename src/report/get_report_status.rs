use std::{fmt, marker::PhantomData, str::FromStr};

use crate::request::WialonRequest;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

pub struct GetReportStatus {
    pub params: GetReportStatusParams,
}

#[derive(Serialize, Default)]
pub struct GetReportStatusParams {}

#[derive(Deserialize, Debug)]
pub enum ReportStatus {
    InQueue,
    Processing,
    Done,
    Canceled,
    Invalid,
}

impl FromStr for ReportStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::InQueue,
            "2" => Self::Processing,
            "4" => Self::Done,
            "8" => Self::Canceled,
            "16" => Self::Invalid,
            _ => Err("Invalid status")?,
        })
    }
}

fn string_or_struct<'de, T, D, E>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = E>,
    D: Deserializer<'de>,
    E: std::fmt::Debug,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T, Err> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Err>,
        Err: std::fmt::Debug,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or struct")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Deserialize, Debug)]
pub struct GetReportStatusResponse {
    #[serde(deserialize_with = "string_or_struct")]
    pub status: ReportStatus,
}

impl WialonRequest for GetReportStatus {
    type Params = GetReportStatusParams;

    type Response = GetReportStatusResponse;

    fn service_name(&self) -> &str {
        "report/get_report_status"
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
