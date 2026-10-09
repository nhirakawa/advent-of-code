use std::iter::successors;
use anyhow::{anyhow, Context};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (row, column) = parse_row_column(input)?;
    Ok(code_at(row, column))
}

fn code_at(row: usize, column: usize) -> usize {
    let index = get_index(row, column);
    codes().nth(index - 1).unwrap()
}

fn codes() -> impl Iterator<Item = usize> {
    successors(Some(20151125), |n| Some((*n * 252533) % 33554393))
}

fn get_index(row: usize, column: usize) -> usize {
    let first_row = triangular_number(column);
    let offset = offset(row, column);
    first_row + offset
}

fn triangular_number(column: usize) -> usize {
    (column * (column + 1)) / 2
}

fn offset(row: usize, column: usize) -> usize {
    let n = row - 1;
    n * column + (n * n - n) / 2
}

fn parse_row_column(s: &str) -> anyhow::Result<(usize, usize)> {
    let s = s.strip_prefix("To continue, please consult the code grid in the manual.  Enter the code at ").ok_or(anyhow!("Could not strip prefix"))?;
    let s = s.strip_suffix(".").ok_or(anyhow!("Could not strip '.'"))?;
    let (row, column) = s.split_once(", ").ok_or(anyhow!("Could not split row and column"))?;
    let row = row.strip_prefix("row ").ok_or(anyhow!("No 'row' prefix"))?.parse().with_context(|| format!("Could not parse row {row}"))?;
    let column = column.strip_prefix("column ").ok_or(anyhow!("No 'column' prefix"))?.parse().with_context(|| format!("Could not parse column {column}"))?;
    Ok((row, column))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_number() {
        assert_eq!(triangular_number(1), 1);
        assert_eq!(triangular_number(2), 3);
        assert_eq!(triangular_number(3), 6);
        assert_eq!(triangular_number(4), 10);
        assert_eq!(triangular_number(5), 15);
    }

    #[test]
    fn test_get_index() {
        assert_eq!(get_index(1, 1), 1);
        assert_eq!(get_index(4, 2), 12);
        assert_eq!(get_index(1, 5), 15);
    }
}