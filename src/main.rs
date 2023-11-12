struct User {
    name: String,
    balance: (f32, String),
}

impl User {
    fn print_user_detail (&self) {
        println!("{} {} {}", self.name, self.balance.0, self.balance.1);
    }
}

fn accrue_interest (user: &mut User, rate: f32) {
    user.balance.0 = user.balance.0 * rate;
    user.print_user_detail();
}

fn compound_interest (user: &mut User, rate: f32, m: u32) {
    for _ in 0..m {
        //accrue_interest(user, ((1.0 + (rate - 1.0) / m as f32)).powi(m as i32));
        accrue_interest(user, rate);
    }
}

fn main() {
    let mut user = User {
        name: "John".to_owned(),
        balance: (100.0, "SGD".to_owned()),
    };
    accrue_interest(&mut user, 1.1);
    compound_interest(&mut user, 1.1, 2)
}