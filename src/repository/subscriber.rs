use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Database Singleton
lazy_static {
	static red SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct RepoSubscriber;

impl RepoSubscriber {
	
}