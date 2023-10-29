use crate::mutex_box::MutexBox;
use nmea::{Nmea, SentenceType};
use serde::{Deserialize,Serialize};

#[derive(Debug, Default,Deserialize,Serialize)]
pub struct Gnss{
    data:MutexBox<Nmea>,
}

impl Gnss {
    pub fn new()->Gnss{
        let data=MutexBox::new();
        data.init(Nmea::default());
        Gnss { data: data }
    }
    
    /// Parse any NMEA sentence and stores the result of sentences that include:
    /// - altitude
    /// - latitude and longitude
    /// - speed_over_ground
    /// - and other
    ///
    /// The type of sentence is returned if implemented and valid.
    pub fn parse(&mut self, sentence: &String) -> Option<SentenceType> {
        let result=
        self.data.open_locked(move|data|{
           if let Ok(sentenance) = data.parse(sentence.as_str()) {
               Some(sentenance)
           } else{
            None
           }
        },None);
        result
    }
}