pub fn can_complete_circuit(gas_amounts: Vec<i32>, adjacent_travel_cost: Vec<i32>) -> i32 {
    let mut tank = 0;
    let mut pos = 0;
    let mut total = 0;

    for (station_idx, diff) in gas_amounts
        .iter()
        .zip(&adjacent_travel_cost)
        .map(|(gas, cost)| gas - cost)
        .enumerate()
    {
        total += diff;
        tank += diff;
        if tank < 0 {
            pos = station_idx + 1;
            tank = 0;
        }
    }

    if total >= 0 { pos as i32 } else { -1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let gas_amounts = (1..=5).collect::<Vec<i32>>();
        let adjacent_travel_cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas_amounts, adjacent_travel_cost), 3);
    }
}
