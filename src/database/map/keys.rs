use conduwuit::{implement, Result};
use futures::{Stream, StreamExt};
use serde::Deserialize;

use crate::{keyval, keyval::Key, stream, stream::Cursor};

#[implement(super::Map)]
pub fn keys<'a, K>(&'a self) -> impl Stream<Item = Result<Key<'_, K>>> + Send
where
	K: Deserialize<'a> + Send,
{
	self.raw_keys().map(keyval::result_deserialize_key::<K>)
}

#[implement(super::Map)]
#[tracing::instrument(skip(self), fields(%self), level = "trace")]
pub fn raw_keys(&self) -> impl Stream<Item = Result<Key<'_>>> + Send {
	let opts = super::iter_options_default();
	stream::Keys::new(&self.db, &self.cf, opts).init(None)
}
