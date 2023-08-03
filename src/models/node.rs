use uuid::Uuid;

pub enum Heading {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

// It would be good to support various formattable text variants. i.e. typst. This would give us good flexibility

pub enum FormattbleText {
    Markdown,
    Typst,
    Peritext,
    Code { lang: String },
}

pub struct Scene3d {
    pub scene_id: Uuid,
}

pub struct Scene2d {
    pub scene_id: Uuid,
}

pub enum Node {
    Scene3d(Scene3d),
    Scene2d(Scene2d),
    Heading(FormattbleText),
    Paragraph(FormattbleText),
}
