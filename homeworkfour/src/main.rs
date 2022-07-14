use homeworkfour::question2::sum_u32;
use homeworkfour::question3::calculate_area;
use homeworkfour::question3::{Circle, Square, Triangle};
use homeworkfour::TrafficLight;
use homeworkfour::TrafficLightTrait;

const U32_MAX: u32 = std::u32::MAX;

fn main() {
    // 题目1
    let red_light = TrafficLight::Red;
    red_light.waiting_time();
    TrafficLight::Green.waiting_time();
    TrafficLight::Yellow.waiting_time();

    println!("--------------------------------");

    // 题目2

    let vec1 = vec![U32_MAX, 1];
    let sum1 = sum_u32(&vec1);
    match sum1 {
        Some(v) => println!("Test1: The sum Option enum is {:?}", Some(v)),
        None => {
            println!("Test1: The result is overflow");
            ()
        }
    };

    let vec2 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum2 = sum_u32(&vec2);

    match sum2 {
        Some(v) => println!("Test2: The sum Option enum is {:?}", Some(v)),
        None => {
            println!("Test2: The result is overflow");
            ()
        }
    };

    println!("--------------------------------");

    // 题目3 计算面积

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
