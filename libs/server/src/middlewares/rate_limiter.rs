use ratelimit;
use std::sync::{Arc, Mutex};
use warp::{reject::Reject, Filter, Rejection};

#[derive(Debug)]
struct RateLimited;

impl Reject for RateLimited {}

pub fn rate_limiter(mut limiter: ratelimit::Limiter) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
  let handle = Arc::new(Mutex::new(limiter.make_handle()));

  std::thread::spawn(move || limiter.run());

  let with_handle = warp::any().map(move || handle.clone());

  warp::any()
    .and(with_handle)
    .and_then(|handle: Arc<Mutex<ratelimit::Handle>>| async move {
      let mut guard = handle.lock().unwrap();
      if (*guard).try_wait().is_err() {
        Err(warp::reject::custom(RateLimited))
      } else {
        Ok::<(), Rejection>(())
      }
    })
}
