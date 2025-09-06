#[cfg(test)]
mod test {
    use axum_dice::dice::Dice;

    #[test]
    fn test_dice_creation() {
        let dice: Dice = Dice::new(6);
        assert_eq!(dice.sides, 6);
    }

    #[test]
    fn test_dice_roll_within_range() {
        let dice: Dice = Dice::new(20);
        for _ in 0..100 {
            let result = dice.roll();
            assert!(result >= 1 && result <= 20);
            println!("Rolled: {}", result);
        }
    }

    #[test]
    fn test_dice_roll_multiple_sides() {
        let sides = [4, 6, 8, 10, 12, 20];
        for &side in &sides {
            let dice: Dice = Dice::new(side);
            for _ in 0..50 {
                let result = dice.roll();
                assert!(result >= 1 && result <= side);
                println!("Rolled a d{}: {}", side, result);
            }
        }
    }
}
