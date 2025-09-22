use rand::Rng;
use std::fmt;
use std::io;

#[derive(Clone)]
struct Member {
    member_id: u32,
    first_name: String,
    last_name: String,
    gender: Gender,
    email: String,
    subscriptions: Vec<Service>,
    purchases: Vec<Item>,
}

#[derive(Clone)]
enum Gender {
    Male,
    Female,
    NonBinary,
    Other(String),
    PreferNotToSay,
}

#[derive(Clone, PartialEq)]
enum Service {
    YogaMondays,
    ScubaLessons,
    WeightTraining,
}

#[derive(Clone, PartialEq)]
enum Item {
    Pepsi,
    DietPepsi,
    SierraMist,
    MountainDew,
    Powerade,
    Coffee,
    Latte,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::NonBinary => write!(f, "Non-Binary"),
            Gender::Other(custom_gender) => write!(f, "{}", custom_gender),
            Gender::PreferNotToSay => write!(f, "N/A"),
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Service::YogaMondays => write!(f, "Mandy's Yoga Mondays"),
            Service::ScubaLessons => write!(f, "Monthly Scuba Lessons"),
            Service::WeightTraining => write!(f, "Willy's Weight Training"),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Pepsi => write!(f, "Pepsi"),
            Item::DietPepsi => write!(f, "Diet Pepsi"),
            Item::SierraMist => write!(f, "Sierra Mist"),
            Item::MountainDew => write!(f, "MountainDew"),
            Item::Powerade => write!(f, "Powerade"),
            Item::Coffee => write!(f, "Coffee"),
            Item::Latte => write!(f, "Latte"),
        }
    }
}

fn newMember(memberList: &mut Vec<Member>) {
    
}

fn uSetter() {

}

fn uGetter() {

}

