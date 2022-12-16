// 05：为枚举通信号灯实际发现一个trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同；

fn main() {
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    trait GetTime {
        fn get_time(&self) -> u32; // 返回持续的时间（单位：秒）
    }

    impl GetTime for TrafficLight {
        fn get_time(&self) -> u32 {
            match *self {
                TrafficLight::Red => 10,   
                TrafficLight::Yellow => 5, 
                TrafficLight::Green => 15, 
            }
        }
    }

    let red = TrafficLight::Red; 
    let yellow = TrafficLight::Yellow; 
    let green = TrafficLight::Green; 


    println!("红灯时长: {}, 黄灯时长: {}, 绿灯时长: {}", red.get_time(), yellow.get_time(), green.get_time() );

}
