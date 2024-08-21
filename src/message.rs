
pub enum Message {
    Bye,
    Name(String),
    Hello(String),
    Message(String),
}

impl Message  {

    pub fn deserialize(m:Message) -> String {
        match m {
            Self::Bye => String::from("Bye\r\n"),
            Self::Name(n) => {
                format!("Name[{n}]\r\n")
            },
            Self::Hello(n) => {
                format!("Hello[{n}]\r\n")
            },
            Self::Message(n) => {
                format!("Message[{n}]\r\n")
            },
        }
    }

    pub fn serialize(s:&str) -> Message {
        match s {
            s if s.starts_with("Bye") => Message::Bye,

            s if s.starts_with("Name[") => {
                let end = s.chars().position(|x| x == ']').unwrap();
                Message::Name( String::from(&s[5..end]) )
            },

            s if s.starts_with("Hello[") => {
                let end = s.chars().position(|x| x == ']').unwrap();
                Message::Hello( String::from( &s[6..end] ) )
            },

            s if s.starts_with("Message[") => {
                let end = s.chars().position(|x| x == ']').unwrap();
                Message::Hello( String::from( &s[9..end] ) )
            },

            _ => { panic!("Unknown message type: {s}"); }
        }
    }

}


