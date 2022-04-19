
pub fn max_profit(prices: Vec<i32>)-> i32{
	
	let mut max_profit: i32 = 0;
	let mut buy_price = 10_000;

	for i in 0..prices.len() as usize {
		if prices[i] < buy_price{
			buy_price = prices[i];
		}else if prices[i] - buy_price > max_profit{
			max_profit = prices[i] - buy_price;
		} 
		
	}
	max_profit
}