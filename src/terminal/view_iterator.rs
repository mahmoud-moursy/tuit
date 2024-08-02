use crate::terminal::Rectangle;

pub struct ViewIterator<I: Iterator> {
    pub(crate) child: I,
    pub(crate) parent_dimensions: (usize, usize),
    pub(crate) current_coord: (usize, usize),
    pub(crate) view_rect: Rectangle
}

impl<I: Iterator> Iterator for ViewIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let (parent_width, _parent_height) = self.parent_dimensions;
        let (width, height) = self.view_rect.dimensions();
        let (x, y) = &mut self.current_coord;

        *x += 1;

        if *x > width {
            let first_cell_of_next_line = self.child.nth(parent_width-width);

            *x = 1;
            *y += 1;

            if *y > height {
                return None;
            }

            return first_cell_of_next_line
        }


        self.child.next()
    }
}