fn main()
{

	// 参照
	let a=5;
	println!("a={}",a);

	let b = &a;		// bはaを参照する
	println!("b={}",*b);	// aの値が出る

	// 値を更新するには参照をミュータブルにする必要がある
	// let mut a =5;
	// let b = &mut a;
/*
	*b=10;
	println!("b={}",*b);	// aの値が出る
	println!("a={}",a); // 更新されたaの値
*/


	// 範囲演算子
	/*
	let mut array = [10, 20, 30, 40, 50, 60, 70, 80];
	let slice = & mut array[2..6];	// 配列の3～6番目の要素を参照する
	println!("{:?}",slice);

	slice[0] = 300;
	println!("{}",array[2]);
*/
}
