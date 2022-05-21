pub trait FieldPrintDebug {
    fn print_debug(&self);
}

impl FieldPrintDebug for super::Field {
    fn print_debug(&self) {
        #[allow(non_upper_case_globals)]
        const align: &str = "       ";
        #[allow(non_upper_case_globals)]
        const sidet: &str = "┌─┬─┬─┐";
        #[allow(non_upper_case_globals)]
        const sidem: &str = "├─┼─┼─┤";
        #[allow(non_upper_case_globals)]
        const sideb: &str = "└─┴─┴─┘";
        #[allow(non_upper_case_globals)]
        const nl: &str = "\n";
        macro_rules! w {($($t:expr)+) => {$(print!("{}", $t);)+};}

        w!(align sidet nl);
        w!(align format!("│{}│{}│{}│", self.cells[0], self.cells[1], self.cells[2]) nl);
        w!(align sidem nl);
        w!(align format!("│{}│{}│{}│", self.cells[3], self.cells[4], self.cells[5]) nl);
        w!(align sidem nl);
        w!(align format!("│{}│{}│{}│", self.cells[6], self.cells[7], self.cells[8]) nl);
        w!(align sideb nl);

        for _ in 0..4 {
            w!(sidet);
        }
        w!(nl);
        w!(format!(
            "│{}│{}│{}│",
            self.cells[9], self.cells[10], self.cells[11]
        ));
        w!(format!(
            "│{}│{}│{}│",
            self.cells[18], self.cells[19], self.cells[20]
        ));
        w!(format!(
            "│{}│{}│{}│",
            self.cells[27], self.cells[28], self.cells[29]
        ));
        w!(format!("│{}│{}│{}│", self.cells[36], self.cells[37], self.cells[38]) nl);
        for _ in 0..4 {
            w!(sidem);
        }
        w!(nl);
        w!(format!(
            "│{}│{}│{}│",
            self.cells[9 + 3],
            self.cells[10 + 3],
            self.cells[11 + 3]
        ));
        w!(format!(
            "│{}│{}│{}│",
            self.cells[18 + 3],
            self.cells[19 + 3],
            self.cells[20 + 3]
        ));
        w!(format!(
            "│{}│{}│{}│",
            self.cells[27 + 3],
            self.cells[28 + 3],
            self.cells[29 + 3]
        ));
        w!(format!("│{}│{}│{}│", self.cells[36+3], self.cells[37+3], self.cells[38+3]) nl);
        for _ in 0..4 {
            w!(sidem);
        }
        w!(nl);
        w!(format!(
            "│{}│{}│{}│",
            self.cells[9 + 6],
            self.cells[10 + 6],
            self.cells[11 + 6]
        ));
        w!(format!(
            "│{}│{}│{}│",
            self.cells[18 + 6],
            self.cells[19 + 6],
            self.cells[20 + 6]
        ));
        w!(format!(
            "│{}│{}│{}│",
            self.cells[27 + 6],
            self.cells[28 + 6],
            self.cells[29 + 6]
        ));
        w!(format!("│{}│{}│{}│", self.cells[36+6], self.cells[37+6], self.cells[38+6]) nl);
        for _ in 0..4 {
            w!(sideb);
        }
        w!(nl);

        w!(align sidet nl);
        w!(align format!("│{}│{}│{}│", self.cells[45], self.cells[46], self.cells[47]) nl);
        w!(align sidem nl);
        w!(align format!("│{}│{}│{}│", self.cells[48], self.cells[49], self.cells[50]) nl);
        w!(align sidem nl);
        w!(align format!("│{}│{}│{}│", self.cells[51], self.cells[52], self.cells[53]) nl);
        w!(align sideb nl);
    }
}
