pub mod time;

use crate::app::ClacksUpdateResult;
use crate::domain::time::Duration;
use crate::errors::Error;
use crate::errors::Result;
use anyhow::anyhow;
use rand::seq::IndexedRandom;
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

pub const MAX_MESSAGE_LEN_BYTES: usize = 20;

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
        let open_shutters_set: HashSet<ShutterLocation> =
            HashSet::from_iter(open_shutters.iter().cloned());
        if open_shutters_set.len() != open_shutters.len() {
            return Err(anyhow!("shutter locations have duplicate values in them").into());
        }
        Ok(Self {
            open_shutters: open_shutters_set,
        })
    }

    pub fn all_closed(&self) -> bool {
        self.open_shutters.is_empty()
    }

    pub fn open_shutters(&self) -> Iter<'_, ShutterLocation> {
        self.open_shutters.iter()
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

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    text: String,
}

impl Message {
    pub(crate) fn new(text: impl Into<String>) -> Result<Self> {
        let text = text.into();

        if text.is_empty() {
            return Err(anyhow!("empty message").into());
        }
        if text.len() > MAX_MESSAGE_LEN_BYTES {
            // yes, it's unclear if that's what we want
            return Err(anyhow!("message too long").into());
        }
        Ok(Self { text })
    }
}

#[derive(Clone)]
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

    pub fn parts(&self) -> &[EncodedMessagePart] {
        &self.parts
    }
}

#[derive(Clone)]
pub struct EncodedMessagePart {
    element: MessageComponent,
    shutter_positions: ShutterPositions,
}

impl EncodedMessagePart {
    pub fn new(element: MessageComponent, encoding: ShutterPositions) -> Self {
        Self {
            element,
            shutter_positions: encoding,
        }
    }

    pub fn element(&self) -> &MessageComponent {
        &self.element
    }

    pub fn shutter_positions(&self) -> &ShutterPositions {
        &self.shutter_positions
    }
}

impl EncodedMessagePart {}

#[derive(Clone)]
pub enum MessageComponent {
    Character(String),
    End,
}

pub struct CurrentMessage {
    before: Vec<EncodedMessagePart>,
    current: Option<EncodedMessagePart>,
    after: Vec<EncodedMessagePart>,
}

impl CurrentMessage {
    pub fn new(
        before: Vec<EncodedMessagePart>,
        current: Option<EncodedMessagePart>,
        after: Vec<EncodedMessagePart>,
    ) -> Self {
        Self {
            before,
            current,
            after,
        }
    }

    pub fn before(&self) -> &[EncodedMessagePart] {
        &self.before
    }

    pub fn current(&self) -> Option<&EncodedMessagePart> {
        self.current.as_ref()
    }

    pub fn after(&self) -> &[EncodedMessagePart] {
        &self.after
    }
}

#[derive(Clone)]
pub struct Encoding {
    characters: HashMap<String, ShutterPositions>,
    message_end: ShutterPositions,
}

