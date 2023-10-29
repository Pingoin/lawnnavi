use bno055::{BNO055OperationMode, Bno055};
use linux_embedded_hal::{Delay, I2cdev};
use cgmath::{Quaternion as quat,Euler};
use mint::Quaternion;


pub struct InertialNavigationSystem{
    imu:Bno055<I2cdev>,
    delay:Delay,
}

impl InertialNavigationSystem{

    pub fn new(dev:I2cdev)->InertialNavigationSystem{
        let  delay = Delay {};
        let  imu = Bno055::new(dev);

        let mut  result=InertialNavigationSystem{
            delay:delay,
            imu:imu,
        };
    
        result.imu.init(&mut result.delay).expect("An error occurred while building the IMU");
    
        result.imu.set_mode(BNO055OperationMode::NDOF, &mut result.delay)
            .expect("An error occurred while setting the IMU mode");
    
        let status = result.imu.get_calibration_status().unwrap();
        println!("The IMU's calibration status is: {:?}", status);

        result
    }

    pub fn update(&mut self){

        let  quaternion: Quaternion<f32>; // = Quaternion::<f32>::from([0.0, 0.0, 0.0, 0.0]);
        match self.imu.quaternion() {
            Ok(val) => {
                quaternion = val;
                let testval=quat::from(quaternion);
                println!("IMU Euler: {:?}",Euler::from( testval));
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        match self.imu.linear_acceleration() {
            Ok(val) =>println!("IMU Accel: {:?}",val),
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}