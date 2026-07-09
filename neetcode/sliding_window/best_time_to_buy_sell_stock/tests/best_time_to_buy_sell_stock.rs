use best_time_to_buy_sell_stock::max_profit;

#[test]
fn examples() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0); // only decreasing -> no profit
}

#[test]
fn edge_cases() {
    assert_eq!(max_profit(vec![1]), 0);            // single day, can't sell
    assert_eq!(max_profit(vec![2, 4, 1]), 2);      // best sell before the dip
    assert_eq!(max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
}
