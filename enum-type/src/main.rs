fn main() {
    #[derive(PartialEq)]
    #[derive(Debug)]
    enum StateEnum {
        Active,
        Disable,
        Draft,
        Deleted
    }

    let article_state = StateEnum::Active;

    fn is_active(state: StateEnum) {
        if state == StateEnum::Active {
            println!("yes the state is active");
        } else {
            println!("No, not active");
        }
    }

    is_active(article_state);

    // using structs
    struct Article{
        state: StateEnum,
        title: String,
    }

    let article1 = Article{
        state: StateEnum::Draft,
        title: String::from("Coucou")
    };

    println!("Article 1 state is {:?}", article1.state)
}
