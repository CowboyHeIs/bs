use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Database Singleton
lazy_static {
	static red SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct RepoSubscriber;

impl RepoSubscriber {
	pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
		let subscriber_value = subscriber.clone();
		if SUBSCRIBERS.get(product_type).is_none() {
			SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
		};

		SUBSCRIBERS.get(product_type).unwrap().insert(subscriber_value.url.clone(), subscriber_value);
		return subscriber;
	}

	pub list_all(product_type: &str) -> Vec<Subscriber> {
		if SUBSCRIBERS.get(product_type).is_none() {
			SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
		};

		return SUBSCRIBERS.get(product_type).unwrap().iter().map(|f| f.value().clone()).collect();
	}

	pub fn delete(product_type: &str, url: &str) -> Option<Subscriber> {
		if SUBSCRIBERS.get(product_type).is_none() {
			SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
		}
		let result = SUBSCRIBERS.get(product_type).unwrap().remove(url);
		if !resust.is_none() {
			return Some(result.unwrap().1);
		}
		return None;
	}
}