use super::range::Range;

#[derive(Clone, Default, Debug)]
pub struct Address {
    sheet_name: String,
    range: Range,
}
impl Address {
    pub(crate) fn get_sheet_name(&self)-> &str {
        &self.sheet_name
    }

    pub(crate) fn set_sheet_name<S: Into<String>>(&mut self, value:S) {
        self.sheet_name = value.into();
    }

    pub(crate) fn get_range(&self)-> &Range {
        &self.range
    }

    pub(crate) fn get_range_mut(&mut self)-> &mut Range {
        &mut self.range
    }

    pub(crate) fn set_range(&mut self, value:Range) {
        self.range = value;
    }

    pub(crate) fn set_address<S: Into<String>>(&mut self, value:S) {
        let org_value = value.into().clone();
        let split_value: Vec<&str> = org_value.split("!").collect();

        if split_value.len() == 1 {
            self.range.set_range(split_value[0]);

        } else if split_value.len() == 2 {
            self.sheet_name = split_value[0].to_string();
            self.range.set_range(split_value[1]);

        } else {
            panic!("Non-standard address");
        }
    }

    pub(crate) fn get_address(&self) -> String {
        let range = self.range.get_range();
        if self.sheet_name == "" {
            return range;
        }
        format!("{}!{}", &self.sheet_name, self.range.get_range())
    }

    pub(crate) fn update_coordinate(&mut self, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        if &self.sheet_name == sheet_name {
            self.range.update_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
        }
    }
}