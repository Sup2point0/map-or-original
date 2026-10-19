pub trait MapOrOriginal: Sized
{
	fn map_or_original(self, f: impl FnOnce(&Self) -> Option<Self>) -> Self
	{
		f(&self).unwrap_or(self)
	}

	fn map_or_orig(self, f: impl FnOnce(&Self) -> Option<Self>) -> Self
	{
		self.map_or_original(f)
	}
}

impl<T> MapOrOriginal for T {}
