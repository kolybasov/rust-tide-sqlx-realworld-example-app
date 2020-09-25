pub fn generate_mass_insert_placeholder<T>(items: &[T], items_in_the_row: usize) -> String {
    (1..=(items.len() * items_in_the_row))
        .step_by(items_in_the_row)
        .map(|step| {
            let row_placeholder = (step..(step + items_in_the_row))
                .map(|index| format!("${}", index))
                .collect::<Vec<String>>()
                .join(",");

            format!("({})", row_placeholder)
        })
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_placeholder_string_test() {
        assert_eq!(
            generate_mass_insert_placeholder(&[1, 2, 3], 1),
            "($1),($2),($3)"
        );
        assert_eq!(
            generate_mass_insert_placeholder(&[1, 2], 2),
            "($1,$2),($3,$4)"
        );
        assert_eq!(
            generate_mass_insert_placeholder(&[1, 2], 3),
            "($1,$2,$3),($4,$5,$6)"
        );
    }
}
