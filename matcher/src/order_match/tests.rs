use super::*;

#[test]
fn low_buy_does_not_match_high_sell() {
    let buy_order = Order::new(5, 10, "account1", Direction::Buy);
    let sell_order = Order::new(10, 10, "account2", Direction::Sell);
    assert!(!OrderMatch::did_match(&buy_order, &sell_order));
    assert!(!OrderMatch::did_match(&sell_order, &buy_order));
}

#[test]
fn higher_buy_does_match_with_sell() {
    let buy_order = Order::new(10, 10, "account1", Direction::Buy);
    let sell_order = Order::new(5, 5, "account2", Direction::Sell);
    assert!(OrderMatch::did_match(&buy_order, &sell_order));
    assert!(OrderMatch::did_match(&sell_order, &buy_order));
}

#[test]
fn quantity_matched_is_always_lowest() {
    let big_buy_order = Order::new(10, 100, "account1", Direction::Buy);
    let small_sell_order = Order::new(10, 5, "account2", Direction::Sell);
    let matched = OrderMatch::match_orders(&big_buy_order, &small_sell_order).unwrap();
    assert_eq!(5, matched.quantity_matched);
}

#[test]
fn price_matched_is_always_top_order() {
    let buy_order = Order::new(15, 5, "account1", Direction::Buy);
    let sell_order = Order::new(10, 5, "account2", Direction::Sell);
    assert_eq!(
        10,
        OrderMatch::match_orders(&buy_order, &sell_order)
            .unwrap()
            .price_matched
    );
    assert_eq!(
        15,
        OrderMatch::match_orders(&sell_order, &buy_order)
            .unwrap()
            .price_matched
    );
}

#[test]
fn new_order_with_no_quantity_does_not_match() {
    let buy_order = Order::new(10, 0, "account1", Direction::Buy);
    let sell_order = Order::new(10, 5, "account2", Direction::Sell);
    assert!(OrderMatch::match_orders(&buy_order, &sell_order).is_none());
}

#[test]
#[should_panic(expected = "Top Order does not have valid quantity")]
fn top_order_with_0_quantity_panics() {
    let new_order = Order::new(10, 10, "account1", Direction::Buy);
    let invalid_top_order = Order::new(10, 0, "account2", Direction::Sell);
    let _match = OrderMatch::match_orders(&new_order, &invalid_top_order);
}

#[test]
#[should_panic(expected = "tried to match with same account_id")]
fn same_account_cannot_match() {
    let buy_order = Order::new(10, 100, "account1", Direction::Buy);
    let sell_order = Order::new(10, 100, "account1", Direction::Sell);
    let _match = OrderMatch::match_orders(&buy_order, &sell_order);
}

#[test]
#[should_panic(expected = "tried to match with same direction")]
fn matching_orders_must_be_different_directions() {
    let buy_order_1 = Order::new(10, 100, "account1", Direction::Buy);
    let buy_order_2 = Order::new(10, 100, "account2", Direction::Buy);
    let _match = OrderMatch::match_orders(&buy_order_1, &buy_order_2);
}
