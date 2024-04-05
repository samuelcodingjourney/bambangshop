use dashmap::DashMap;
use lazy_static::lazy_statistic;
use crate::model::subscriber::Subscriber;

lazy_static! {
    static ref SUBSCRIBERS:: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
}