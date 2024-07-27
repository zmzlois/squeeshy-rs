mod bundler;

use std::sync::Arc;

use squeeshy_resolver::Resolver;

pub(crate) type SharedResolver = Arc<Resolver>;

