use map_or_original::MapOrOriginal;


mod primitives
{
	use super::*;

	#[test] fn ints()
	{
		assert_eq!( 1.map_or_orig(|&x| (x > 0).then_some(x * 10)), 10 );
		assert_eq!( 0.map_or_orig(|&x| (x > 0).then_some(x * 10)), 0 );
	}
}

mod enums
{
	use super::*;

	#[test] fn option()
	{
		let t = Some(0);
		assert_eq!( t.map_or_orig(|_| None), t );
		assert_eq!( t.map_or_orig(|_| Some(Some(1))), Some(1) );
	}
}
