use lapin::Channel;

pub struct Consumer {
    channel: Channel,
}

impl Consumer {
    pub fn new(channel: Channel) -> Self {
        Consumer { channel }
    }

}