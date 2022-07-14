use homeworkfour::calculate_area;
use homeworkfour::TrafficLight;
use homeworkfour::{Circle, Square, Triangle};

const U32_MAX: u32 = std::u32::MAX;

fn main() {
    let red_light = TrafficLight::Red;
    red_light.waiting_time();
    TrafficLight::Green.waiting_time();
    TrafficLight::Yellow.waiting_time();

    // 求u32的和

    let vec = vec![U32_MAX - 1, 1];

    let sum = sum_u32(&vec);
    match sum {
        Some(v) => println!("The sum Option enum is {:?}", Some(v)),
        None => {
            println!("The result is overflow");
            ()
        }
    };

    // 计算面积

    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };

    let square = Square { side: 5.0 };

    let circle = Circle { radius: 10.0 };

    println!("The area of triangle is {}", calculate_area(triangle));
    println!("The area of square is {}", calculate_area(square));
    println!("The area of circle is {}", calculate_area(circle));
}

//题目2   u32 类型求和
pub fn sum_u32(vec: &[u32]) -> Option<u32> {
    let mut sum = 0;

    for i in vec.iter() {
        sum += i;
    }

    sum.checked_add(0)
}
