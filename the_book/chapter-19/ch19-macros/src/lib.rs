#[macro_export]
macro_rules! coba_vec {
	( $( $x:expr ),* ) => {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x);
			)*
			temp_vec
		}
	};
}

