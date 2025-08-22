//! MongoDB query builders

use mongodb::bson::{doc, Document};
use chrono::{DateTime, Utc};

/// MongoDB query builder
pub struct MongoQueryBuilder {
    filter: Document,
}

impl MongoQueryBuilder {
    /// Create new query builder
    pub fn new() -> Self {
        Self {
            filter: Document::new(),
        }
    }

    /// Add field filter
    pub fn field(mut self, name: &str, value: impl Into<mongodb::bson::Bson>) -> Self {
        self.filter.insert(name, value);
        self
    }

    /// Add date range filter
    pub fn date_range(mut self, field: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        use std::time::SystemTime;
        
        let start_system = SystemTime::from(start);
        let end_system = SystemTime::from(end);
        
        self.filter.insert(field, doc! {
            "$gte": mongodb::bson::DateTime::from_system_time(start_system),
            "$lte": mongodb::bson::DateTime::from_system_time(end_system)
        });
        self
    }

    /// Add text search
    pub fn text_search(mut self, field: &str, query: &str) -> Self {
        self.filter.insert(field, doc! {
            "$regex": query,
            "$options": "i"
        });
        self
    }

    /// Add sort by field
    pub fn sort_by(self, field: &str, descending: bool) -> MongoSortBuilder {
        MongoSortBuilder::new(self.filter, field, descending)
    }

    /// Build final filter document
    pub fn build(self) -> Document {
        self.filter
    }
}

/// MongoDB sort builder
pub struct MongoSortBuilder {
    filter: Document,
    sort: Document,
}

impl MongoSortBuilder {
    fn new(filter: Document, field: &str, descending: bool) -> Self {
        let mut sort = Document::new();
        sort.insert(field, if descending { -1 } else { 1 });
        
        Self { filter, sort }
    }

    /// Add additional sort field
    pub fn then_by(mut self, field: &str, descending: bool) -> Self {
        self.sort.insert(field, if descending { -1 } else { 1 });
        self
    }

    /// Build final query with sort
    pub fn build(self) -> (Document, Document) {
        (self.filter, self.sort)
    }
}

impl Default for MongoQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
