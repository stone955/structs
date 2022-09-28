fn main() {
    {
        // 直接创建
        let user = User {
            username: String::from("stone"),
            email: String::from("335612970@qq.com"),
            active: true,
            sign_in_count: 1,
        };

        println!(
            "[User] username = {}, email = {}, active = {}, sign_in_count = {}",
            user.username, user.email, user.active, user.sign_in_count,
        );
    }

    {
        // 通过函数创建
        let user = build_user(String::from("stone"), String::from("335612970@qq.com"));

        println!(
            "[build_user] username = {}, email = {}, active = {}, sign_in_count = {}",
            user.username, user.email, user.active, user.sign_in_count,
        );
    }

    {
        // 元组结构体
        let start = Point(0.0, 0.0);
        let end = Point(3.0, 4.0);
        let distance = calculate_distance(&start, &end);
        println!("[calculate_distance] distance = {}", distance);
    }

    {
        // 结构体方法
        let rect = Rectangle::from(40, 50);

        let area = rect.calculate_area();
        println!("[calculate_area] Rectangle's area: {}", area);
    }
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

struct Point(f32, f32);

fn calculate_distance(start: &Point, end: &Point) -> f32 {
    ((end.0 - start.0) * (end.0 - start.0) + (end.1 - start.1) * (end.1 - start.1)).sqrt()
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn from(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }

    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
}