fn main() {
    println!(
        "COMMANDS:\nnew - make new member\nsubscribe - subscribe a member to a service\npurchase - add a purchase to a member's account\nmembers - provides a list of members in the gym's records\nsubscriptions - gives a list of subscriptions for particular user\npurchases - shows a user's purchase history\nhelp - print this menu again"
    );

    let mut members: Vec<Member> = populate();

    loop {
        println!("Enter command:");

        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Invalid command");

        let first_word: &str = cmd.split_whitespace().next().unwrap_or("");

        if first_word == "new" {
            let (mut first, mut last, mut mail, mut gender_raw) =
                (String::new(), String::new(), String::new(), String::new());

            println!("Enter user's first name:");
            io::stdin().read_line(&mut first).expect("Invalid command");

            println!("Enter user's last name:");
            io::stdin().read_line(&mut last).expect("Invalid command");

            println!("Enter user's email address:");
            io::stdin().read_line(&mut mail).expect("Invalid command");

            println!(
                "User's gender? Enter 1 for \"Male\", 2 for \"Female\", 3 for \"Non-Binary\", or you can type something else that describes you best! (Also feel free to leave blank)"
            );
            io::stdin()
                .read_line(&mut gender_raw)
                .expect("Invalid command");

            members.push(Member {
                member_id: rand::rng().random_range(1000..9999),
                first_name: first.trim().to_string(),
                last_name: last.trim().to_string(),
                gender: if gender_raw.trim() == "1" {
                    Gender::Male
                } else if gender_raw.trim() == "2" {
                    Gender::Female
                } else if gender_raw.trim() == "3" {
                    Gender::NonBinary
                } else if gender_raw.trim() == "" {
                    Gender::PreferNotToSay
                } else {
                    Gender::Other(gender_raw.trim().into())
                },
                email: mail.trim().to_string(),
                subscriptions: vec![],
                purchases: vec![],
            });
        } else if first_word == "subscribe" {
            let mut uid = String::new();

            println!("Please enter your User ID:");
            io::stdin().read_line(&mut uid).expect("Invalid command");

            if let Ok(int_id) = uid.trim().parse::<u32>() {
                let mut member_iter = members.clone().into_iter();
                if let Some(user_index) = member_iter.position(|usr| usr.member_id == int_id) {
                    let mut serv = String::new();
                    println!(
                        "Hello, {}. What service would you like to subscribe to? Enter 1 for Mandy's Yoga Mondays, 2 for Monthly Scuba Lessons, 3 for Willy's Weight Training",
                        members[user_index].first_name
                    );
                    io::stdin().read_line(&mut serv).expect("Invalid command");
                    if serv.trim() == "1" {
                        if !members[user_index]
                            .subscriptions
                            .contains(&Service::YogaMondays)
                        {
                            members[user_index].subscriptions.push(Service::YogaMondays);
                        } else {
                            println!("You are already subscribed to that service!");
                        }
                    } else if serv.trim() == "2" {
                        if !members[user_index]
                            .subscriptions
                            .contains(&Service::ScubaLessons)
                        {
                            members[user_index]
                                .subscriptions
                                .push(Service::ScubaLessons);
                        } else {
                            println!("You are already subscribed to that service!");
                        }
                    } else if serv.trim() == "3" {
                        if !members[user_index]
                            .subscriptions
                            .contains(&Service::WeightTraining)
                        {
                            members[user_index]
                                .subscriptions
                                .push(Service::WeightTraining);
                        } else {
                            println!("You are already subscribed to that service!");
                        }
                    } else {
                        println!("Invalid input, no services rendered");
                    }
                } else {
                    println!("No user with that ID");
                }
            } else {
                println!("Not a valid user ID");
            }
        } else if first_word == "purchase" {
            let mut uid = String::new();

            println!("Please enter your User ID:");
            io::stdin().read_line(&mut uid).expect("Invalid command");

            if let Ok(int_id) = uid.trim().parse::<u32>() {
                let mut member_iter = members.clone().into_iter();
                if let Some(user_index) = member_iter.position(|usr| usr.member_id == int_id) {
                    let mut itm = String::new();
                    println!(
                        "Hello, {}. What item would you like to purchase? Enter 1 for Pepsi, 2 for Diet Pepsi, 3 for Sierra Mist, 4 for Mountain Dew, 5 for Powerade, 6 for Coffee, 7 for Latte",
                        members[user_index].first_name
                    );
                    io::stdin().read_line(&mut itm).expect("Invalid command");
                    if itm.trim() == "1" {
                        members[user_index].purchases.push(Item::Pepsi);
                    } else if itm.trim() == "2" {
                        members[user_index].purchases.push(Item::DietPepsi);
                    } else if itm.trim() == "3" {
                        members[user_index].purchases.push(Item::SierraMist);
                    } else if itm.trim() == "4" {
                        members[user_index].purchases.push(Item::MountainDew);
                    } else if itm.trim() == "5" {
                        members[user_index].purchases.push(Item::Powerade);
                    } else if itm.trim() == "6" {
                        members[user_index].purchases.push(Item::Coffee);
                    } else if itm.trim() == "7" {
                        members[user_index].purchases.push(Item::Latte);
                    } else {
                        println!("Invalid input, no item purchased");
                    }
                } else {
                    println!("No user with that ID");
                }
            } else {
                println!("Not a valid user ID");
            }
        } else if first_word == "members" {
            let user_iter = members.clone().into_iter();
            for member in user_iter {
                println! {"{} - {} {} (Gender: {}), {}", member.member_id, member.first_name, member.last_name, member.gender, member.email};
            }
        } else if first_word == "subscriptions" {
            let mut uid = String::new();

            println!("Please enter your User ID:");
            io::stdin().read_line(&mut uid).expect("Invalid command");

            if let Ok(int_id) = uid.trim().parse::<u32>() {
                if let Some(user) = members
                    .clone()
                    .into_iter()
                    .find(|member| member.member_id == int_id)
                {
                    for subscription in user.subscriptions {
                        println!("{}", subscription);
                    }
                } else {
                    println!("No user with that ID");
                }
            } else {
                println!("Not a valid user ID");
            }
        } else if first_word == "purchases" {
            let mut uid = String::new();

            println!("Please enter your User ID:");
            io::stdin().read_line(&mut uid).expect("Invalid command");

            if let Ok(int_id) = uid.trim().parse::<u32>() {
                if let Some(user) = members
                    .clone()
                    .into_iter()
                    .find(|member| member.member_id == int_id)
                {
                    for purchase in user.purchases {
                        println!("{}", purchase);
                    }
                } else {
                    println!("No user with that ID");
                }
            } else {
                println!("Not a valid user ID");
            }
        } else if first_word == "quit" || first_word == "exit" {
            break;
        } else if first_word == "help" {
            println!(
                "COMMANDS:\nnew - make new member\nsubscribe - subscribe a member to a service\npurchase - add a purchase to a member's account\nmembers - provides a list of members in the gym's records\nsubscriptions - gives a list of subscriptions for particular user\npurchases - shows a user's purchase history\nhelp - print this menu again"
            );
        } else {
            println!("Not a recognized command, try again");
        }

        println!("");
    }
}

fn populate() -> Vec<Member> {
    vec![
        Member {
            member_id: 8492,
            first_name: String::from("Alex"),
            last_name: String::from("Algazi"),
            gender: Gender::Male,
            email: String::from("alex.algazi@fakesite.com"),
            subscriptions: vec![Service::YogaMondays, Service::ScubaLessons],
            purchases: vec![
                Item::DietPepsi,
                Item::DietPepsi,
                Item::DietPepsi,
                Item::DietPepsi,
            ],
        },
        Member {
            member_id: 2014,
            first_name: String::from("Justin"),
            last_name: String::from("Clitheroe"),
            gender: Gender::NonBinary,
            email: String::from("jclithy@veryrealdomain.com"),
            subscriptions: vec![Service::YogaMondays],
            purchases: vec![Item::Latte, Item::DietPepsi],
        },
        Member {
            member_id: 1234,
            first_name: String::from("Jenny"),
            last_name: String::from("Galverson"),
            gender: Gender::Female,
            email: String::from("jennysaykwah@clickhere.com"),
            subscriptions: vec![Service::ScubaLessons],
            purchases: vec![],
        },
    ]
}
