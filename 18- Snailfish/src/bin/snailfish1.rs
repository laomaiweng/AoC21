use snailfish::{add, reduce, magnitude, parse_stdin};

fn main() {
    let (mut arena, numbers) = parse_stdin();
    let mut index = numbers[0];
    for n in &numbers[1..] {
        index = add(index, *n, &mut arena);
        reduce(index, &mut arena);
    }
    let result = arena.get(index).unwrap().borrow();
    println!("Sum: {}", result.to_string(&arena));
    println!("Magnitude: {}", magnitude(index, &arena));
}

#[cfg(test)]
mod tests {
    use super::*;
    use snailfish::{explode, parse_string};

    fn do_explode(number: &str) -> String {
        let (mut arena, numbers) = parse_string(number);
        explode(numbers[0], &mut arena);
        let number = arena.get(numbers[0]).unwrap().borrow();
        number.to_string(&arena)
    }

    #[test]
    fn explode_1() {
        assert_eq!(do_explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn explode_2() {
        assert_eq!(do_explode("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn explode_3() {
        assert_eq!(do_explode("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn explode_4() {
        assert_eq!(do_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    }

    #[test]
    fn explode_5() {
        assert_eq!(do_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    fn do_add(numbers: &str) -> String {
        let (mut arena, numbers) = parse_string(numbers);
        let mut index = numbers[0];
        for n in &numbers[1..] {
            index = add(index, *n, &mut arena);
            reduce(index, &mut arena);
        }
        let result = arena.get(index).unwrap().borrow();
        result.to_string(&arena)
    }

    #[test]
    fn add_1() {
        assert_eq!(do_add(r#"
                [[[[4,3],4],4],[7,[[8,4],9]]]
                [1,1]
            "#), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn add_2() {
        assert_eq!(do_add(r#"
                [1,1]
                [2,2]
                [3,3]
                [4,4]
            "#), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn add_3() {
        assert_eq!(do_add(r#"
                [1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
            "#), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn add_4() {
        assert_eq!(do_add(r#"
                [1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
                [6,6]
            "#), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn add_5() {
        assert_eq!(do_add(r#"
                [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]
            "#), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    fn do_magnitude(number: &str) -> u32 {
        let (mut arena, numbers) = parse_string(number);
        magnitude(numbers[0], &mut arena)
    }

    #[test]
    fn magnitude_1() {
        assert_eq!(do_magnitude("[9,1]"), 29);
    }

    #[test]
    fn magnitude_2() {
        assert_eq!(do_magnitude("[1,9]"), 21);
    }

    #[test]
    fn magnitude_3() {
        assert_eq!(do_magnitude("[[9,1],[1,9]]"), 129);
    }

    #[test]
    fn magnitude_4() {
        assert_eq!(do_magnitude("[[1,2],[[3,4],5]]"), 143);
    }

    #[test]
    fn magnitude_5() {
        assert_eq!(do_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
    }

    #[test]
    fn magnitude_6() {
        assert_eq!(do_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
    }

    #[test]
    fn magnitude_7() {
        assert_eq!(do_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
    }

    #[test]
    fn magnitude_8() {
        assert_eq!(do_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
    }

    #[test]
    fn magnitude_9() {
        assert_eq!(do_magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"), 3488);
    }

    #[test]
    fn homework() {
        assert_eq!(do_add(r#"
                [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
                [[[5,[2,8]],4],[5,[[9,9],0]]]
                [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
                [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
                [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
                [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
                [[[[5,4],[7,7]],8],[[8,3],8]]
                [[9,3],[[9,9],[6,[4,9]]]]
                [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
                [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
            "#), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(do_magnitude("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"), 4140);
    }
}
