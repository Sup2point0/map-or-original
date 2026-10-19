pub trait MapOrOriginal: Sized
{
	fn map_or_original(self, f: impl FnOnce(&Self) -> Option<Self>) -> Self
	{
		f(&self).unwrap_or(self)
	}
}

impl<T> MapOrOriginal for T {}
