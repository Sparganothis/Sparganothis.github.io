pub struct GameBoardTetStyle {
    pub s: String,
    pub z: String,
    pub t: String,
    pub o: String,
    pub i: String,
    pub j: String,
    pub l: String,
}

impl GameBoardTetStyle{
    pub fn new () -> Self{
       GameBoardTetStyle {
        s: "#74C21D".to_string(),
        z: "#FF4A58".to_string(),
        t: "#DA5DB2".to_string(),
        o: "#FFC125".to_string(),
        i: "#21B6F8".to_string(),
        j: "#4169E7".to_string(),
        l: "#FF8720".to_string(),
    }
}
}