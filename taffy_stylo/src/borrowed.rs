use crate::{stylo, t2s};

pub struct TaffyStyloStyleRef<'a>(pub &'a stylo::ComputedValues);

impl<'a> From<&'a stylo::ComputedValues> for TaffyStyloStyleRef<'a> {
    fn from(value: &'a stylo::ComputedValues) -> Self {
        Self(value)
    }
}

impl taffy::CoreStyle for TaffyStyloStyleRef<'_> {
    #[inline]
    fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
        t2s::box_generation_mode(self.0.get_box().display)
    }

    #[inline]
    fn is_block(&self) -> bool {
        t2s::is_block(self.0.get_box().display)
    }

    #[inline]
    fn overflow(&self) -> taffy::Point<taffy::Overflow> {
        let box_styles = self.0.get_box();
        taffy::Point { x: t2s::overflow(box_styles.overflow_x), y: t2s::overflow(box_styles.overflow_y) }
    }

    #[inline]
    fn scrollbar_width(&self) -> f32 {
        0.0
    }

    #[inline]
    fn position(&self) -> taffy::Position {
        t2s::position(self.0.get_box().position)
    }

    #[inline]
    fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        let position_styles = self.0.get_position();
        taffy::Rect {
            left: t2s::length_percentage_auto(&position_styles.left),
            right: t2s::length_percentage_auto(&position_styles.right),
            top: t2s::length_percentage_auto(&position_styles.top),
            bottom: t2s::length_percentage_auto(&position_styles.bottom),
        }
    }

    #[inline]
    fn size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size { width: t2s::dimension(&position_styles.width), height: t2s::dimension(&position_styles.height) }
    }

    #[inline]
    fn min_size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: t2s::dimension(&position_styles.min_width),
            height: t2s::dimension(&position_styles.min_height),
        }
    }

    #[inline]
    fn max_size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: t2s::max_size_dimension(&position_styles.max_width),
            height: t2s::max_size_dimension(&position_styles.max_height),
        }
    }

    #[inline]
    fn aspect_ratio(&self) -> Option<f32> {
        t2s::aspect_ratio(self.0.get_position().aspect_ratio)
    }

    #[inline]
    fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        let margin_styles = self.0.get_margin();
        taffy::Rect {
            left: t2s::length_percentage_auto(&margin_styles.margin_left),
            right: t2s::length_percentage_auto(&margin_styles.margin_right),
            top: t2s::length_percentage_auto(&margin_styles.margin_top),
            bottom: t2s::length_percentage_auto(&margin_styles.margin_bottom),
        }
    }

    #[inline]
    fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
        let padding_styles = self.0.get_padding();
        taffy::Rect {
            left: t2s::length_percentage(&padding_styles.padding_left.0),
            right: t2s::length_percentage(&padding_styles.padding_right.0),
            top: t2s::length_percentage(&padding_styles.padding_top.0),
            bottom: t2s::length_percentage(&padding_styles.padding_bottom.0),
        }
    }

    #[inline]
    fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
        let border_styles = self.0.get_border();
        taffy::Rect {
            left: taffy::LengthPercentage::Length(border_styles.border_left_width.to_f32_px()),
            right: taffy::LengthPercentage::Length(border_styles.border_right_width.to_f32_px()),
            top: taffy::LengthPercentage::Length(border_styles.border_top_width.to_f32_px()),
            bottom: taffy::LengthPercentage::Length(border_styles.border_bottom_width.to_f32_px()),
        }
    }
}

impl taffy::FlexboxContainerStyle for TaffyStyloStyleRef<'_> {
    #[inline]
    fn flex_direction(&self) -> taffy::FlexDirection {
        t2s::flex_direction(self.0.get_position().flex_direction)
    }

    #[inline]
    fn flex_wrap(&self) -> taffy::FlexWrap {
        t2s::flex_wrap(self.0.get_position().flex_wrap)
    }

    #[inline]
    fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: t2s::gap(&position_styles.column_gap),
            height: taffy::LengthPercentage::Length(0.0), // TODO: enable row_gap in stylo
        }
    }

    #[inline]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        t2s::align_content(self.0.get_position().align_content)
    }

    #[inline]
    fn align_items(&self) -> Option<taffy::AlignItems> {
        t2s::align_items(self.0.get_position().align_items)
    }

    #[inline]
    fn justify_content(&self) -> Option<taffy::JustifyContent> {
        t2s::justify_content(self.0.get_position().justify_content)
    }
}

impl taffy::FlexboxItemStyle for TaffyStyloStyleRef<'_> {
    #[inline]
    fn flex_basis(&self) -> taffy::Dimension {
        t2s::flex_basis(&self.0.get_position().flex_basis)
    }

    #[inline]
    fn flex_grow(&self) -> f32 {
        self.0.get_position().flex_grow.0
    }

    #[inline]
    fn flex_shrink(&self) -> f32 {
        self.0.get_position().flex_grow.0
    }

    #[inline]
    fn align_self(&self) -> Option<taffy::AlignSelf> {
        t2s::align_self(self.0.get_position().align_self)
    }
}
