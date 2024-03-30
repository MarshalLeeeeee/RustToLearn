enum StatesEnum {
    New,
    Doing,
    Done,
}

struct StateMachineEnum {
    state: StatesEnum,
    data: String,
}

impl StateMachineEnum {
    fn new() -> Self {
        Self {
            state: StatesEnum::New,
            data: String::from(""),
        }
    }
    fn start(&mut self) {
        match self.state {
            StatesEnum::New => { self.state = StatesEnum::Doing; },
            _ => { println!("Already start."); },
        }
    }
    fn do_content(&mut self, s: &str) {
        match self.state {
            StatesEnum::Doing => { self.data = String::from(s); },
            _ => { println!("Not start or have done."); },
        }
    }
    fn submit(&mut self) {
        match self.state {
            StatesEnum::Doing => { self.state = StatesEnum::Done; },
            _ => { println!("Not start or have done."); },
        }
    }
    fn preview(&self) -> &str {
        self.data.as_str()
    }
    fn review(&self) -> &str {
        match self.state {
            StatesEnum::New => "",
            StatesEnum::Doing => "",
            StatesEnum::Done => self.data.as_str(),
        }
    }
}

fn test_state_pattern_by_enum() {
    println!("---------- test_state_pattern_by_enum ----------");

    println!("---------- new ----------");
    let mut sm = StateMachineEnum::new();
    sm.do_content("content");
    sm.submit();
    println!("Preview: {}", sm.preview());
    println!("Review: {}", sm.review());
    sm.start();

    println!("---------- doing ----------");
    sm.start();
    sm.do_content("content");
    println!("Preview: {}", sm.preview());
    println!("Review: {}", sm.review());
    sm.submit();

    println!("---------- done ----------");
    sm.start();
    sm.do_content("content");
    sm.submit();
    println!("Preview: {}", sm.preview());
    println!("Review: {}", sm.review());
}

trait StateTrait {
    fn start(self: Box<Self>) -> Box<dyn StateTrait>;
    fn submit(self: Box<Self>) -> Box<dyn StateTrait>;
    fn check_can_modify(&self) -> bool { false }
    fn preview<'a>(&self, sm: &'a StateMachineDyn) -> &'a str { sm.data.as_str() }
    fn review<'a>(&self, sm: &'a StateMachineDyn) -> &'a str { "" }
}

struct StateDynNew;
impl StateTrait for StateDynNew {
    fn start(self: Box<Self>) -> Box<dyn StateTrait> {
        Box::new(StateDynDoing)
    }
    fn submit(self: Box<Self>) -> Box<dyn StateTrait> {
        println!("Not start or have done.");
        self
    }
}

struct StateDynDoing;
impl StateTrait for StateDynDoing {
    fn start(self: Box<Self>) -> Box<dyn StateTrait> {
        println!("Already start.");
        self
    }
    fn submit(self: Box<Self>) -> Box<dyn StateTrait> {
        Box::new(StateDynDone)
    }
    fn check_can_modify(&self) -> bool { true }
}

struct StateDynDone;
impl StateTrait for StateDynDone {
    fn start(self: Box<Self>) -> Box<dyn StateTrait> {
        println!("Already start.");
        self
    }
    fn submit(self: Box<Self>) -> Box<dyn StateTrait> {
        println!("Not start or have done.");
        self
    }
    fn review<'a>(&self, sm: &'a StateMachineDyn) -> &'a str {
        sm.data.as_str()
    }
}

struct StateMachineDyn {
    state: Option<Box<dyn StateTrait>>,
    data: String,
}

impl StateMachineDyn {
    fn new() -> Self {
        Self {
            state: Some(Box::new(StateDynNew)),
            data: String::from(""),
        }
    }
    fn start(&mut self) {
        self.state = Some(self.state.take().unwrap().start());
    }
    fn do_content(&mut self, s: &str) {
        if self.state.as_ref().unwrap().check_can_modify() {
            self.data = String::from(s);
        }
        else {
            println!("Not start or have done.");
        }
    }
    fn submit(&mut self) {
        self.state = Some(self.state.take().unwrap().submit());
    }
    fn preview(&self) -> &str {
        self.state.as_ref().unwrap().preview(self)
    }
    fn review(&self) -> &str {
        self.state.as_ref().unwrap().review(self)
    }
}

fn test_state_pattern_by_dyn() {
    println!("---------- test_state_pattern_by_dyn ----------");

    println!("---------- new ----------");
    let mut sm = StateMachineDyn::new();
    sm.do_content("content");
    sm.submit();
    println!("Preview: {}", sm.preview());
    println!("Review: {}", sm.review());
    sm.start();

    println!("---------- doing ----------");
    sm.start();
    sm.do_content("content");
    println!("Preview: {}", sm.preview());
    println!("Review: {}", sm.review());
    sm.submit();

    println!("---------- done ----------");
    sm.start();
    sm.do_content("content");
    sm.submit();
    println!("Preview: {}", sm.preview());
    println!("Review: {}", sm.review());
}

struct StateNew;
impl StateNew {
    fn start(self) -> StateDoing {
        StateDoing::new()
    }
}

struct StateDoing {
    s: String,
}
impl StateDoing {
    fn new() -> Self {
        Self {
            s: String::from(""),
        }
    }
    fn do_content(&mut self, s: &str) {
        self.s = String::from(s);
    }
    fn submit(self) -> StateDone {
        StateDone::new(self.s.as_str())
    }
}

struct StateDone {
    s: String,
}
impl StateDone {
    fn new(ss: &str) -> Self {
        Self {
            s: String::from(ss),
        }
    }
    fn review(&self) -> &str {
        self.s.as_str()
    }
}

fn test_trans_pattern() {
    println!("---------- test_trans_pattern ----------");

    let new_s = StateNew;
    let mut doing_s = new_s.start();
    doing_s.do_content("content");
    let done_s = doing_s.submit();
    println!("Review: {}", done_s.review());
}

fn main() {
    test_state_pattern_by_enum();
    test_state_pattern_by_dyn();
    test_trans_pattern();
}
