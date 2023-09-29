use ej_individual::bomberman::Bomberman;

// Next 3 integration test come from https://taller-1-fiuba-rust.github.io/proyecto/23C2/ejercicio_individual.html
#[test]
fn integration_test_example_1() {
    let input = "B2 R R _ F1 _ _\n_ W R W _ W _\nB5 _ _ _ B2 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n";
    let result = "_ R R _ _ _ _\n_ W R W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n";
    let mut game = Bomberman::new(input).unwrap();
    let board = game.play(Point::new(0, 0)).unwrap();
    assert_eq!(result, board);
}

#[test]
fn integration_test_example_2() {
    let input = "_ _ B2 _ B1 _ _\n_ W _ W _ W _\n_ _ B2 R F1 _ _\n_ W _ W R W _\n_ _ B4 _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1\n";
    let result = "_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ R F1 _ _\n_ W _ W R W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1\n";
    let mut game = Bomberman::new(input).unwrap();
    let board = game.play(Point::new(2, 4)).unwrap();
    assert_eq!(result, board);
}

#[test]
fn integration_test_example_3() {
    let input = "_ _ _ _ _ _ _\n_ W _ W _ W _\nS4 R R R F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n";
    let result = "_ _ _ _ _ _ _\n_ W _ W _ W _\n_ R R R _ _ _\n_ W _ W _ W _\n_ _ _ _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n";
    let mut game = Bomberman::new(input).unwrap();
    let board = game.play(Point::new(2, 4)).unwrap();
    assert_eq!(result, board);
}