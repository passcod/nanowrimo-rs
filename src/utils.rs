use crate::{NanoKind, ObjectRef, RelationLink};

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use chrono::Duration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// TODO: Once serde supports better custom Option with annotations, use those instead
//       of the opt_* funcs

pub(crate) fn de_str_num<'de, T, D>(des: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    String::deserialize(des)?
        .parse::<T>()
        .map_err(serde::de::Error::custom)
}

pub(crate) fn de_opt_str_num<'de, T, D>(des: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    Ok(de_str_num(des).ok())
}

pub(crate) fn se_str_id<S>(num: &u64, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    num.to_string().serialize(ser)
}

pub(crate) fn de_duration_mins<'de, D>(des: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let val = i64::deserialize(des)?;

    Ok(Duration::minutes(val))
}

pub(crate) fn se_duration_mins<S>(duration: &Duration, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration.num_minutes().serialize(ser)
}

// NanoKind related stuff

pub(crate) fn de_nanokind<'de, D>(des: D) -> Result<NanoKind, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(des).map_err(serde::de::Error::custom)?;

    NanoKind::from_name(&str).map_err(serde::de::Error::custom)
}

// pub(crate) fn de_opt_nanokind<'de, D>(des: D) -> Result<Option<NanoKind>, D::Error>
//     where
//         D: Deserializer<'de>
// {
//     Ok(de_nanokind(des).ok())
// }

pub(crate) fn se_nanokind<S>(kind: &NanoKind, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    kind.api_name().serialize(ser)
}

// pub(crate) fn se_nanokind_unique<S>(kind: &NanoKind, ser: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer
// {
//     kind
//         .api_unique_name()
//         .serialize(ser)
// }
//
// pub(crate) fn se_opt_nanokind_unique<S>(kind: &Option<NanoKind>, ser: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer
// {
//     match kind {
//         Some(kind) => se_nanokind_unique(kind, ser),
//         None => <Option<()>>::None.serialize(ser)
//     }
// }

pub(crate) fn de_rel_includes<'de, D>(des: D) -> Result<HashMap<NanoKind, Vec<ObjectRef>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct DataWrap {
        data: Option<Vec<ObjectRef>>,
    }

    HashMap::<String, DataWrap>::deserialize(des)
        .map(|table| {
            table
                .into_iter()
                .filter(|(_, val)| val.data.is_some())
                .map(|(key, val)| {
                    (
                        NanoKind::from_name(&key).expect("unwrap de_rel_includes key"),
                        val.data.expect("unwrap de_rel_includes val"),
                    )
                })
                .collect()
        })
        .map_err(serde::de::Error::custom)
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum SeRelIncludeInner {
    Single { data: ObjectRef },
    Multi(Vec<ObjectRef>),
}

pub(crate) fn se_rel_includes<S>(
    val: &HashMap<NanoKind, Vec<ObjectRef>>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.iter()
        .map(|(key, val)| {
            if val.len() == 1 {
                (
                    key.api_unique_name().to_string(),
                    SeRelIncludeInner::Single {
                        data: val.first().unwrap().clone(),
                    },
                )
            } else {
                (
                    key.api_name().to_string(),
                    SeRelIncludeInner::Multi(val.clone()),
                )
            }
        })
        .collect::<HashMap<String, SeRelIncludeInner>>()
        .serialize(ser)
}

pub(crate) fn de_relation<'de, D>(des: D) -> Result<HashMap<NanoKind, RelationLink>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct LinkWrap {
        links: RelationLink,
    }

    HashMap::<String, LinkWrap>::deserialize(des)
        .map(|table| {
            table
                .into_iter()
                .map(|(key, val)| {
                    (
                        NanoKind::from_name(&key).expect("unwrap de_relation name"),
                        val.links,
                    )
                })
                .collect()
        })
        .map_err(serde::de::Error::custom)
}

pub(crate) fn se_relation<S>(
    val: &HashMap<NanoKind, RelationLink>,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.iter()
        .map(|(key, val)| (key.api_name().to_string(), val.clone()))
        .collect::<HashMap<String, RelationLink>>()
        .serialize(ser)
}

pub(crate) fn de_heighten_img<'de, D>(des: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct ImageWrap {
        src: String,
    }

    ImageWrap::deserialize(des).map(|val| val.src)
}
