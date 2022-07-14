//题目3 求面积

//trait
pub trait Area {
    fn area(&self) -> f64;
}

//struct
pub struct Circle {
    pub radius: f64,
}
pub struct Square {
    pub side: f64,
}
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

//为类型实现trait
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

//定义函数，要求类型必须实现Area trait
pub fn calculate_area<T: Area>(graphics: T) -> f64 {
    graphics.area()
}
