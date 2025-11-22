pub mod time;

use crate::errors::Error;
use crate::errors::Result;
use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum ShutterPosition {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShutterLocation {
    TopLeft,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ShutterPositions {
    open_shutters: HashSet<ShutterLocation>,
}

impl ShutterPositions {
    pub fn new(open_shutters: &[ShutterLocation]) -> Result<Self> {
        let open_shutters_set: HashSet<ShutterLocation> = HashSet::from_iter(open_shutters.iter().map(|v| v.clone()));
        if open_shutters_set.len() != open_shutters.len() {
           return Err(anyhow!("shutter locations have duplicate values in them").into());
        }
        Ok(Self { open_shutters: open_shutters_set })
    }

    pub fn all_closed(&self) -> bool {
        self.open_shutters.is_empty()
    }
}

impl Hash for ShutterPositions {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut locations: Vec<&ShutterLocation> = self.open_shutters.iter().collect();
        locations.sort();
        for s in locations {
            s.hash(state);
        }
    }
}

pub struct Message {
    text: String,
}

pub struct EncodedMessage {
    parts: Vec<EncodedMessagePart>,
}

impl EncodedMessage {
    fn new(parts: Vec<EncodedMessagePart>) -> Result<EncodedMessage> {
        for (i, part) in parts.iter().enumerate() {
            match part.element {
                MessageComponent::Character(_) => {
                    if i == parts.len() - 1 {
                        return Err(anyhow!(
                            "characters can't appear as the last element of an encoded message"
                        )
                        .into());
                    }
                }
                MessageComponent::End => {
                    if i != parts.len() - 1 {
                        return Err(anyhow!(
                            "message end indicator can only appear as the last element of an encoded message"
                        )
                        .into());
                    }
                }
            }
        }
        Ok(EncodedMessage { parts })
    }
}

pub struct EncodedMessagePart {
    element: MessageComponent,
    encoding: ShutterPositions,
}

impl EncodedMessagePart {
    pub fn new(element: MessageComponent, encoding: ShutterPositions) -> Self {
        Self { element, encoding }
    }
}

impl EncodedMessagePart {}

pub enum MessageComponent {
    Character(char),
    End,
}

pub struct CurrentMessage {
    before: Vec<EncodedMessagePart>,
    current: Option<EncodedMessagePart>,
    after: Vec<EncodedMessagePart>,
}

pub struct Encoding {
    characters: HashMap<String, ShutterPositions>,
    message_end: ShutterPositions,
}

impl Default for Encoding {
    fn default() -> Self {
        let mut characters:HashMap<String, ShutterPositions> = HashMap::new();
        characters.insert("A".into(), ShutterPositions::new(&[ShutterLocation::MiddleLeft, ShutterLocation::BottomRight]).unwrap());
        characters.insert("B".into(), ShutterPositions::new(&[ShutterLocation::MiddleRight, ShutterLocation::BottomLeft]).unwrap());
        characters.insert("C".into(), ShutterPositions::new(&[ShutterLocation::MiddleLeft, ShutterLocation::MiddleRight]).unwrap());
        //todo
        // characters.insert(" ".into(), ShutterPositions::new([ShutterLocation::BottomLeft].into()));

        Self::new(characters, ShutterPositions::new(&[ShutterLocation::TopLeft, ShutterLocation::TopRight, ShutterLocation::BottomLeft,ShutterLocation::BottomRight]).unwrap()).unwrap()
    }
}

impl Encoding {
    pub fn new(
        characters: HashMap<String, ShutterPositions>,
        message_end: ShutterPositions,
    ) -> Result<Self> {
        let mut positions: HashSet<ShutterPositions> = HashSet::new();

        if characters.is_empty() {
            return Err(anyhow!(
                "if the characters mapping is empty then we can't encode anything"
            )
            .into());
        }

        let characters: HashMap<String,ShutterPositions> = characters.iter().map(|(k, v)| (k.to_uppercase(), v.clone())).collect();

        for (character, position) in &characters {
            if position.all_closed() {
                return Err(anyhow!("character encoding can't be all shutters closed").into());
            }

            if positions.contains(&position) {
                return Err(
                    anyhow!("duplicate shutter position for character '{}'", character).into(),
                );
            }
            positions.insert(position.clone());
        }

        if message_end.all_closed() {
            return Err(anyhow!("message end encoding can't be all shutters closed").into());
        }

        if positions.contains(&message_end) {
            return Err(anyhow!("duplicate shutter position for message end").into());
        }

        Ok(Self {
            characters,
            message_end,
        })
    }

    pub fn encode(&self, message: &Message) -> Result<EncodedMessage> {
        let mut parts = vec![];

        for c in message.text.chars() {
            let uppercase_string =String::from(c).to_uppercase();
            match self.characters.get(&uppercase_string) {
                Some(positions) => {
                    parts.push(EncodedMessagePart::new(
                        MessageComponent::Character(c),
                        positions.clone(),
                    ));
                }
                None => {
                    return Err(Error::CannotEncodeCharacter(c));
                }
            }
        }
        parts.push(EncodedMessagePart::new(
            MessageComponent::End,
            self.message_end.clone(),
        ));

        EncodedMessage::new(parts)
    }
}

#[derive(Clone)]
pub struct Queue {
    messages: Arc<Mutex<Vec<Message>>>,
    max_messages: usize,
}

impl Queue {
    pub fn new(max_messages: usize) -> Result<Self> {
        if max_messages == 0 {
            return Err(anyhow!("max_messages in the queue can't be set to zero").into());
        }
        Ok(Self {
            messages: Arc::new(Mutex::new(vec![])),
            max_messages,
        })
    }

    pub fn add_message(&self, message: Message) -> Result<()> {
        let mut messages = self.messages.lock().unwrap();
        if messages.len() >= self.max_messages {
            return Err(Error::QueueIsFull);
        }
        messages.push(message);
        Ok(())
    }

    pub fn pop_message(&self, _message: Message) -> Option<Message> {
        let mut messages = self.messages.lock().unwrap();
        messages.pop()
    }
}

pub struct Clacks {
    current_message: Option<CurrentMessage>,
    queue: Queue,
}

impl Clacks {
    pub fn new(queue: Queue) -> Self {
        Self {
            current_message: None,
            queue,
        }
    }

    pub fn update(&self) -> Result<()> {
        Err(anyhow!("Not implemented").into())
    }
}
