//! <div class="warning">
//! <strong>Requires Rust Nightly</strong>
//! </div>
//!
#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(static_mut_refs)]

pub mod commands;
pub mod registers;

#[cfg(test)]
mod tests {
    use super::commands::{
        self, SetSleep, SetStandby, SleepConfig, SpiCommand, StdbyConfig, WriteBuffer,
    };
    use static_fifo_queue::Queue;

    #[test]
    fn test_queue() {
        let mut queue: Queue<commands::SpiDescriptor, 8> = Queue::new();
        static mut SET_SLEEP: SetSleep =
            commands::SetSleep::new(SleepConfig::new().with_warm_start(true));
        static mut SET_STANDY: SetStandby = commands::SetStandby::new(StdbyConfig::StdbyRc);
        static mut WRITE_BUFFER: WriteBuffer<7> = commands::WriteBuffer::new(0, [2, 4, 7, 9, 3]);
        unsafe {
            queue.enqueue(SET_SLEEP.descriptor());
            queue.enqueue(SET_STANDY.descriptor());
            queue.enqueue(WRITE_BUFFER.descriptor());

            let mut desc = queue.dequeue().unwrap();
            let mut tx_buf =
                core::slice::from_raw_parts(desc.tx_buf_ptr, desc.transfer_length as usize);
            assert_eq!(tx_buf, [0x84, 1 << 2]);

            desc = queue.dequeue().unwrap();
            tx_buf = core::slice::from_raw_parts(desc.tx_buf_ptr, desc.transfer_length as usize);
            assert_eq!(tx_buf, [0x80, 0]);

            desc = queue.dequeue().unwrap();
            tx_buf = core::slice::from_raw_parts(desc.tx_buf_ptr, desc.transfer_length as usize);
            assert_eq!(tx_buf, [0x0E, 0, 2, 4, 7, 9, 3]);
        }
    }
}
