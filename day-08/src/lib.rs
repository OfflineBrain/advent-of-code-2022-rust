pub fn part1(input: &str) -> String {
    let trees_matrix = input
        .lines()
        .map(|line| {
            line.split("")
                .skip(1)
                .take(line.len())
                .map(|symbol| symbol.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = trees_matrix.len();
    let width = trees_matrix.first().unwrap().len();

    let mut result = ((height + width) * 2) - 4;

    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            let tree = trees_matrix[row][col];

            if !(trees_matrix[row][..col].iter().any(|t| *t >= tree)
                && trees_matrix[row][(col + 1)..].iter().any(|t| *t >= tree)
                && trees_matrix[..row].iter().any(|c| c[col] >= tree)
                && trees_matrix[(row + 1)..].iter().any(|c| c[col] >= tree))
            {
                result += 1;
            }
        }
    }

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let trees_matrix = input
        .lines()
        .map(|line| {
            line.split("")
                .skip(1)
                .take(line.len())
                .map(|symbol| symbol.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = trees_matrix.len();
    let width = trees_matrix.first().unwrap().len();

    let mut result = Vec::<usize>::new();

    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            let tree = trees_matrix[row][col];

            let mut l_view = trees_matrix[row][..col]
                .iter()
                .rev()
                .take_while(|&t| *t < tree)
                .count();
            if l_view < col {
                l_view += 1;
            }

            let mut r_view = trees_matrix[row][(col + 1)..]
                .iter()
                .take_while(|&t| *t < tree)
                .count();
            if r_view < width - (col + 1) {
                r_view += 1;
            }

            let mut u_view = trees_matrix[..row]
                .iter()
                .rev()
                .take_while(|&t| t[col] < tree)
                .count();
            if u_view < row {
                u_view += 1;
            }

            let mut d_view = trees_matrix[(row + 1)..]
                .iter()
                .take_while(|&t| t[col] < tree)
                .count();
            if d_view < height - (row + 1) {
                d_view += 1;
            }

            result.push(l_view * r_view * u_view * d_view)
        }
    }

    result.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "8");
    }
}
