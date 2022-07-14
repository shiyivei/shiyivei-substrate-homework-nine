//题目1

//交通信号灯枚举
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//枚举方法，返回每个每个分支的结果
impl TrafficLight {
    pub fn waiting_time(&self) -> i32 {
        match self {
            TrafficLight::Red => {
                println!("The red light is needed to waiting for {} seconds", 18);
                18
            }
            TrafficLight::Yellow => {
                println!("The Yellow light is needed to waiting for {} seconds", 3);
                3
            }
            TrafficLight::Green => {
                println!("There is no need to wait,just go!");
                0
            }
        }
    }
}

//题目2
pub mod question2;

//题目3
pub mod question3;