impl Default for Encoding {
    fn default() -> Self {
        let mut characters: HashMap<String, ShutterPositions> = HashMap::new();
        characters.insert(
            "A".into(),
            ShutterPositions::new(&[ShutterLocation::MiddleLeft, ShutterLocation::BottomRight])
                .unwrap(),
        );
        characters.insert(
            "B".into(),
            ShutterPositions::new(&[ShutterLocation::MiddleRight, ShutterLocation::BottomLeft])
                .unwrap(),
        );
        characters.insert(
            "C".into(),
            ShutterPositions::new(&[ShutterLocation::MiddleLeft, ShutterLocation::MiddleRight])
                .unwrap(),
        );
        characters.insert(
            "D".into(),
            ShutterPositions::new(&[ShutterLocation::TopLeft, ShutterLocation::BottomLeft])
                .unwrap(),
        );
        characters.insert(
            "E".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "F".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "G".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "H".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "I".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "J".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopRight,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "K".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "L".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "M".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::MiddleLeft,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "N".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "O".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::MiddleRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "O".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "Q".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "R".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "S".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "T".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "U".into(),
            ShutterPositions::new(&[
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "V".into(),
            ShutterPositions::new(&[
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "W".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::MiddleLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "X".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomLeft,
            ])
            .unwrap(),
        );
        characters.insert(
            "Y".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            "Z".into(),
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::MiddleLeft,
                ShutterLocation::MiddleRight,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        );
        characters.insert(
            " ".into(),
            ShutterPositions::new(&[ShutterLocation::BottomLeft]).unwrap(),
        );

        Self::new(
            characters,
            ShutterPositions::new(&[
                ShutterLocation::TopLeft,
                ShutterLocation::TopRight,
                ShutterLocation::BottomLeft,
                ShutterLocation::BottomRight,
            ])
            .unwrap(),
        )
        .unwrap()
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

        let characters: HashMap<String, ShutterPositions> = characters
            .iter()
            .map(|(k, v)| (k.to_uppercase(), v.clone()))
            .collect();

        for (character, position) in &characters {
            if position.all_closed() {
                return Err(anyhow!("character encoding can't be all shutters closed").into());
            }

            if positions.contains(position) {
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
            let uppercase_string = String::from(c).to_uppercase();
            match self.characters.get(&uppercase_string) {
                Some(positions) => {
                    parts.push(EncodedMessagePart::new(
                        MessageComponent::Character(uppercase_string),
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

    pub fn supported_characters(&self) -> Vec<String> {
        self.characters.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct Queue {
    messages: Arc<Mutex<Vec<EncodedMessage>>>,
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

    pub fn add_message(&self, message: EncodedMessage) -> Result<()> {
        let mut messages = self.messages.lock().unwrap();
        if messages.len() >= self.max_messages {
            return Err(Error::QueueIsFull);
        }
        messages.push(message);
        Ok(())
    }

    pub fn pop_message(&self) -> Option<EncodedMessage> {
        let mut messages = self.messages.lock().unwrap();
        messages.pop()
    }

    pub fn get_messages(&self) -> Result<Vec<EncodedMessage>> {
        let messages = self.messages.lock().unwrap();
        Ok(messages.clone())
    }
}

#[derive(Clone)]
pub struct MessagesToInject {
    messages: Arc<Vec<EncodedMessage>>,
}

impl MessagesToInject {
    pub fn new(messages: Vec<EncodedMessage>) -> Self {
        Self {
            messages: Arc::new(messages),
        }
    }

    pub fn get(&self) -> Option<&EncodedMessage> {
        self.messages.choose(&mut rand::rng())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingConfig {
    show_character_for: Duration,
    pause_between_characters_for: Duration,
    pause_between_messages_for: Duration,
    inject_message_if_no_next_message_after_pausing_between_messages_for: Duration,
}

impl TimingConfig {
    pub fn new(
        show_character_for: Duration,
        pause_between_characters_for: Duration,
        pause_between_messages_for: Duration,
        inject_message_if_no_next_message_after_pausing_between_messages_for: Duration,
    ) -> Self {
        Self {
            show_character_for,
            pause_between_characters_for,
            pause_between_messages_for,
            inject_message_if_no_next_message_after_pausing_between_messages_for,
        }
    }

    pub fn show_character_for(&self) -> &Duration {
        &self.show_character_for
    }

    pub fn pause_between_characters_for(&self) -> &Duration {
        &self.pause_between_characters_for
    }

    pub fn pause_between_messages_for(&self) -> &Duration {
        &self.pause_between_messages_for
    }

    pub fn inject_message_if_no_next_message_after_pausing_between_messages_for(
        &self,
    ) -> &Duration {
        &self.inject_message_if_no_next_message_after_pausing_between_messages_for
    }
}

#[derive(Clone)]
pub struct Clacks {
    current_state: Arc<Mutex<Box<dyn ClacksState>>>,
    config: TimingConfig,
    queue: Queue,
    messages_to_inject: MessagesToInject,
}

impl Clacks {
    pub fn new(config: TimingConfig, queue: Queue, messages_to_inject: MessagesToInject) -> Self {
        Self {
            current_state: Arc::new(Mutex::new(Box::new(ClacksWaitingForNextMessage::new()))),
            config,
            queue,
            messages_to_inject,
        }
    }

    pub fn update(&self) -> Result<ClacksUpdateResult> {
        let mut current_state = self.current_state.lock().unwrap();
        if let Some(new_state) =
            current_state.update(&self.queue, &self.config, &self.messages_to_inject)?
        {
            *current_state = new_state;
            return Ok(ClacksUpdateResult::StateChanged);
        };
        Ok(ClacksUpdateResult::StateNotChanged)
    }

    pub fn current_message(&self) -> Option<CurrentMessage> {
        let current_state = self.current_state.lock().unwrap();
        current_state.current_message()
    }
}

trait ClacksState: Send {
    fn update(
        &self,
        queue: &Queue,
        config: &TimingConfig,
        messages_to_inject: &MessagesToInject,
    ) -> Result<Option<Box<dyn ClacksState>>>;
    fn current_message(&self) -> Option<CurrentMessage>;
}

struct ClacksWaitingForNextMessage {
    started_at: time::DateTime,
}

impl ClacksWaitingForNextMessage {
    pub fn new() -> Self {
        Self {
            started_at: time::DateTime::now(),
        }
    }
}

impl ClacksState for ClacksWaitingForNextMessage {
    fn update(
        &self,
        queue: &Queue,
        _config: &TimingConfig,
        messages_to_inject: &MessagesToInject,
    ) -> Result<Option<Box<dyn ClacksState>>> {
        match queue.pop_message() {
            Some(encoded_message) => {
                return Ok(Some(Box::new(ClacksShowingCharacter::new_message(
                    encoded_message,
                ))));
            }
            None => {}
        }

        match messages_to_inject.get() {
            Some(encoded_message) => {
                return Ok(Some(Box::new(ClacksShowingCharacter::new_message(
                    encoded_message.clone(),
                ))));
            }
            None => {}
        }

        Ok(None)
    }

    fn current_message(&self) -> Option<CurrentMessage> {
        None
    }
}

struct ClacksShowingCharacter {
    before: Vec<EncodedMessagePart>,
    current: EncodedMessagePart,
    after: Vec<EncodedMessagePart>,
    started_at: time::DateTime,
}

impl ClacksShowingCharacter {
    pub fn new_message(message: EncodedMessage) -> Self {
        let first = message.parts[0].clone();
        Self {
            before: vec![],
            current: first,
            after: message.parts.into_iter().skip(1).collect(),
            started_at: time::DateTime::now(),
        }
    }

    fn next_character(state: &ClacksPausingBetweenCharacters) -> Result<Self> {
        if state.after.is_empty() {
            return Err(
                anyhow!("after can't be empty when advancing to the next character").into(),
            );
        }

        let next = state.after[0].clone();
        Ok(Self {
            before: state.before.clone(),
            current: next,
            after: state.after.clone().into_iter().skip(1).collect(),
            started_at: time::DateTime::now(),
        })
    }
}

impl ClacksState for ClacksShowingCharacter {
    fn update(
        &self,
        _queue: &Queue,
        config: &TimingConfig,
        _messages_to_inject: &MessagesToInject,
    ) -> Result<Option<Box<dyn ClacksState>>> {
        let since = &time::DateTime::now() - &self.started_at;
        if since < config.show_character_for {
            return Ok(None);
        }

        if !self.after.is_empty() {
            return Ok(Some(Box::new(ClacksPausingBetweenCharacters::new(self)?)));
        }

        Ok(Some(Box::new(ClacksPausingBetweenMessages::new())))
    }

    fn current_message(&self) -> Option<CurrentMessage> {
        Some(CurrentMessage::new(
            self.before.clone(),
            Some(self.current.clone()),
            self.after.clone(),
        ))
    }
}

struct ClacksPausingBetweenCharacters {
    before: Vec<EncodedMessagePart>,
    after: Vec<EncodedMessagePart>,
    started_at: time::DateTime,
}

impl ClacksPausingBetweenCharacters {
    pub fn new(state: &ClacksShowingCharacter) -> Result<Self> {
        if state.after.is_empty() {
            return Err(
                anyhow!("after can't be empty when entering pausing before characters").into(),
            );
        }

        let mut before: Vec<EncodedMessagePart> = state.before.clone();
        before.push(state.current.clone());

        Ok(Self {
            before,
            after: state.after.clone(),
            started_at: time::DateTime::now(),
        })
    }
}

impl ClacksState for ClacksPausingBetweenCharacters {
    fn update(
        &self,
        _queue: &Queue,
        config: &TimingConfig,
        _messages_to_inject: &MessagesToInject,
    ) -> Result<Option<Box<dyn ClacksState>>> {
        let since = &time::DateTime::now() - &self.started_at;
        if since < config.pause_between_characters_for {
            return Ok(None);
        }

        Ok(Some(Box::new(ClacksShowingCharacter::next_character(
            self,
        )?)))
    }

    fn current_message(&self) -> Option<CurrentMessage> {
        Some(CurrentMessage::new(
            self.before.clone(),
            None,
            self.after.clone(),
        ))
    }
}

struct ClacksPausingBetweenMessages {
    started_at: time::DateTime,
}

impl ClacksPausingBetweenMessages {
    pub fn new() -> Self {
        Self {
            started_at: time::DateTime::now(),
        }
    }
}

impl ClacksState for ClacksPausingBetweenMessages {
    fn update(
        &self,
        _queue: &Queue,
        config: &TimingConfig,
        _messages_to_inject: &MessagesToInject,
    ) -> Result<Option<Box<dyn ClacksState>>> {
        let since = &time::DateTime::now() - &self.started_at;
        if since < config.pause_between_messages_for {
            return Ok(None);
        }

        Ok(Some(Box::new(ClacksWaitingForNextMessage::new())))
    }

    fn current_message(&self) -> Option<CurrentMessage> {
        None
    }
}
