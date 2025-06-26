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
    use super::commands::{self, SpiDescriptor};
    use static_fifo_queue::Queue;
    #[test]
    fn test_queue() {
        let mut queue: Queue<SpiDescriptor, 8> = Queue::new();
        static mut SET_SLEEP: commands::SetSleep =
            commands::SetSleep::new(commands::SleepConfig::new().with_warm_start(true));
        static mut SET_STANDY: commands::SetStandby =
            commands::SetStandby::new(commands::StdbyConfig::StdbyRc);
        static mut WRITE_BUFFER: commands::WriteBuffer<7> =
            commands::WriteBuffer::new(0, [2, 4, 7, 9, 3]);
        unsafe {
            queue.enqueue(SpiDescriptor::new(&mut SET_SLEEP));
            queue.enqueue(SpiDescriptor::new(&mut SET_STANDY));
            queue.enqueue(SpiDescriptor::new(&mut WRITE_BUFFER));

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
