use requestty::Question;

fn main() {
    let questions = vec![
        Question::confirm("bacon")
            .message("Do you like bacon?")
            .build(),
        Question::input("favorite")
            .message("Bacon lover, what is your favourite type of bacon?")
            .when(|ans: &requestty::Answers| ans["bacon"].as_bool().unwrap())
            .build(),
        Question::confirm("pizza")
            .message("Ok... Do you like pizza?")
            .when(|ans: &requestty::Answers| !ans["bacon"].as_bool().unwrap())
            .build(),
        Question::input("favourite")
            .message("Whew! What is your favourite type of pizza?")
            .when(|ans: &requestty::Answers| {
                ans.get("pizza")
                    .map(|b| b.as_bool().unwrap())
                    .unwrap_or(false)
            })
            .build(),
    ];

    println!("{:#?}", requestty::prompt(questions));
}
