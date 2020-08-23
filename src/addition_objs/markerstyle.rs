pub struct MarkerStyle {
    pub marker: Option<String>,
    pub fillstyle: Option<String>,
}

impl MarkerStyle {
    pub fn new(marker: Option<String>, fillstyle: Option<String>) -> MarkerStyle {
        MarkerStyle { marker, fillstyle }
    }
}
