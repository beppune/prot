
enum Message {
    Bye,
    Name(String),
    Hello(String),
    Message(String),
}

impl Message  {

    pub fn bye() -> Message {
        Message::Bye
    }

    pub fn name(s:&str) -> Message {
        Message::Name(s.clone())
    }

    pub fn hello(s:&str) -> Message  {
        Message::Hello(s.clone())
    }

    pub fn message(s:&str) -> Message  {
        Message::Message(s.clone());
    }

    pub fn parse(s:&str) -> Message {
        if s == "Bye" {
            Message::Bye
        }

        if s.starts_with("Name[") {
            Message::Name( String::from( ) )
        }
    }

    pub fn from(m:Message) -> String {
        match m {
            Bye => "Bye\r\n",
            Name(n) => {
                format!("Name[{n}]\r\n")
            },
            Hello(n) => {
                format!("Hello[{n}]\r\n")
            },
            Message(n) => {
                format!("Message[{n}]\r\n")
            },
        }
    }

}


