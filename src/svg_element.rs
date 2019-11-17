// (c) 2019 Joost Yervante Damad <joost@damad.be>

use svg::node::element::Text;
use svg::node::element::Path;

pub enum SVGElement {
    Text(Text),
    Path(Path)
}