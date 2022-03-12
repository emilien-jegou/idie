use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::from_str;
use std::str::from_utf8;
use warp::hyper::{body::Bytes, Response};

pub fn extract_json_body<D>(resp: Response<Bytes>) -> Result<D>
where
  for<'de> D: Deserialize<'de>,
{
  // parse the body as serde_json::Value
  let str_body = from_utf8(resp.body())?;

  from_str(str_body)
    .with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", str_body))
}
